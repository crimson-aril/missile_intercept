use godot::classes::{
AnimatedSprite2D, // Used to check which type of bullet was detected.
Area2D,           // Used for detecting enemy bullets entering the player hit zone.
InputEvent,       // Represents keyboard, mouse, or other input events.
Label,            // Used to display health and score.
Node,             // Basic Godot node type.
Node2D,           // Used for accessing 2D nodes such as the cannon and game-over screen.
INode,            // Interface required for a custom class based on Node.
};
use godot::prelude::*;

// Our custom Rust classes.
use crate::bullet::Bullet;
use crate::bullet_brain::BulletBrain;

// Player controls the player's game state.
//
// It keeps track of:
// - Health
// - Score
// - Whether the player can shoot
// - Whether the game is over
//
// It also handles:
// - Enemy bullets hitting the player
// - Updating the HUD
// - Increasing difficulty
// - Restarting the game
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Player {

 
// Stores a reference to BulletBrain.
//
// Option is used because the reference is found later
// when ready() runs.
bullet_brain: Option<Gd<BulletBrain>>,

// Determines whether the player is currently allowed to shoot.
//
// This becomes false while a player bullet is travelling
// toward its BulletStopper.
pub can_shoot: bool,

// Becomes true when the player's health reaches zero.
pub game_over: bool,

// Player's current health.
//
// #[export] makes this value editable from the Godot Inspector.
#[export]
pub health: i32,

// Player's current score.
pub score: i32,

// The underlying Godot Node.
base: Base<Node>,
 

}

#[godot_api]
impl INode for Player {

 
// Called when Godot creates the Player.
fn init(base: Base<Node>) -> Self {
    Self {
        // BulletBrain will be found later in ready().
        bullet_brain: None,

        // The player can shoot when the game starts.
        can_shoot: true,

        // The game has not ended yet.
        game_over: false,

        // Start with 3 health points.
        health: 3,

        // Start with zero points.
        score: 0,

        // Store the underlying Godot Node.
        base,
    }
}


// Called when the Player enters the scene tree.
fn ready(&mut self) {

    // Find BulletBrain directly from the main game scene.
    //
    // The path is:
    //
    // /root
    //   └── game
    //       └── bullets
    //           └── bulletBrain
    let bullet_brain =
        self.base()
            .get_node_as::<BulletBrain>(
                "/root/game/bullets/bullet_brain",
            );

    // Store the Bullet_brain reference so it can be used later.
    self.bullet_brain = Some(bullet_brain);


    // Display the starting health and score.
    self.update_ui();
}


// Called when the Player receives an input event.
fn input(&mut self, input_event: Gd<InputEvent>) {

    // If the player clicks while the game is over,
    // restart the current scene.
    if input_event.is_action_pressed("click")
        && self.game_over
    {
        self.base()
            .get_tree()
            .reload_current_scene();
    }
}


// Called every frame.
//
// The Player currently does not need to perform
// any continuous per-frame work.
fn process(&mut self, _delta: f64) {
}
 

}

#[godot_api]
impl Player {

 
// Called when an Area2D enters the player's hit zone.
//
// This function should be connected to the hit zone's
// "area_entered" signal in Godot.
#[func]
pub fn on_player_hit_zone_area_entered(
    &mut self,
    mut bullet: Gd<Area2D>,
) {

    // Try to find the AnimatedSprite2D inside the
    // detected Area2D.
    //
    // We use this to identify whether the object is
    // using the enemy bullet animation.
    let Some(bullet_sprite) =
        bullet.try_get_node_as::<AnimatedSprite2D>(
            "AnimatedSprite2D",
        )
    else {
        // If there is no AnimatedSprite2D, this is not
        // an object we are interested in.
        return;
    };


    // Only enemy bullets should damage the player.
    //
    // Player bullets are ignored.
    if bullet_sprite.get_animation().to_string()
        != "enemy"
    {
        return;
    }


    // Make sure the detected Area2D is actually our
    // custom Rust Bullet class.
    //
    // try_cast consumes the Gd, so we clone it first
    // to keep the original bullet handle available.
    if bullet.clone().try_cast::<Bullet>().is_err() {
        return;
    }


    // Remember the enemy bullet's position.
    //
    // The explosion will appear at this position.
    let bullet_position =
        bullet.get_global_position();


    // Check whether we have a valid BulletBrain reference.
    if let Some(bullet_brain) =
        &mut self.bullet_brain
    {
        // Ask BulletBrain to create an enemy explosion.
        //
        // call_deferred() waits until the current collision
        // processing has finished before calling the function.
        bullet_brain.call_deferred(
            "spawn_explosion",
            &[
                bullet_position.to_variant(),
                "enemy".to_variant(),
            ],
        );
    }


    // Remove the enemy bullet from the game.
    bullet.queue_free();


    // Reduce the player's health by 1.
    self.hit_player(1);
}


// Reduces the player's health.
#[func]
pub fn hit_player(
    &mut self,
    damage_amount: i32,
) {

    // Subtract the damage from health.
    //
    // max(0) prevents health from becoming negative.
    self.health =
        (self.health - damage_amount).max(0);


    // Update the HUD so the new health value is displayed.
    self.update_ui();


    // Check whether the player's health has reached zero.
    //
    // The second condition prevents the game-over logic
    // from running more than once.
    if self.health <= 0
        && !self.game_over
    {
        // Mark the game as over.
        self.game_over = true;

        // Prevent the player from shooting after dying.
        self.can_shoot = false;


        // Find the game-over screen in the HUD.
        let mut game_over_screen =
            self.base()
                .get_node_as::<Node2D>(
                    "/root/game/hud/gameOverScreen",
                );


        // Make the game-over screen visible.
        game_over_screen.set_visible(true);


        // Find the player's cannon.
        let mut cannon =
            self.base()
                .get_node_as::<Node2D>(
                    "/root/game/foreground/cannon",
                );


        // Create an explosion where the cannon was located.
        if let Some(bullet_brain) =
            &mut self.bullet_brain
        {
            bullet_brain.call_deferred(
                "spawn_explosion",
                &[
                    cannon
                        .get_global_position()
                        .to_variant(),
                    "enemy".to_variant(),
                ],
            );
        }


        // Remove the cannon from the game.
        cannon.queue_free();
    }
}


// Adds points to the player's score.
#[func]
pub fn add_score(
    &mut self,
    score_amount: i32,
) {

    // Add the requested amount to the current score.
    self.score += score_amount;


    // Update the HUD with the new score.
    self.update_ui();


    // Increase the game's difficulty after scoring.
    //
    // BulletBrain reduces the amount of time between
    // enemy bullet spawns.
    if let Some(bullet_brain) =
        &mut self.bullet_brain
    {
        bullet_brain.call(
            "increase_difficulty",
            &[],
        );
    }
}


// Updates the health and score text shown on the HUD.
#[func]
pub fn update_ui(&mut self) {

    // Find the Label used to display health and score.
    let mut health_and_score =
        self.base()
            .get_node_as::<Label>(
                "/root/game/hud/healthAndScore",
            );


    // Build the text that will appear on the screen.
    //
    // Example:
    //
    // HEALTH: 3     SCORE: 5
    let new_hud_text = format!(
        "HEALTH: {}     SCORE: {}",
        self.health,
        self.score
    );


    // Replace the Label's current text.
    health_and_score.set_text(
        &new_hud_text,
    );
}
 

}
