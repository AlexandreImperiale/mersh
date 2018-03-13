#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate serde_derive;

/// Definition of basic geometrical utilities.
pub mod base;

/// Definition of mersh interpreter
pub mod interpreter;

/// Data structures for tagging utilities.
pub mod tag;

/// Data structures representing mesh elements.
pub mod elements;

/// Definition of meshes.
pub mod mesh;

/// Definition of view on mesh elements.
pub mod views;
