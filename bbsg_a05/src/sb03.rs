use crate::dcl::Pan;
use crate::dcl::PeaAssVar;
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub sbid: Option<String>,
}

use crate::p08::ld_sub_info;
use crate::p08::SubInfo;
//use sglab02_lib::sg::prc1::SubstInfo;
//use sglab02_lib::sg::prc5::sub_inf;
use crate::dcl::VarType;
use crate::dcl::DNM;
use std::collections::HashMap;

const FLD_LIST: [(VarType, &str); 14] = [
    (VarType::SmallSellTr, ""),
    (VarType::HmChgEvTr, "/tr01"),
    (VarType::CntLvPowSatTr, ""),
    (VarType::ChgStnCap, ""),
    (VarType::ChgStnSell, ""),
    (VarType::MvPowSatTr, ""),
    (VarType::SolarRoof, ""),
    (VarType::ZoneTr, ""),
    (VarType::PopTr, ""),
    (VarType::MvVspp, ""),
    (VarType::HvSpp, ""),
    (VarType::UnbalPow, ""),
    (VarType::CntUnbalPow, ""),
    (VarType::Uc1Val, ""),
];

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "sb03.html")]
pub struct WebTemp {
    name: String,
    assv: Vec<PeaAssVar>,
    sbif: HashMap<String, SubInfo>,
    flds: Vec<(VarType, &'static str)>,
}

//use axum::extract::Query;
//pub async fn sb01(para: Query<Param>) -> WebTemp {
pub async fn sb03() -> WebTemp {
    // ============================
    // ==== read rw3 data
    let Ok(buf) = std::fs::read(format!("{DNM}/000-sbrw.bin")) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    // ==== read rw3 data
    let Ok((mut assv, _)): Result<(Vec<PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3:");
        return WebTemp::default();
    };
    assv.sort_by(|b, a| {
        a.v[VarType::HmChgEvTr.tousz()]
            .v
            .partial_cmp(&b.v[VarType::HmChgEvTr.tousz()].v)
            .unwrap()
    });

    //let sbif = sub_inf(); //HashMap<String, SubstInfo>
    let sbif = ld_sub_info();
    WebTemp {
        name: "sb03 - Substation sort by EV".to_string(),
        assv,
        sbif: sbif.clone(),
        flds: FLD_LIST.to_vec(),
    }
}
