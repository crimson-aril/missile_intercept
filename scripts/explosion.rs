use godot::classes::{
AnimatedSprite2D, // Used to check bullet animations and play the explosion animation.
Area2D,           // Used for detecting when a bullet enters the explosion.
IArea2D,          // Interface for a custom Rust class based on Area2D.
};
use godot::prelude::*;

// Import our custom Bullet class so we can verify that
// the detected Area2D is actually a Rust Bullet.
use crate::bullet::Bullet;

// Import the Player class so we can update the player's score.
use crate::player::Player;

// Explosion is an Area2D that detects enemy bullets.
//
// When an enemy bullet enters the explosion:
// 1. Check that it is an enemy bullet.
// 2. Verify that it is our Rust Bullet class.
// 3. Create an enemy explosion effect.
// 4. Remove the enemy bullet.
// 5. Add one point to the player's score.
//
// When the explosion animation finishes, the Explosion node
// removes itself from the scene.
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Explosion {

 
// Stores a reference to the Player.
//
// Option is used because the Player reference is found
// later when ready() runs.
player: Option<Gd<Player>>,

// The underlying Godot Area2D.
base: Base<Area2D>,
 

}

#[godot_api]
impl IArea2D for Explosion {

 
// Called when Godot creates the Explosion.
fn init(base: Base<Area2D>) -> Self {
    Self {
        // The Player has not been found yet.
        player: None,

        // Store the underlying Area2D.
        base,
    }
}


// Called when the Explosion enters the scene tree.
fn ready(&mut self) {
    godot_print!("Explosion ready");

    // Find the Player directly from the main game scene.
    //
    // The path is:
    //
    // /root/game/player
    let player = self
        .base()
        .get_node_as::<Player>("/root/game/player");

    // Store the Player reference for later use.
    self.player = Some(player);

    godot_print!("Explosion found Player");
}
 

}

#[godot_api]
impl Explosion {

 
// Called when another Area2D enters the Explosion.
//
// This function should be connected to the Area2D's
// "area_entered" signal in Godot.
#[func]
pub fn on_explosion_area_entered(
    &mut self,
    mut area: Gd<Area2D>,
) {
    godot_print!("EXPLOSION DETECTED AREA");


    // Try to find an AnimatedSprite2D inside the detected Area2D.
    //
    // We use try_get_node_as() because not every Area2D
    // entering the explosion will necessarily contain
    // an AnimatedSprite2D.
    let Some(bullet_sprite) =
        area.try_get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
    else {
        godot_print!(
            "Detected Area2D has no AnimatedSprite2D"
        );

        // Stop here if the object does not have the
        // AnimatedSprite2D we expect.
        return;
    };


    // Get the current animation name from the bullet.
    //
    // In this project:
    //
    // "enemy"  = enemy rocket/bullet
    // "player" = player's bullet
    let animation = bullet_sprite.get_animation().to_string();

    godot_print!(
        "Detected bullet animation: {}",
        animation
    );


    // The explosion should only destroy enemy bullets.
    //
    // If this is a player bullet, ignore it.
    if animation != "enemy" {
        godot_print!("Not an enemy bullet");
        return;
    }


    // Make sure the detected Area2D is actually our
    // custom Rust Bullet class.
    //
    // try_cast() attempts to convert the generic Area2D
    // into a Bullet.
    //
    // If the conversion fails, the object is not our Bullet.
    if area.clone().try_cast::<Bullet>().is_err() {
        godot_print!(
            "Detected object is not a Rust Bullet"
        );
        return;
    }

    godot_print!("ENEMY ROCKET INTERCEPTED!");


    // Save the enemy bullet's current position.
    //
    // The explosion will be created at this position.
    let bullet_position = area.get_global_position();


    // Find BulletBrain directly in the scene tree.
    //
    // BulletBrain is responsible for creating the
    // explosion scene.
    let mut bullet_brain = self
        .base()
        .get_node_as::<Node>(
            "/root/game/bullets/bullet_brain",
        );


    // Ask BulletBrain to create an enemy explosion.
    //
    // call_deferred() delays the function call until
    // Godot has finished processing the current collision event.
    //
    // Arguments:
    //
    // bullet_position → where the explosion should appear
    // "enemy"         → tells BulletBrain which animation to use
    bullet_brain.call_deferred(
        "spawn_explosion",
        &[
            bullet_position.to_variant(),
            "enemy".to_variant(),
        ],
    );

    godot_print!("Enemy explosion requested");


    // Remove the enemy rocket from the scene.
    //
    // queue_free() safely schedules the object for deletion.
    area.queue_free();

    godot_print!("Enemy rocket freed");


    // Add one point to the player's score.
    //
    // First check that we still have a valid Player reference.
    if let Some(player) = &mut self.player {
        godot_print!("Adding 1 point");

        // Call the Player's add_score() function.
        player.bind_mut().add_score(1);

        // Print the new score for debugging.
        godot_print!(
            "Score after interception: {}",
            player.bind().score
        );
    } else {
        // This should normally not happen because the Player
        // is assigned during ready().
        godot_print!("ERROR: Player reference is missing");
    }
}


// Called when the explosion's AnimatedSprite2D finishes
// playing its animation.
//
// This function should be connected to the
// AnimatedSprite2D's "animation_finished" signal in Godot.
#[func]
pub fn on_animated_sprite_2d_animation_finished(&mut self) {
    godot_print!("Explosion animation finished");

    // The visual explosion is finished, so remove the
    // Explosion node from the scene.
    self.base_mut().queue_free();
}
 

}
