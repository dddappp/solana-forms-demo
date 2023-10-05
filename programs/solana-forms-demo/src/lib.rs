use anchor_lang::prelude::*;
pub mod context;
pub mod state;
pub mod event;

use crate::context::*;
//use crate::state::*;
//use crate::event::*;

mod main_form_create_logic;

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
        let signer_address = *ctx.accounts.authority.key;
        let main_form_created = main_form_create_logic::verify(
            signer_address,
            arg_1,
            arg_2,
            arg_3,
            arg_4,
            arg_5,
            arg_6,
            arg_7,
            arg_8,
            arg_9,
            arg_10,
            arg_11,
            arg_12,
            arg_13,
            arg_14,
            arg_15,
            arg_16,
            &ctx,
        );
        assert_eq!(signer_address, main_form_created.signer_address, "SignerAddress of event does not match");
        let main_form = main_form_create_logic::mutate(
            &main_form_created,
        );
        assert_eq!(signer_address, main_form.signer_address, "SignerAddress of state does not match");
        assert_eq!(0, main_form.version, "Version of state does not match");
        *ctx.accounts.main_form = main_form;
        emit!(main_form_created);

        Ok(())
    }
}
