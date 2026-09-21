use godot::classes::{
    ISprite2D,   // Interface used when creating a custom Sprite2D class.
    InputEvent,  // Represents an input event, such as a mouse click.
    Node2D,      // Base class for 2D nodes with position and rotation.
    PackedScene, // A saved Godot scene that can be loaded and instantiated.
    Sprite2D,    // Godot's 2D sprite node.
};
use godot::prelude::*;

// Import the systems that CannonBarrel needs to communicate with.
use crate::bullet_brain::BulletBrain;
use crate::bullet_stopper::BulletStopper;
use crate::player::Player;

// CannonBarrel represents the player's cannon.
//
// It is responsible for:
// - Pointing the cannon toward the mouse.
// - Detecting the player's click.
// - Asking BulletBrain to create a bullet.
// - Creating a BulletStopper at the clicked position.
#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct CannonBarrel {
    // The underlying Godot Sprite2D.
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for CannonBarrel {
    // Called when Godot creates the CannonBarrel.
    fn init(base: Base<Sprite2D>) -> Self {
        Self { base }
    }

    // Called when CannonBarrel enters the scene tree.
    fn ready(&mut self) {
        godot_print!("CannonBarrel ready");
    }

    // Called when an input event is received by this node.
    //
    // InputEvent can represent things such as:
    // - mouse clicks
    // - keyboard presses
    // - controller input
    fn input(&mut self, input_event: Gd<InputEvent>) {
        // Ignore every input event except our "click" action.
        //
        // "click" must be configured in Godot's Input Map.
        if !input_event.is_action_pressed("click") {
            return;
        }

        godot_print!("CLICK ACTION DETECTED");

        // Find the Player directly from the main game scene.
        //
        // The path means:
        //
        // /root
        //   └── game
        //       └── player
        let player = self.base().get_node_as::<Player>("/root/game/player");

        godot_print!("Player found");

        // Check whether the player is allowed to shoot.
        //
        // can_shoot prevents the player from firing another bullet
        // while the current bullet is still active.
        if !player.bind().can_shoot {
            godot_print!("Player cannot shoot");
            return;
        }

        // The player can shoot, so aim and fire at the mouse.
        self.shoot_at_mouse(player);
    }

    // Called every frame.
    fn process(&mut self, _delta: f64) {
        // Get the mouse position in global/world coordinates.
        let mouse_position = self.base().get_global_mouse_position();

        // Rotate the cannon so that it points toward the mouse.
        self.base_mut().look_at(mouse_position);
    }
}

#[godot_api]
impl CannonBarrel {
    // Creates a player bullet aimed at the mouse position.
    //
    // The Player is passed in so this function can change
    // the player's can_shoot state.
    #[func]
    pub fn shoot_at_mouse(&mut self, mut player: Gd<Player>) {
        godot_print!("SHOOTING PLAYER BULLET");

        // Get the position of the cannon.
        //
        // This is where the player bullet will start.
        let cannon_position = self.base().get_global_position();

        // Get the current mouse position.
        //
        // This is where the player bullet will be aimed.
        let mouse_position = self.base().get_global_mouse_position();

        // Print the cannon position for debugging.
        godot_print!(
            "Cannon position: {}, {}",
            cannon_position.x,
            cannon_position.y
        );

        // Print the mouse position for debugging.
        godot_print!(
            "Mouse position: {}, {}",
            mouse_position.x,
            mouse_position.y
        );

        // Find BulletBrain directly in the scene tree.
        //
        // BulletBrain is responsible for creating the actual
        // bullet instance.
        let mut bullet_brain = self
            .base()
            .get_node_as::<BulletBrain>("/root/game/bullets/bullet_brain");

        godot_print!("BulletBrain found");

        // Ask BulletBrain to create a bullet.
        //
        // call() calls a Godot-exposed function by its name.
        //
        // The arguments are:
        //
        // 1. cannon_position → where the bullet starts
        // 2. mouse_position  → where the bullet is aimed
        // 3. "player"        → tells BulletBrain this is a player bullet
        bullet_brain.call(
            "spawn_bullet",
            &[
                cannon_position.to_variant(),
                mouse_position.to_variant(),
                "player".to_variant(),
            ],
        );

        godot_print!("PLAYER BULLET SPAWN REQUEST SENT");

        // Prevent the player from firing another bullet.
        //
        // can_shoot will become true again when the bullet
        // reaches the BulletStopper.
        player.bind_mut().can_shoot = false;

        godot_print!("Player can_shoot = false");

        // Load the BulletStopper scene from the Godot project.
        //
        // PackedScene is like a reusable template for creating
        // a new BulletStopper node.
        let stopper_scene =
            load::<PackedScene>("res://scenes/bullet_stopper.tscn");

        // Create an actual BulletStopper from the scene template.
        let mut stopper = stopper_scene.instantiate_as::<BulletStopper>();

        // Find the Node2D that contains active bullets and
        // related objects.
        let mut bullets =
            self.base().get_node_as::<Node2D>("/root/game/bullets");

        // Add the new BulletStopper to the scene tree.
        bullets.add_child(&stopper);

        // Put the BulletStopper at the mouse position.
        //
        // When the player's bullet reaches this position,
        // BulletStopper detects it and allows the player
        // to shoot again.
        stopper.set_global_position(mouse_position);

        godot_print!("BulletStopper created");
    }
}