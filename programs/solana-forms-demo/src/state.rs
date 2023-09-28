use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MainForm {
    pub signer_address: Pubkey,
    pub version: u64,
    pub fr_5pqi: u128,
    #[max_len(2, 1)]
    pub fr_duif: Vec<String>,
    #[max_len(3, 1)]
    pub fr_6i34: Vec<String>,
    #[max_len(255)]
    pub fr_8xjs: String,
    #[max_len(1)]
    pub fr_9c3f: String,
    pub fr_4ok6: u128,
    #[max_len(255)]
    pub fr_b3ub: String,
    pub fr_1z7o: u128,
    #[max_len(2, 20)]
    pub fr_d8rw: Vec<String>,
    #[max_len(2, 20)]
    pub fr_dy3l: Vec<String>,
    pub fr_6f68: u128,
    #[max_len(2, 20)]
    pub fr_47yy: Vec<String>,
    #[max_len(2, 1)]
    pub fr_gh3o: Vec<String>,
    #[max_len(3, 1)]
    pub fr_fbba: Vec<String>,
    #[max_len(255)]
    pub fr_hhzp: String,
    #[max_len(50)]
    pub single_text1: String,
}
