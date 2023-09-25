use anchor_lang::prelude::*;

#[event]
pub struct MainFormCreated {
    pub signer_address: Pubkey,
}

