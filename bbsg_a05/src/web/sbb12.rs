use crate::dcl::Pan;
use crate::dcl::PeaAssVar;
use crate::dcl::VarType;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub fld: Option<String>,
}

use crate::dcl::DNM;
use crate::dcl::SHOW_FLDS2;
use crate::p08::ld_sub_info;
use crate::p08::SubInfo;
use std::collections::HashMap;

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "sbb0t.html")]
pub struct WebTemp {
    name: String,
    assv: Vec<PeaAssVar>,
    sbif: HashMap<String, SubInfo>,
    shwfld: Vec<VarType>,
}

pub async fn page(_para: Query<Param>) -> WebTemp {
    let name = "PROVINCE".to_string();
    let Ok(buf) = std::fs::read(format!("{DNM}/000-pvrw.bin")) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    // ==== read rw3 data
    let Ok((mut assv0, _)): Result<(Vec<PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3:");
        return WebTemp::default();
    };
    assv0.sort_by(|b, a| {
        a.v[VarType::FirCstRate.tousz()]
            .v
            .partial_cmp(&b.v[VarType::FirCstRate.tousz()].v)
            .unwrap()
    });
    let mut sumv = PeaAssVar::from(0u64);
    let mut assv = Vec::<PeaAssVar>::new();
    for ass in assv0.iter().take(25) {
        sumv.add(ass);
        assv.push(ass.clone());
    }
    assv.push(sumv);

    let sbif = ld_sub_info();
    WebTemp {
        name,
        assv,
        sbif: sbif.clone(),
        shwfld: SHOW_FLDS2.to_vec(),
    }
}
