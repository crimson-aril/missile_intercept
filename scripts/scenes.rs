use godot::prelude::*;

// #[derive(GodotClass)] registers this Rust struct as a class
// that can be used by Godot.
//
// #[class(base = Node)] means this class inherits from Godot's Node.
// The "base" field below stores the connection to that Node.
#[derive(GodotClass)]
#[class(base = Node)]
pub struct Scenes {
// Store the PackedScene resources that we want to reuse.
//
// A PackedScene is a saved Godot scene (.tscn) that can be
// loaded and instantiated later during the game.
pub scene_bullet: Gd<PackedScene>,
pub scene_explosion: Gd<PackedScene>,
pub scene_bullet_stopper: Gd<PackedScene>,
pub scene_cloud: Gd<PackedScene>,


// Godot requires the base Node to be stored in the Rust class.
//
// Base<Node> gives this Rust object access to the underlying
// Godot Node that it inherits from.
base: Base<Node>,


}

// This implementation provides the initialization code for
// the Godot Node.
//
// INode is the Godot-Rust trait used for Node-based classes.
#[godot_api]
impl INode for Scenes {
// Godot calls init() when this class is created.
//
// The "base" parameter is the underlying Godot Node.
fn init(base: Base<Node>) -> Self {
Self {
// Load each scene from the Godot project.
//
// load() uses a "res://" path, which starts from the
// root folder of the Godot project.
scene_bullet: load("res://scenes/bullet.tscn"),
scene_explosion: load("res://scenes/explosion.tscn"),
scene_bullet_stopper: load("res://scenes/bullet_stopper.tscn"),
scene_cloud: load("res://scenes/cloud.tscn"),


        // Save the Godot Node provided by the engine.
        base,
    }
}


}

// This implementation contains functions that can be called
// from Godot.
//
// The #[godot_api] attribute exposes the implementation to
// Godot-Rust.
#[godot_api]
impl Scenes {


// #[func] exposes this Rust function to Godot.
//
// This function returns the loaded bullet scene.
#[func]
pub fn get_bullet(&self) -> Gd<PackedScene> {
    // clone() creates another handle to the same PackedScene.
    // We clone the handle because the original scene is still
    // stored inside this Scenes object.
    self.scene_bullet.clone()
}


// Return the loaded explosion scene.
#[func]
pub fn get_explosion(&self) -> Gd<PackedScene> {
    self.scene_explosion.clone()
}


// Return the loaded bullet stopper scene.
#[func]
pub fn get_bullet_stopper(&self) -> Gd<PackedScene> {
    self.scene_bullet_stopper.clone()
}


// Return the loaded cloud scene.
#[func]
pub fn get_cloud(&self) -> Gd<PackedScene> {
    self.scene_cloud.clone()
}


}
