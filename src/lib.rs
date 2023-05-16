//! A Token Metadata program for the Solana blockchain.
//!
//! The program attach additional data to Fungible or Non-Fungible Tokens on Solana.

pub mod assertions;

// (Re-)Declare modules to maintain API compatibility.

pub mod escrow {
    pub use crate::{instruction::escrow::*, processor::escrow::*};
}

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod pda;
pub mod processor;
pub mod state;
pub mod utils;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("BrUqabiDc3BT33RrCHaYaZT7p2RPAaHGmCWWnRwntx8u");