use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MainForm {
    pub signer_address: Pubkey,
    pub version: u64,
    pub fr_5pqi: u128,
    #[max_len(50, 5)]
    pub fr_duif: Vec<String>,
    #[max_len(50, 5)]
    pub fr_6i34: Vec<String>,
    #[max_len(255)]
    pub fr_8xjs: String,
}
