use anchor_lang::prelude::*;

pub mod context;
pub mod processor;
pub mod state;
pub mod event;

use crate::context::*;
use crate::state::*;

mod main_form_create_logic;


declare_id!("9dTbbtqwNo2KcECHDPX16sYNQNQ5dduEwFnGPG29mmDH");

#[program]
pub mod solana_forms_demo {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        fr_d5pqi: u128,
        // fr_duif: Vec<String>,
        // fr_6i34: Vec<String>,
        // fr_8xjs: String,
    ) -> Result<()> {
        msg!("fr_5pqi: {}", fr_d5pqi);
        msg!("ctx.accounts.authority.key: {}", *ctx.accounts.authority.key);

        *ctx.accounts.main_form = MainForm {
            signer_address: ctx.accounts.authority.key(),
            version: 0,
            fr_5pqi: fr_d5pqi,
            fr_duif: vec![],
            fr_6i34: vec![],
            fr_8xjs: "".to_string(),
        };
        Ok(())
    }
}
