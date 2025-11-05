use crate::dcl::Geo;
use crate::dcl::Pan;
use crate::dcl::PeaAssVar;
use crate::dcl::VarType;
use crate::dcl::DNM;
use crate::dcl::SHOW_FLDS;
use crate::dcl::SSHOW_YEAR_BEG;
use crate::dcl::SSHOW_YEAR_END;
use crate::p08::ld_sub_info;
use crate::p08::SubInfo;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use serde::Deserialize;
//use sglib04::geo1::n1d_2_utm;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub fld: Option<String>,
    pub sbid: Option<String>,
    pub fdid: Option<String>,
}

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "tra00.html")]
pub struct WebTemp {
    name: String,
    sbid: String,
    fdid: String,
    fld: String,
    assv: Vec<PeaAssVar>,
    sbinf: SubInfo,
    //sbif: HashMap<String, SubInfo>,
    se_fld: VarType,
    shwfld: Vec<VarType>,
}

pub async fn page(para: Query<Param>) -> WebTemp {
    let mut fldm = HashMap::<String, VarType>::new();
    for vt in &SHOW_FLDS {
        let fd = format!("{:?}", vt);
        fldm.insert(fd, vt.clone());
    }
    let fld = if let Some(fld) = &para.fld {
        fld.clone()
    } else {
        format!("{:?}", SHOW_FLDS[0])
    };
    let sbid = if let Some(sbid) = &para.sbid {
        sbid.clone()
    } else {
        "KLO".to_string()
    };
    let fdid = if let Some(fdid) = &para.fdid {
        fdid.clone()
    } else {
        format!("{sbid}01")
    };
    let Some(se_fld) = fldm.get(&fld) else {
        println!("NO SELECTED FIELD");
        return WebTemp::default();
    };
    let name = format!("FIELD {fld}");
    //let Ok(buf) = std::fs::read(format!("{DNM}/000-sbrw.bin")) else {
    let Ok(buf) = std::fs::read(format!("{DNM}/{sbid}-rw4.bin")) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    // ==== read rw3 data
    let Ok((assv0, _)): Result<(Vec<PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3:");
        return WebTemp::default();
    };
    let assv0 = assv0
        .iter()
        .filter(|a| a.fdid == fdid && a.own == "P")
        .cloned()
        .collect::<Vec<_>>();
    let mut sumv = PeaAssVar::from(0u64);
    let mut assv = Vec::<PeaAssVar>::new();
    for ass in assv0 {
        sumv.add(&ass);
        assv.push(ass);
    }
    assv.push(sumv);

    let sbif = ld_sub_info();
    let Some(sbinf) = sbif.get(&sbid) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };
    WebTemp {
        name,
        assv,
        fld,
        sbid,
        fdid,
        sbinf: sbinf.clone(),
        //sbif: sbif.clone(),
        //flds: FLD_LIST.to_vec(),
        se_fld: se_fld.clone(),
        shwfld: SHOW_FLDS.to_vec(),
    }
}
