//! This module provides a collection of OpenGL utilities and abstractions.
//!
//! The `gl` module is organized into several submodules that handle different
//! aspects of OpenGL operations, such as setting up OpenGL states, handling vertex
//! array objects (VAO), managing colors, and providing debugging utilities.
//! 
//! # Safety
//! 
//! Most of the functions are marked as `unsafe` because it interacts with raw OpenGL functions 
//! that require manual memory management. The caller is responsible for ensuring that the OpenGL 
//! context is properly initialized before calling this function.
//!
//! # Submodules
//!
//! - `types`: Contains type definitions and constants for OpenGL.
//! - `debugging`: Provides OpenGL debugging functionality.
//! - `setup`: Handles OpenGL initialization and configuration.
//! - `color`: Manages color-related functions and utilities.
//! - `rendering`: Handles rendering-related functions.
//! - `vao`: Functions for creating and managing vertex array objects.
//! - `state`: Manages and queries OpenGL state, including retrieving values
//!    of specific OpenGL parameters.

use ogl::types::GLuint;

pub mod types;
pub mod debugging;
pub mod setup;
pub mod color;
pub mod rendering;
pub mod vao;
pub mod state;
pub mod buffer;

pub trait GLConstant {
    fn to_gl_constant(self) -> GLuint;
}
