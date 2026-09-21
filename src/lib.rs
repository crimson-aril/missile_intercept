use godot::prelude::*;

// These modules contain the Rust scripts used by the game.
//
// #[path = "..."] tells Rust where to find the module file.
// This is useful here because the scripts are stored in a separate
// "scripts" folder instead of next to this file.

#[path = "../scripts/bullet.rs"]
mod bullet;

#[path = "../scripts/cloud.rs"]
mod cloud;

#[path = "../scripts/bullet_brain.rs"]
mod bullet_brain;

#[path = "../scripts/bullet_stopper.rs"]
mod bullet_stopper;

#[path = "../scripts/cannon_barrel.rs"]
mod cannon_barrel;

#[path = "../scripts/explosion.rs"]
mod explosion;

#[path = "../scripts/player.rs"]
mod player;

#[path = "../scripts/scenes.rs"]
mod scenes;

// This is the main library/extension for our Godot project.
//
// The struct itself does not need to contain anything.
// Its purpose is to act as the entry point for the Rust extension.
struct MissileDefense;

// #[gdextension] tells Godot-Rust that this type represents
// the Godot extension library.
//
// ExtensionLibrary is responsible for registering the Rust classes,
// functions, and other Godot-related features with Godot when the
// game starts.
//
// "unsafe" is required by the Godot-Rust API for implementing
// the ExtensionLibrary trait.
#[gdextension]
unsafe impl ExtensionLibrary for MissileDefense {}
