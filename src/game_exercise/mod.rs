//! # Game Exercise Module
//! 
//! A simple game where a player moves on a 2D board, eating food to gain strength
//! and avoiding poison that reduces it. The goal is to survive for a set number of moves.

pub mod direction;
pub mod cell;
pub mod player;
pub mod game;

// Re-export main types for convenience
pub use direction::Direction;
pub use cell::Cell;
pub use player::Player;
pub use game::Game;
