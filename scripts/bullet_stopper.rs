use godot::classes::{
    // Displays and controls 2D sprite animations.
    AnimatedSprite2D,

    // A 2D area used for detecting overlaps with other
    // Area2D nodes, such as the player's bullet.
    Area2D,

    // Interface required when creating a custom Rust class
    // that inherits from Godot's Area2D.
    IArea2D,
};

// Provides commonly used Godot-Rust types and features,
// such as GodotClass, Base, Gd, and #[godot_api].
use godot::prelude::*;

// Import the custom Bullet class from bullet.rs.
// Used to verify that the detected Area2D is actually
// one of our Rust Bullet objects.
use crate::bullet::Bullet;

// Import BulletBrain so BulletStopper can ask it to
// create an explosion when a bullet reaches the stopper.
use crate::bullet_brain::BulletBrain;

// Import the custom Player class so we can access
// player-related properties such as can_shoot.
use crate::player::Player;

// BulletStopper detects when the player's bullet reaches
// the bullet stopper area.
//
// It then:
// 1. Creates an explosion.
// 2. Removes the player's bullet.
// 3. Removes itself.
// 4. Allows the player to shoot again.
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct BulletStopper {
    // Stores a reference to the Player.
    //
    // Option is used because the player is found later in ready().
    player: Option<Gd<Player>>,

    // The Godot Area2D that this Rust class inherits from.
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for BulletStopper {
    // Called when Godot creates the BulletStopper.
    fn init(base: Base<Area2D>) -> Self {
        Self {
            // The player has not been found yet.
            player: None,

            // Store the underlying Godot Area2D.
            base,
        }
    }

    // ready() runs after the node has entered the scene tree.
    //
    // At this point, we can safely look for other nodes
    // that already exist in the game scene.
    fn ready(&mut self) {
        godot_print!("BulletStopper ready");

        // Find the Player node in the main game scene.
        //
        // The path starts at /root and follows:
        //
        // /root/game/player
        //
        // get_node_as::<Player>() also tells Godot-Rust
        // that we expect this node to be our custom Player class.
        let player = self.base().get_node_as::<Player>("/root/game/player");

        // Store the Player reference so we can use it later.
        self.player = Some(player);

        godot_print!("BulletStopper found Player");
    }
}

#[godot_api]
impl BulletStopper {
    // This function is intended to be connected to the
    // Area2D's "area_entered" signal in Godot.
    //
    // It is called when another Area2D enters the stopper.
    #[func]
    pub fn on_bullet_stopper_area_entered(&mut self, mut area: Gd<Area2D>) {
        godot_print!("BULLET STOPPER DETECTED AREA");

        // Print the name of the detected Area2D.
        // This is useful when debugging collisions.
        godot_print!("Detected object: {}", area.get_name());

        // Try to find an AnimatedSprite2D inside the detected object.
        //
        // Some Area2D objects may not have this child node.
        // try_get_node_as() returns None instead of causing an error
        // when the node cannot be found.
        let Some(bullet_sprite) = area.try_get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
        else {
            godot_print!("Detected object does NOT have AnimatedSprite2D");

            // Stop here because we cannot identify the object
            // using its animation.
            return;
        };

        // Get the name of the animation currently assigned
        // to the AnimatedSprite2D.
        //
        // In this game:
        // "player" = player's bullet
        // "enemy"  = enemy bullet
        let animation = bullet_sprite.get_animation().to_string();

        godot_print!("Detected animation: {}", animation);

        // Only the player's bullet should activate the stopper.
        //
        // If an enemy bullet enters the area, simply ignore it.
        if animation != "player" {
            godot_print!("This is NOT the player bullet");
            return;
        }

        // Make sure the detected object is actually our
        // custom Rust Bullet class.
        //
        // try_cast() attempts to convert the generic Area2D
        // into a Bullet.
        //
        // If the cast fails, this is not our Bullet object.
        if area.clone().try_cast::<Bullet>().is_err() {
            godot_print!("Detected object is not a Rust Bullet");
            return;
        }

        godot_print!("PLAYER BULLET REACHED STOPPER!");

        // Get the global position of the BulletStopper.
        //
        // This is where the explosion will be created.
        let position = self.base().get_global_position();

        // Find BulletBrain directly using its known path.
        //
        // BulletBrain is responsible for creating explosions,
        // so we ask it to create one at the stopper's position.
        let mut bullet_brain = self
            .base()
            .get_node_as::<BulletBrain>("/root/game/bullets/bullet_brain");

        godot_print!("BulletBrain found");

        // Ask BulletBrain to create the explosion.
        //
        // call_deferred() tells Godot to perform the function
        // call safely after the current signal/collision event
        // has finished processing.
        //
        // The first argument is the function name.
        // The remaining arguments are passed to that function.
        bullet_brain.call_deferred(
            "spawn_explosion",
            &[position.to_variant(), "player".to_variant()],
        );

        godot_print!("Player explosion requested");

        // Remove the player's bullet from the scene.
        //
        // queue_free() schedules the node for deletion safely.
        area.queue_free();

        godot_print!("Player bullet freed");

        // Remove the BulletStopper itself.
        //
        // Once the player's bullet has reached it, the stopper
        // has completed its job.
        self.base_mut().queue_free();

        godot_print!("BulletStopper freed");

        // Allow the player to fire another bullet.
        //
        // player is an Option, so we first check whether it
        // actually contains a Player reference.
        if let Some(player) = &mut self.player {
            player.bind_mut().can_shoot = true;

            godot_print!("Player can_shoot = true");
        }
    }
}
