use anchor_lang::prelude::*;

pub mod context;
pub mod processor;
pub mod state;
use crate::context::*;

declare_id!("9dTbbtqwNo2KcECHDPX16sYNQNQ5dduEwFnGPG29mmDH");

#[program]
pub mod solana_forms_demo {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        Ok(())
    }
}
