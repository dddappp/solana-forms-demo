use anchor_lang::prelude::*;

#[account]
pub struct MainForm {
    pub signer_address: Pubkey,
    pub version: u64,
    pub fr_5pqi: u128,
    pub fr_duif: Vec<String>,
    pub fr_6i34: Vec<String>,
    pub fr_8xjs: String,
}
