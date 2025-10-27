use crate::dcl::Pan;
use crate::utl::get_sbrw00;
use askama::Template;
use askama_web::WebTemplate;
//use sglib05::c04::Pea;
use crate::dcl::PeaAssVar;
use crate::dcl::VarType;
use crate::dcl::EV_AT_2050;
use crate::dcl::EV_HR_DAY;
use crate::dcl::EV_MWH_BAHT;
use crate::dcl::SSHOW_YEAR_BEG;
use crate::dcl::SSHOW_YEAR_END;
use crate::utl::get_scurv;

pub struct WebTempRow {
    sbid: String,
    pvid: String,
    note: String,
    evrt: f32,
    evgr: Vec<f32>,
}

#[derive(Template, WebTemplate)]
#[template(path = "p06.html")]
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

//use sglib05::c04::PeaAssVar;
use std::collections::HashMap;

pub async fn p06() -> WebTemp {
    let sbrw = get_sbrw00();
    let mut pvrwm = HashMap::<String, usize>::new();
    let mut pvrwv = Vec::<PeaAssVar>::new();
    for (i, pv) in PROV.iter().enumerate() {
        let mut ass = PeaAssVar::from(0);
        ass.pvid = pv.to_string();
        pvrwv.push(ass);
        pvrwm.insert(pv.to_string(), i);
    }
    let mut cn = 0;
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
        cn += 1;
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
    println!("cnt: {cn}");
    let mut rows2 = Vec::<WebTempRow>::new();
    for (i, sr) in pvrwv.iter().enumerate() {
        let mut evgr = get_scurv();
        for ev in evgr.iter_mut() {
            *ev *= evcs2[i] * EV_AT_2050;
            *ev *= 0.007;
            *ev *= 365.0 * EV_HR_DAY;
            *ev *= EV_MWH_BAHT;
        }
        let rw = WebTempRow {
            sbid: sr.sbid.to_string(),
            pvid: sr.pvid.to_string(),
            note: "".to_string(),
            evrt: evcs2[i],
            evgr,
        };
        rows2.push(rw);
    }

    ///////////////////////////////////////////////////
    let mut rows = Vec::<WebTempRow>::new();
    let mut evcs = vec![0f32; sbrw.len()];
    for (i, sr) in sbrw.iter().enumerate() {
        evcs[i] = sr.v[VarType::NoHmChgEvTr as usize].v / evs;
    }
    for (i, sr) in sbrw.iter().enumerate() {
        let note = format!("{}", sr.v[VarType::TakeNote as usize].v);
        if note != "1" {
            continue;
        }
        if sr.sbid == "SQB" {
            continue;
        }
        let evrt = evcs[i];

        //use fast_math;
        let mut evgr = get_scurv();
        for ev in evgr.iter_mut() {
            *ev *= evrt * EV_AT_2050;
            *ev *= 0.007;
            *ev *= 365.0 * EV_HR_DAY;
            *ev *= EV_MWH_BAHT;
        }
        let rw = WebTempRow {
            sbid: sr.sbid.to_string(),
            pvid: sr.pvid.to_string(),
            note,
            evrt,
            evgr,
        };
        rows.push(rw);
    }

    WebTemp {
        name: "Year Baht for Charge EV - p06".to_string(),
        rows: rows2,
    }
}
