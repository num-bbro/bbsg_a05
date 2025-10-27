use crate::dcl::Pan;
use crate::dcl::PeaAssVar;
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub sbid: Option<String>,
}

use crate::dcl::VarType;
use crate::dcl::DNM;
use crate::p08::ld_sub_info;
use crate::p08::SubInfo;
use std::collections::HashMap;

const FLD_LIST: [(VarType, &str); 17] = [
    (VarType::SmallSellTr, ""),
    (VarType::HmChgEvTr, "/tr01"),
    (VarType::CntLvPowSatTr, ""),
    (VarType::ChgStnCap, ""),
    //(VarType::ChgStnSell, ""),
    (VarType::MvPowSatTr, ""),
    (VarType::SolarRoof, ""),
    (VarType::ZoneTr, "/tr02"),
    (VarType::PopTr, "/tr02"),
    (VarType::MvVspp, ""),
    (VarType::HvSpp, ""),
    //(VarType::UnbalPow, ""),
    (VarType::CntUnbalPow, ""),
    (VarType::Uc1Val, ""),
    (VarType::Uc2Val, ""),
    (VarType::Uc3Val, ""),
    (VarType::Uc1Rank, ""),
    (VarType::Uc2Rank, ""),
    (VarType::Uc3Rank, ""),
];

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "sba01.html")]
pub struct WebTemp {
    name: String,
    assv: Vec<PeaAssVar>,
    sbif: HashMap<String, SubInfo>,
    flds: Vec<(VarType, &'static str)>,
    aojsbh: HashMap<String, Vec<String>>,
}

impl WebTemp {
    /*
    fn aoj5(&self, sa: &PeaAssVar) -> bool {
        let (mut c1, mut c2) = (0, 0);
        for s in &sa.aojv {
            c1 += 1;
            if let Some(vv) = self.aojsbh.get(&s.code) {
                c2 += vv.len();
            }
        }
        c1 <= 3 && c2 <= 6
    }
    */
    fn aoj4(&self, sa: &PeaAssVar) -> String {
        let (mut c1, mut c2) = (0, 0);
        let mut ss = String::new();
        use std::fmt::Write;
        for s in &sa.aojv {
            if !ss.is_empty() {
                write!(ss, ",").unwrap();
            }
            write!(ss, "{}", s.name).unwrap();
            c1 += 1;
            if let Some(vv) = self.aojsbh.get(&s.code) {
                write!(ss, "[{}]", vv.len()).unwrap();
                c2 += vv.len();
            }
        }
        format!("{c1}:{c2}:{ss}")
        //format!("{}-{}", sa.aojv.len(), self.aojsbv.len())
    }
}

//use axum::extract::Query;
//pub async fn sb01(para: Query<Param>) -> WebTemp {
pub async fn sba01() -> WebTemp {
    // ============================
    // ==== read rw3 data
    let Ok(buf) = std::fs::read(format!("{DNM}/000-sbno.bin")) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    //println!("read normalized data");
    // ==== read rw3 data
    let Ok((mut assv, _)): Result<(Vec<crate::dcl::PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3:");
        return WebTemp::default();
    };
    // ===== AOJ_sbv
    let Ok(buf) = std::fs::read(format!("{DNM}/aoj_sbv.bin")) else {
        println!("NO aoj_sbv.bin");
        return WebTemp::default();
    };
    let Ok((aojsbh, _)): Result<(HashMap<String, Vec<String>>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode aojsbv:");
        return WebTemp::default();
    };

    assv.sort_by(|a, b| {
        let ar = a.v[VarType::Uc1Rank.tousz()].v
            + a.v[VarType::Uc2Rank.tousz()].v
            + a.v[VarType::Uc3Rank.tousz()].v;
        let br = b.v[VarType::Uc1Rank.tousz()].v
            + b.v[VarType::Uc2Rank.tousz()].v
            + b.v[VarType::Uc3Rank.tousz()].v;
        ar.partial_cmp(&br).unwrap()
    });
    //let sbif = sub_inf(); //HashMap<String, SubstInfo>
    let sbif = ld_sub_info();
    WebTemp {
        name: "Substation - sba01 (sort by usecase rank)".to_string(),
        assv,
        sbif: sbif.clone(),
        flds: FLD_LIST.to_vec(),
        aojsbh,
    }
}
