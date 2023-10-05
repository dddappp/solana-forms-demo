use anchor_lang::prelude::*;

use crate::event::MainFormUpdated;
use crate::state::MainForm;
use crate::context::Update;

pub(crate) fn verify(
    fr_5pqi: u128,
    fr_duif: Vec<String>,
    fr_6i34: Vec<String>,
    fr_8xjs: String,
    fr_9c3f: String,
    fr_4ok6: u128,
    fr_b3ub: String,
    fr_1z7o: u128,
    fr_d8rw: Vec<String>,
    fr_dy3l: Vec<String>,
    fr_6f68: u128,
    fr_47yy: Vec<String>,
    fr_gh3o: Vec<String>,
    fr_fbba: Vec<String>,
    fr_hhzp: String,
    single_text1: String,
    main_form: &Account<MainForm>,
    ctx: &Context<Update>,
) -> MainFormUpdated {
    let _ = ctx;
    MainFormUpdated {
        signer_address: main_form.signer_address,
        version: main_form.version,
        fr_5pqi,
        fr_duif,
        fr_6i34,
        fr_8xjs,
        fr_9c3f,
        fr_4ok6,
        fr_b3ub,
        fr_1z7o,
        fr_d8rw,
        fr_dy3l,
        fr_6f68,
        fr_47yy,
        fr_gh3o,
        fr_fbba,
        fr_hhzp,
        single_text1,
    }
}

pub(crate) fn mutate(
    main_form_updated: &MainFormUpdated,
    main_form: &Account<MainForm>,
) -> MainForm {
    MainForm {
        signer_address: main_form.signer_address.clone(),
        version: main_form.version + 1,
        fr_5pqi: main_form_updated.fr_5pqi.clone(),
        fr_duif: main_form_updated.fr_duif.clone(),
        fr_6i34: main_form_updated.fr_6i34.clone(),
        fr_8xjs: main_form_updated.fr_8xjs.clone(),
        fr_9c3f: main_form_updated.fr_9c3f.clone(),
        fr_4ok6: main_form_updated.fr_4ok6.clone(),
        fr_b3ub: main_form_updated.fr_b3ub.clone(),
        fr_1z7o: main_form_updated.fr_1z7o.clone(),
        fr_d8rw: main_form_updated.fr_d8rw.clone(),
        fr_dy3l: main_form_updated.fr_dy3l.clone(),
        fr_6f68: main_form_updated.fr_6f68.clone(),
        fr_47yy: main_form_updated.fr_47yy.clone(),
        fr_gh3o: main_form_updated.fr_gh3o.clone(),
        fr_fbba: main_form_updated.fr_fbba.clone(),
        fr_hhzp: main_form_updated.fr_hhzp.clone(),
        single_text1: main_form_updated.single_text1.clone(),
    }
}
