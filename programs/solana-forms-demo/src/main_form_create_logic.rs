// use anchor_lang::prelude::*;
//
// use crate::event::MainFormCreated;
// use crate::state::MainForm;
//
// pub(crate) fn verify(
//     signer_address: Pubkey,
//     fr_5pqi: u128,
//     fr_duif: Vec<String>,
//     fr_6i34: Vec<String>,
//     fr_8xjs: String,
// ) -> MainFormCreated {
//     MainFormCreated {
//         signer_address,
//         fr_5pqi,
//         fr_duif,
//         fr_6i34,
//         fr_8xjs,
//     }
// }
//
// pub(crate) fn mutate(
//     main_form_created: &MainFormCreated,
// ) -> MainForm {
//     MainForm {
//         signer_address: main_form_created.signer_address,
//         version: 0,
//         fr_5pqi: main_form_created.fr_5pqi.clone(),
//         fr_duif: main_form_created.fr_duif.clone(),
//         fr_6i34: main_form_created.fr_6i34.clone(),
//         fr_8xjs: main_form_created.fr_8xjs.clone(),
//     }
// }
