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

    pub fn initialize(
        ctx: Context<Create>,
        fr_5pqi: u128,
        fr_duif: Vec<String>,
        fr_6i34: Vec<String>,
        fr_8xjs: String,
    ) -> Result<()> {
        *ctx.accounts.main_form = MainForm {
            signer_address: *ctx.accounts.authority.key,
            version: 0,
            fr_5pqi,
            fr_duif,
            fr_6i34,
            fr_8xjs,
        };
        emit!(MainFormCreated {
            signer_address: *ctx.accounts.authority.key
        });

        Ok(())
    }
}
