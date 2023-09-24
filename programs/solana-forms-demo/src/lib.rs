use anchor_lang::prelude::*;

pub mod context;
pub mod processor;
pub mod state;
pub mod event;

use crate::context::*;

mod main_form_create_logic;


declare_id!("9dTbbtqwNo2KcECHDPX16sYNQNQ5dduEwFnGPG29mmDH");

#[program]
pub mod solana_forms_demo {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        fr_5pqi: u128,
        fr_duif: Vec<String>,
        fr_6i34: Vec<String>,
        fr_8xjs: String,
    ) -> Result<()> {

        Ok(())
    }
}
