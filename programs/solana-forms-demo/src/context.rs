use crate::state::*;
use anchor_lang::prelude::*;
//use anchor_spl::token::{Mint, Token, TokenAccount};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
    init,
    seeds = [
        b"MainForm".as_ref(),
        authority.key().as_ref(),
    ],
    bump,
    payer = authority,
    space = 8 + 2000
    )]
    pub main_form: Account<'info, MainForm>,

    /// Account that whose going to pay for this transaction.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System Program
    pub rent: Sysvar<'info, Rent>,

    /// System Program
    pub system_program: Program<'info, System>,
}
