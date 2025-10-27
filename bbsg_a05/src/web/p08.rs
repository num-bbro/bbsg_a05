use crate::dcl::Pan;
use crate::dcl::PeaAssVar;
use crate::dcl::VarType;
use crate::dcl::SSHOW_YEAR_BEG;
use crate::dcl::SSHOW_YEAR_END;
use crate::utl::get_pvrw00;
use crate::utl::get_sbrw00;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub ben: Option<usize>,
}

pub struct WebTempRow {
    pvid: String,
    evgr: Vec<f32>,
}

#[derive(Template, WebTemplate)]
#[template(path = "p08.html")]
pub struct WebTemp {
    name: String,
    rows: Vec<WebTempRow>,
}

pub const PROV: [&str; 25] = [
    "สมุทรสาคร",
    "พระนครศรีอยุธยา",
    "ปทุมธานี",
    "ชลบุรี",
    "ระยอง",
    "ฉะเชิงเทรา",
    "นครปฐม",
    "ปราจีนบุรี",
    "สงขลา",
    "ราชบุรี",
    "ภูเก็ต",
    "นครสวรรค์",
    "ระนอง",
    "สมุทรสงคราม",
    "กระบี่",
    "เพชรบุรี",
    "สุราษฎร์ธานี",
    "สระบุรี",
    "นครราชสีมา",
    "เชียงใหม่",
    "พิษณุโลก",
    "ขอนแก่น",
    "ลพบุรี",
    "บุรีรัมย์",
    "สระแก้ว",
];

use sglib04::prc43::BENET;

fn ben_nm(n: usize) -> String {
    for (i, (h, _)) in BENET.iter().enumerate() {
        let i = i + 1;
        let h = h.trim();
        if i == n {
            return h.to_string();
        }
        //println!("{i}=={h}");
    }
    "".to_string()
}

pub async fn p08(para: Query<Param>) -> WebTemp {
    ben_nm(0);
    let pvrw = get_pvrw00();
    //println!("pvrw:{}", pvrw.len());
    let mut pvasm = HashMap::<String, PeaAssVar>::new();
    for a in pvrw {
        pvasm.insert(a.pvid.to_string(), a.clone());
    }
    let sbrw = get_sbrw00();
    let mut pvrwm = HashMap::<String, usize>::new();
    let mut pvrwv = Vec::<PeaAssVar>::new();
    for (i, pv) in PROV.iter().enumerate() {
        let pv = pv.to_string();
        let Some(a) = pvasm.get(&pv) else {
            println!("error {}", pv);
            continue;
        };
        pvrwv.push(a.clone());
        pvrwm.insert(pv.to_string(), i);
    }
    //let mut cn = 0;
    for sr in sbrw.iter() {
        let note = format!("{}", sr.v[VarType::TakeNote as usize].v);
        if note != "1" {
            continue;
        }
        if sr.sbid == "SQB" {
            continue;
        }
        let pv = sr.pvid.to_string();
        let Some(pvi) = pvrwm.get_mut(&pv) else {
            println!("error {pv}");
            continue;
        };
        //cn += 1;
        pvrwv[*pvi].add(sr);
    }
    let evs: f32 = sbrw
        .iter()
        .map(|sr| sr.v[VarType::NoHmChgEvTr as usize].v)
        .sum();
    let mut evcs2 = vec![0f32; sbrw.len()];
    for (i, sr) in pvrwv.iter().enumerate() {
        evcs2[i] = sr.v[VarType::NoHmChgEvTr as usize].v / evs;
    }
    let mut b_be = VarType::FirBilAccu.tousz();
    let mut b_en = VarType::FirDataEntrySave.tousz();
    let mut ben_id = 0;
    let mut ben_no = 0;
    if let Some(ben) = para.ben {
        ben_no = ben - 7;
        let b_of = ben - 8;
        //println!("ben: {ben} {b_of}");
        ben_id = ben;
        b_be += b_of;
        b_en = b_be;
    }
    //println!("cnt: {cn}");
    let mut rows2 = Vec::<WebTempRow>::new();
    let mut bbbs = vec![0.0; 20];
    for (_, sr) in pvrwv.iter().enumerate() {
        //let evgr = get_scurv();
        let b8 = sr.vy[VarType::FirBilAccu.tousz()].clone();
        let mut ben = vec![0.0; b8.len()];
        //for vi in VarType::FirBilAccu.tousz()..=VarType::FirDataEntrySave.tousz() {
        for vi in b_be..=b_en {
            let mut bbs = 0.0;
            for (bi, bn) in ben.iter_mut().enumerate() {
                *bn += sr.vy[vi][bi];
                bbs += sr.vy[vi][bi];
            }
            bbbs[vi - VarType::FirBilAccu.tousz()] += bbs;
        }
        let mut ben2 = vec![0.0; 3];
        ben2.append(&mut ben);
        //println!("{} - ev:{} b8:{}", sr.pvid, evgr.len(), ben2.len());
        let rw = WebTempRow {
            pvid: sr.pvid.to_string(),
            evgr: ben2,
        };
        rows2.push(rw);
    }
    /*
    for (i, bbb) in bbbs.iter().enumerate() {
        let i = i + 1;
        let bbb = bbb / 1_000_000.0;
        println!("{i} - {bbb}");
    }
    */

    WebTemp {
        name: format!(
            "Energy Management Benefit - {ben_no} ({ben_id}) - {}",
            ben_nm(ben_id)
        ),
        rows: rows2,
    }
}
