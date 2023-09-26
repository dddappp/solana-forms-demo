use anchor_lang::prelude::*;
pub mod context;
pub mod state;
pub mod event;

use crate::context::*;
use crate::state::*;
use crate::event::*;

declare_id!("DJYREhvzFLcicT1aaqgsSigLkj8v3vwiiWKaPyPFTcos");

#[program]
pub mod solana_forms_demo {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        arg_1: u128,
        arg_2: Vec<String>,
        arg_3: Vec<String>,
        arg_4: String,
        arg_5: String,
        arg_6: u128,
        arg_7: String,
        arg_8: u128,
        arg_9: Vec<String>,
        arg_10: Vec<String>,
        arg_11: u128,
        arg_12: Vec<String>,
        arg_13: Vec<String>,
        arg_14: Vec<String>,
        arg_15: String,
        arg_16: String,
    ) -> Result<()> {
        *ctx.accounts.main_form = MainForm {
            signer_address: *ctx.accounts.authority.key,
            version: 0,
            fr_5pqi: arg_1,
            fr_duif: arg_2,
            fr_6i34: arg_3,
            fr_8xjs: arg_4,
            fr_9c3f: arg_5,
            fr_4ok6: arg_6,
            fr_b3ub: arg_7,
            fr_1z7o: arg_8,
            fr_d8rw: arg_9,
            fr_dy3l: arg_10,
            fr_6f68: arg_11,
            fr_47yy: arg_12,
            fr_gh3o: arg_13,
            fr_fbba: arg_14,
            fr_hhzp: arg_15,
            single_text1: arg_16,
        };
        emit!(MainFormCreated {
            signer_address: *ctx.accounts.authority.key
        });

        Ok(())
    }
}
