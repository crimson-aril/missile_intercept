// Godot classes used by this script.
//
// These are specific Godot node types and interfaces that we need
// to work with things such as sprites, bullets, timers, and nodes.
use godot::classes::{
    AnimatedSprite2D, // Plays and controls 2D sprite animations.
    Area2D,           // Detects overlaps/collisions with other Area2D nodes.
    INode,            // Allows us to implement Godot's Node lifecycle functions.
    Node,             // Basic Godot node type.
    Node2D,           // Base type for 2D nodes with position, rotation, etc.
    PackedScene,      // A saved Godot scene that can be loaded and instantiated.
    Timer,            // Counts down and emits a timeout signal.
};

// Provides Godot's random floating-point number function.
//
// randf_range( min, max ) returns a random decimal number
// between the specified minimum and maximum values.
use godot::global::randf_range;

// Imports the main Godot-Rust types and commonly used functionality.
//
// This includes things such as GodotClass, Base, Gd, Vector2,
// #[godot_api], #[func], and other Godot-Rust features.
use godot::prelude::*;

// Import our custom Rust Bullet class.
//
// This allows this script to create, access, and work with
// Bullet objects defined in bullet.rs.
use crate::bullet::Bullet;

// BulletBrain controls spawning bullets, explosions, and clouds.
//
// It also controls the difficulty by gradually reducing the time
// between enemy bullet spawns.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct BulletBrain {
    // These values are exported, so they can be changed directly
    // from the Godot Inspector.
    //
    // Maximum time between enemy spawns at the beginning of the game.
    #[export]
    pub max_spawn_interval: f32,

    // Smallest amount of time allowed between enemy spawns.
    #[export]
    pub min_spawn_interval: f32,

    // How much the spawn interval decreases when difficulty increases.
    #[export]
    pub spawn_interval_decrease: f32,

    // The current time between enemy spawns.
    //
    // Unlike the values above, this is not exported because it is
    // controlled automatically by the game.
    pub spawn_interval: f32,

    // Movement speed for bullets fired by the player.
    #[export]
    pub player_bullet_speed: i32,

    // Movement speed for bullets fired by enemies.
    #[export]
    pub enemy_bullet_speed: i32,

    // Stores the enemy spawning Timer.
    //
    // Option is used because the Timer is not available until
    // the Node is ready inside Godot.
    enemy_spawner: Option<Gd<Timer>>,

    // PackedScenes are reusable Godot scenes loaded from .tscn files.
    //
    // We keep these scenes here so BulletBrain can create new
    // instances of them whenever they are needed.
    scene_bullet: Gd<PackedScene>,
    scene_explosion: Gd<PackedScene>,
    scene_cloud: Gd<PackedScene>,

    // The underlying Godot Node that this Rust class inherits from.
    base: Base<Node>,
}

#[godot_api]
impl INode for BulletBrain {
    // init() is called when Godot creates the BulletBrain node.
    fn init(base: Base<Node>) -> Self {
        Self {
            // Starting difficulty settings.
            max_spawn_interval: 4.0,
            min_spawn_interval: 0.5,
            spawn_interval_decrease: 0.2,

            // This will be set to max_spawn_interval in ready().
            spawn_interval: 0.0,

            // Bullet movement speeds.
            player_bullet_speed: 300,
            enemy_bullet_speed: 250,

            // The Timer will be found later in ready().
            enemy_spawner: None,

            // Load the scenes from the Godot project.
            scene_bullet: load("res://scenes/bullet.tscn"),
            scene_explosion: load("res://scenes/explosion.tscn"),
            scene_cloud: load("res://scenes/cloud.tscn"),

            base,
        }
    }

    // ready() runs when the node has entered the scene tree
    // and its child nodes are available.
    fn ready(&mut self) {
        // Find the Timer named "enemySpawner" that belongs
        // to the BulletBrain node.
        let enemy_spawner = self.base().get_node_as::<Timer>("enemySpawner");

        // Start the game using the maximum spawn interval.
        self.spawn_interval = self.max_spawn_interval;

        // Set the Timer's wait time to match our spawn interval.
        let mut enemy_spawner = enemy_spawner;
        enemy_spawner.set_wait_time(self.spawn_interval as f64);

        // Store the Timer so we can change and restart it later.
        self.enemy_spawner = Some(enemy_spawner);
    }
}

#[godot_api]
impl BulletBrain {
    // Reduces the time between enemy spawns.
    //
    // Calling this function makes the game progressively harder.
    #[func]
    pub fn increase_difficulty(&mut self) {
        // Reduce the current interval.
        //
        // max() prevents the interval from becoming smaller
        // than min_spawn_interval.
        let new_spawn_interval =
            (self.spawn_interval - self.spawn_interval_decrease).max(self.min_spawn_interval);

        // Make sure the Timer exists before trying to use it.
        if let Some(enemy_spawner) = &mut self.enemy_spawner {
            // Apply the new interval to the Timer.
            enemy_spawner.set_wait_time(new_spawn_interval as f64);

            // Restart the Timer using the new interval.
            enemy_spawner.start();
        }

        // Save the new interval as the current difficulty.
        self.spawn_interval = new_spawn_interval;
    }

