use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

// Bullet is a custom Rust class that inherits from Godot's Area2D.
//
// Area2D is useful here because bullets need to detect overlaps
// with other objects in the game.
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Bullet {
    // Controls how fast the bullet moves.
    //
    // This is public because another Rust script, such as
    // BulletBrain, can change the speed after creating the bullet.
    pub speed: f32,

    // Stores the amount and direction the bullet should move
    // during the current frame.
    velocity: Vector2,

    // The underlying Godot Area2D that this Rust class controls.
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Bullet {
    // Called when Godot creates a new Bullet.
    fn init(base: Base<Area2D>) -> Self {
        Self {
            // Default bullet speed.
            speed: 400.0,

            // Vector2::ZERO means the bullet initially has
            // no movement.
            velocity: Vector2::ZERO,

            // Store the Godot Area2D.
            base,
        }
    }

    // Called once when the Bullet enters the scene tree.
    //
    // This function is currently empty because the bullet does
    // not need any special setup when it is created.
    fn ready(&mut self) {}

    // Called every frame by Godot.
    //
    // delta is the amount of time that has passed since the
    // previous frame, measured in seconds.
    //
    // Using delta makes movement independent of the frame rate.
    fn process(&mut self, delta: f64) {
        // Get the bullet's current rotation in radians.
        //
        // The bullet uses its rotation to determine which
        // direction it should travel.
        let rotation = self.base().get_rotation();

        // Vector2::RIGHT represents the direction:
        //
        //     (1, 0)
        //
        // Rotating this vector gives us the direction that
        // the bullet is facing.
        //
        // We then multiply the direction by:
        //     speed → how fast the bullet moves
        //     delta → how much time passed this frame
        //
        // This produces the distance the bullet should travel
        // during the current frame.
        self.velocity = Vector2::RIGHT.rotated(rotation) * self.speed * delta as f32;

        // Copy the calculated velocity into a local variable.
        //
        // This makes it easier to pass the movement value
        // to translate() below.
        let velocity = self.velocity;

        // Move the bullet by the calculated amount.
        //
        // translate() moves the Area2D relative to its
        // current position.
        self.base_mut().translate(velocity);
    }
}
