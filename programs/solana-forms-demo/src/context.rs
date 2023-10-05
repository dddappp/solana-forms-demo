use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + MainForm::INIT_SPACE,
        seeds = [
            b"MainForm",
            authority.key().as_ref(),
        ],
        bump
    )]
    pub main_form: Account<'info, MainForm>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [
            b"MainForm",
            main_form.signer_address.as_ref(),
        ],
        bump
    )]
    pub main_form: Account<'info, MainForm>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

