use godot::classes::{
AnimatedSprite2D,    // Godot node used to display animated 2D sprites.
IAnimatedSprite2D,   // Interface required for a custom AnimatedSprite2D class.
};
use godot::global::randf_range; // Generates random floating-point numbers.
use godot::prelude::*;          // Common Godot-Rust types and functionality.

// Cloud is a custom Rust class that inherits from Godot's
// AnimatedSprite2D.
//
// This allows the cloud to display an animation while also
// having custom Rust movement logic.
#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct Cloud {
// Controls how fast the cloud moves.
pub speed: f32,

 
// Stores how far the cloud should move during the
// current frame.
velocity: Vector2,

// The underlying Godot AnimatedSprite2D.
base: Base<AnimatedSprite2D>,
 

}

#[godot_api]
impl IAnimatedSprite2D for Cloud {

 
// Called when Godot creates a new Cloud.
fn init(base: Base<AnimatedSprite2D>) -> Self {
    Self {
        // Default speed.
        //
        // This value is replaced with a random speed
        // when ready() runs.
        speed: 400.0,

        // Start with no movement.
        velocity: Vector2::ZERO,

        // Store the Godot AnimatedSprite2D.
        base,
    }
}


// Called when the Cloud enters the scene tree.
fn ready(&mut self) {

    // Give each cloud a random movement speed.
    //
    // This makes clouds move at slightly different speeds
    // instead of all moving together.
    self.speed = randf_range(100.0, 200.0) as f32;


    // Change the cloud's color and transparency.
    //
    // Color::from_rgba() uses values from 0.0 to 1.0:
    //
    // Red   = 0.50
    // Green = 0.00
    // Blue  = 0.25
    // Alpha = 0.7 (70% opacity)
    //
    // This gives the cloud a reddish/purple tint.
    self.base_mut().set_modulate(
        Color::from_rgba(0.50, 0.00, 0.25, 0.7)
    );
}


// Called every frame.
//
// delta is the amount of time that passed since
// the previous frame, measured in seconds.
fn process(&mut self, delta: f64) {

    // Get the cloud's current rotation.
    //
    // The rotation determines which direction
    // the cloud will move.
    let rotation = self.base().get_rotation();


    // Calculate how far the cloud should move this frame.
    //
    // Vector2::RIGHT represents the direction (1, 0).
    //
    // rotated(rotation) changes that direction based
    // on the cloud's rotation.
    //
    // Multiplying by speed controls how fast it moves.
    //
    // Multiplying by delta makes the movement independent
    // of the game's frame rate.
    self.velocity =
        Vector2::RIGHT.rotated(rotation)
        * self.speed
        * delta as f32;


    // Copy the calculated movement into a local variable.
    //
    // This avoids borrowing self mutably while we are
    // still using another value from self.
    let velocity = self.velocity;


    // Move the cloud by the calculated amount.
    self.base_mut().translate(velocity);
}
 

}