    // Called when the enemySpawner Timer reaches zero.
    #[func]
    pub fn on_enemy_spawner_timeout(&mut self) {
        self.spawn_enemy();
    }

    // Called when the cloud spawning Timer reaches zero.
    #[func]
    pub fn on_cloud_spawner_timeout(&mut self) {
        self.spawn_cloud();
    }

    // Creates a new enemy bullet.
    #[func]
    pub fn spawn_enemy(&mut self) {
        // Pick a random horizontal position near the top of the screen.
        let spawn_position = Vector2::new(randf_range(0.0, 1000.0) as f32, -30.0);

        // Pick a random target position near the bottom of the screen.
        let target_position = Vector2::new(randf_range(0.0, 1000.0) as f32, 550.0);

        // Create an enemy bullet and tell it which direction to travel.
        self.spawn_bullet(spawn_position, target_position, "enemy".into());
    }

    // Creates either a player or enemy bullet.
    //
    // animation_name determines which animation the bullet uses.
    #[func]
    pub fn spawn_bullet(
        &mut self,
        spawn_position: Vector2,
        target_position: Vector2,
        animation_name: GString,
    ) {
        // Create a new instance of bullet.tscn.
        //
        // instantiate_as::<Bullet>() converts the scene instance
        // into our custom Rust Bullet class.
        let mut bullet = self.scene_bullet.instantiate_as::<Bullet>();

        // Find the Node2D that contains all active bullets.
        let mut bullets = self.base().get_node_as::<Node2D>("/root/game/bullets");

        // Add the newly created bullet to the scene tree.
        bullets.add_child(&bullet);

        // Put the bullet at its starting position.
        bullet.set_global_position(spawn_position);

        // Rotate the bullet so it points toward its target.
        bullet.look_at(target_position);

        // Find the AnimatedSprite2D inside the bullet scene.
        let mut bullet_sprite = bullet.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // Select the correct animation.
        //
        // For example:
        // "player" → player bullet animation
        // "enemy"  → enemy bullet animation
        bullet_sprite.set_animation(animation_name.to_string().as_str());

        // Start playing the selected animation.
        bullet_sprite.play();

        // Choose the bullet speed based on who fired it.
        let bullet_speed = if animation_name == "player" {
            self.player_bullet_speed
        } else {
            self.enemy_bullet_speed
        };

        // Access the custom Rust Bullet object and set its speed.
        bullet.bind_mut().speed = bullet_speed as f32;
    }

    // Creates an explosion at the specified position.
    #[func]
    pub fn spawn_explosion(&mut self, spawn_position: Vector2, animation_name: GString) {
        // Create a new instance of the explosion scene.
        //
        // The explosion scene uses Area2D as its root node.
        let mut explosion = self.scene_explosion.instantiate_as::<Area2D>();

        // Find the Node2D containing the active bullets and explosions.
        let mut bullets = self.base().get_node_as::<Node2D>("/root/game/bullets");

        // Add the explosion to the scene tree.
        bullets.add_child(&explosion);

        // Move the explosion to the requested position.
        explosion.set_global_position(spawn_position);

        // Find the AnimatedSprite2D inside the explosion scene.
        let mut explosion_sprite = explosion.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // Select the requested explosion animation.
        explosion_sprite.set_animation(animation_name.to_string().as_str());

        // Play the explosion animation.
        explosion_sprite.play();
    }

    // Creates a cloud and places it in the foreground.
    #[func]
    pub fn spawn_cloud(&mut self) {
        // Create a new cloud from cloud.tscn.
        //
        // The cloud scene's root node is an AnimatedSprite2D,
        // so we can instantiate it directly as AnimatedSprite2D.
        let mut cloud = self.scene_cloud.instantiate_as::<AnimatedSprite2D>();

        // Find the foreground Node2D where clouds should appear.
        let mut foreground = self.base().get_node_as::<Node2D>("/root/game/foreground");

        // Add the cloud to the foreground.
        foreground.add_child(&cloud);

        // Randomly choose one of the cloud sprite frames.
        //
        // randf_range() returns a floating-point number, so it is
        // converted to an integer before being used as a frame index.
        cloud.set_frame(randf_range(0.0, 3.0) as i32);

        // Start the cloud slightly outside the left side of the screen.
        //
        // A negative X value means the cloud starts off-screen
        // and can move into view later.
        let spawn_position = Vector2::new(-100.0, randf_range(0.0, 400.0) as f32);

        // Move the cloud to its random starting position.
        cloud.set_global_position(spawn_position);

        // Choose a random size for the cloud.
        let random_scale = randf_range(0.0, 1.0) as f32;

        // Apply the same random value to X and Y so the cloud
        // keeps its original proportions instead of stretching.
        cloud.set_scale(Vector2::new(random_scale, random_scale));
    }
}
