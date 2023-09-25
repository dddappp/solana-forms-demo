use anchor_lang::prelude::*;
pub mod context;
pub mod state;
pub mod event;

use crate::context::*;
use crate::state::*;
use crate::event::*;

declare_id!("9dTbbtqwNo2KcECHDPX16sYNQNQ5dduEwFnGPG29mmDH");

#[program]
pub mod solana_forms_demo {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        arg_1: u128,
        arg_2: Vec<String>,
        arg_3: Vec<String>,
        arg_4: String,
    ) -> Result<()> {
        *ctx.accounts.main_form = MainForm {
            signer_address: *ctx.accounts.authority.key,
            version: 0,
            fr_5pqi: arg_1,
            fr_duif: arg_2,
            fr_6i34: arg_3,
            fr_8xjs: arg_4,
        };
        emit!(MainFormCreated {
            signer_address: *ctx.accounts.authority.key
        });

        Ok(())
    }
}
