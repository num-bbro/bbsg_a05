use crate::dcl::ProcEngine;
use crate::dcl::*;
use sglib04::geo3::GisZone;
use sglib04::ld1::get_sele_subs;
use std::collections::HashSet;
use std::sync::OnceLock;

pub fn z2o(v: f32) -> f32 {
    if v == 0f32 {
        1f32
    } else {
        v
    }
}

pub fn get_scurv() -> Vec<f32> {
    let mut curv = Vec::<f32>::new();
    for y in SSHOW_YEAR_BEG..=SSHOW_YEAR_END {
        let a = (y - SCURV_YEAR_BEG) as f32;
        let b = a - 14f32;
        //let c = b * 0.3f32;
        let c = b * 0.41f32;
        //let d = c + 0.0f32;
        let d = c + 1.205f32;
        let d = -d;
        let e = d.exp();
        let f = 1f32 / (1f32 + e);
        //let g = f.powf(1f32);
        let g = f.powf(1.1f32);
        curv.push(g);
    }
    curv
}

pub fn get_scurv_re() -> Vec<f32> {
    let mut curv = Vec::<f32>::new();
    for y in SSHOW_YEAR_BEG..=SSHOW_YEAR_END {
        let a = (y - RE_SCURV_BEG) as f32;
        let b = a - 14f32;
        let c = b * 0.3f32;
        //let c = b * 0.41f32;
        let d = c + 0.0f32;
        //let d = c + 1.205f32;
        let d = -d;
        let e = d.exp();
        let f = 1f32 / (1f32 + e);
        let g = f.powf(1f32);
        //let g = f.powf(1.1f32);
        curv.push(g);
    }
    curv
}

pub static SBRW00: OnceLock<Vec<PeaAssVar>> = OnceLock::new();
pub fn get_sbrw00() -> &'static Vec<PeaAssVar> {
    SBRW00.get_or_init(sbrw00_init)
}
fn sbrw00_init() -> Vec<PeaAssVar> {
    //let dnm = "/mnt/e/CHMBACK/pea-data/c01_pea";
    let buf = std::fs::read(format!("{DNM}/000-sbrw.bin")).unwrap();
    let (ass, _): (Vec<PeaAssVar>, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    ass
}

pub static PEA00: OnceLock<Pea> = OnceLock::new();
pub fn get_pea00() -> &'static Pea {
    PEA00.get_or_init(pea00_init)
}
fn pea00_init() -> Pea {
    //let dnm = "/mnt/e/CHMBACK/pea-data/c01_pea";
    let buf = std::fs::read(format!("{DNM}/000_pea.bin")).unwrap();
    let (pea, _): (Pea, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    println!("pea: {}", pea.aream.len());
    pea
}

pub static PVRW00: OnceLock<Vec<PeaAssVar>> = OnceLock::new();
pub fn get_pvrw00() -> &'static Vec<PeaAssVar> {
    PVRW00.get_or_init(pvrw00_init)
}
fn pvrw00_init() -> Vec<PeaAssVar> {
    //let dnm = "/mnt/e/CHMBACK/pea-data/c01_pea";
    let buf = std::fs::read(format!("{DNM}/000-pvrw.bin")).unwrap();
    let (ass, _): (Vec<PeaAssVar>, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    println!("pv: {}", ass.len());
    ass
}

pub fn mon_kwh_2_kw(kwh: f32) -> f32 {
    kwh / (24f32 * 30f32) * 3f32
}

pub fn trf_kva_2_kw(kva: f32) -> f32 {
    kva * 0.9f32 * 0.85f32
}

pub fn zone_factor(zn: &GisZone) -> f32 {
    if zn.zncd.is_none() {
        return 10.0;
    }
    match zn.zncd.clone().unwrap().as_str() {
        "21" => 12.0,
        "22" => 21.0,
        "23" => 20.0,
        "24" => 19.0,
        "25" => 18.0,
        "11" => 17.0,
        "12" => 16.0,
        "13" => 15.0,
        "14" => 14.0,
        "31" => 13.0,
        "41" => 12.0,
        "42" => 11.0,
        "51" => 10.0,
        _ => 10.0,
    }
}

pub fn get_tr_zone(ti: usize, eg: &ProcEngine) -> f32 {
    let zn = &eg.zntr[ti];
    if zn.is_empty() {
        return 10.0;
    }
    zone_factor(&eg.zons[zn[0]])
}

pub fn get_tr_den(ti: usize, eg: &ProcEngine) -> f32 {
    let am = &eg.amtr[ti];
    let mu = &eg.mutr[ti];
    let mut dns = None;
    if !am.is_empty() {
        dns = Some(&eg.amps[am[0]]);
    }
    if !mu.is_empty() {
        dns = Some(&eg.muni[mu[0]]);
    }
    if let Some(dns) = dns {
        dns.dens
    } else {
        let mut dn = eg.amps[0].dens;
        for ad in &eg.amps {
            dn = dn.min(ad.dens);
        }
        dn
    }
}

pub fn get_tr_sorf(ti: usize, eg: &ProcEngine) -> f32 {
    if eg.sotr.len() > ti {
        let mut so = 0.0;
        for sr in &eg.sotr[ti] {
            if let Some(p) = eg.sola[*sr].pow {
                so += p;
            }
        }
        return so;
    }
    0.0
}

pub fn get_tr_volta(ti: usize, eg: &ProcEngine) -> (f32, f32) {
    let vos = &eg.votr[ti];
    if let Some(vi) = vos.iter().next() {
        let vo = &eg.vols[*vi];
        let mut pow = 0.0;
        for (pw, no) in &vo.chgr {
            pow += (pw * no) as f32;
        }
        let mut sel = 0.0;
        //println!("VOL: {:?}", vo.stno);
        for (_ym, am) in &vo.sell {
            sel += am;
            //println!("  {ym} {am}");
        }
        return (pow, sel);
    }
    /*
    for vi in vos {
        let vo = &eg.vols[*vi];
        let mut pow = 0.0;
        for (pw, no) in &vo.chgr {
            pow += (pw * no) as f32;
        }
        let mut sel = 0.0;
        //println!("VOL: {:?}", vo.stno);
        for (_ym, am) in &vo.sell {
            sel += am;
            //println!("  {ym} {am}");
        }
        return (pow, sel);
    }
    */
    (0.0, 0.0)
}

pub fn p01_chk() -> HashSet<String> {
    let subs = get_sele_subs();
    let mut subhs = HashSet::<String>::new();
    for s in subs {
        subhs.insert(s);
    }
    //println!("sele sub {}", subhs.len());
    subhs
}

pub const RE_SCURV_BEG: usize = 2018;
pub const EV_SCURV_BEG: usize = 2021;
pub const SCURV_WIND_BEG: usize = 2026;
pub const SCURV_WIND_END: usize = 2040;

pub fn ev_scurv() -> Vec<f32> {
    let mut curv = Vec::<f32>::new();
    for y in SCURV_WIND_BEG..=SCURV_WIND_END {
        let a = (y - EV_SCURV_BEG) as f32;
        let b = a - 14f32;
        //let c = b * 0.3f32;
        let c = b * 0.41f32;
        //let d = c + 0.0f32;
        let d = c + 1.205f32;
        let d = -d;
        let e = d.exp();
        let f = 1f32 / (1f32 + e);
        //let g = f.powf(1f32);
        let g = f.powf(1.1f32);
        curv.push(g);
    }
    curv
}

pub fn re_scurv() -> Vec<f32> {
    let mut curv = Vec::<f32>::new();
    for y in SCURV_WIND_BEG..=SCURV_WIND_END {
        let a = (y - RE_SCURV_BEG) as f32;
        let b = a - 14f32;
        let c = b * 0.3f32;
        //let c = b * 0.41f32;
        let d = c + 0.0f32;
        //let d = c + 1.205f32;
        let d = -d;
        let e = d.exp();
        let f = 1f32 / (1f32 + e);
        let g = f.powf(1f32);
        //let g = f.powf(1.1f32);
        curv.push(g);
    }
    curv
}

pub fn et_scurv() -> Vec<f32> {
    let mut curv = Vec::<f32>::new();
    for y in SCURV_WIND_BEG..=SCURV_WIND_END {
        let a = (y - EV_SCURV_BEG) as f32;
        let b = a - 14f32;
        //let c = b * 0.3f32;
        let c = b * 0.41f32;
        //let d = c + 0.0f32;
        let d = c + 1.205f32;
        let d = -d;
        let e = d.exp();
        let f = 1f32 / (1f32 + e);
        //let g = f.powf(1f32);
        let g = f.powf(1.1f32);
        curv.push(g);
    }
    curv
}

pub fn eb_scurv() -> Vec<f32> {
    let mut curv = Vec::<f32>::new();
    for y in SCURV_WIND_BEG..=SCURV_WIND_END {
        let a = (y - EV_SCURV_BEG) as f32;
        let b = a - 14f32;
        //let c = b * 0.3f32;
        let c = b * 0.41f32;
        //let d = c + 0.0f32;
        let d = c + 1.205f32;
        let d = -d;
        let e = d.exp();
        let f = 1f32 / (1f32 + e);
        //let g = f.powf(1f32);
        let g = f.powf(1.1f32);
        curv.push(g);
    }
    curv
}

use bincode::{Decode, Encode};
use calamine::open_workbook;
use calamine::Reader;
use calamine::Xlsx;
use std::path::PathBuf;

#[derive(Debug, Encode, Decode)]
pub struct XlsSheet {
    pub path: String,
    pub name: String,
    pub shnm: String,
    pub rcnt: usize,
    pub ccnt: usize,
    pub data: Vec<Vec<String>>,
}

pub fn load_xlsx(flst: &Vec<&str>) -> Result<Vec<XlsSheet>, Box<dyn std::error::Error>> {
    let mut xlsv = Vec::<XlsSheet>::new();
    for fl in flst {
        let pt = PathBuf::from(fl);
        let mut excel: Xlsx<_> = open_workbook(fl)?;
        let ff = pt.file_name().unwrap().to_str().unwrap();
        let sheets = excel.sheet_names().to_owned();
        for sh in &sheets {
            if let Ok(range) = excel.worksheet_range(sh) {
                let path = fl.to_string();
                let name = ff.to_string();
                let shnm = sh.to_string();
                let rcnt = range.get_size().0;
                let ccnt = range.get_size().1;
                let mut data = Vec::<Vec<String>>::new();

                for row in range.rows() {
                    let mut rw = Vec::<String>::new();
                    for c in row {
                        rw.push(c.to_string());
                    }
                    data.push(rw);
                }

                let xls_info = XlsSheet {
                    path,
                    name,
                    shnm,
                    rcnt,
                    ccnt,
                    data,
                };
                xlsv.push(xls_info);
            }
        }
    }
    Ok(xlsv)
}

use crate::p02::FeederLoadProf;
use crate::p02::SubLoadProf;
use std::collections::HashMap;

pub static LP24FD: OnceLock<HashMap<String, FeederLoadProf>> = OnceLock::new();
pub fn lp24_fd() -> &'static HashMap<String, FeederLoadProf> {
    LP24FD.get_or_init(lp24_fd_init)
}
fn lp24_fd_init() -> HashMap<String, FeederLoadProf> {
    let yr = 2024;
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_read_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (lps, _): (HashMap<String, FeederLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    lps
}

pub static LP24SB: OnceLock<HashMap<String, SubLoadProf>> = OnceLock::new();
pub fn lp24_sb() -> &'static HashMap<String, SubLoadProf> {
    LP24SB.get_or_init(lp24_sb_init)
}
fn lp24_sb_init() -> HashMap<String, SubLoadProf> {
    let yr = 2024;
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (subh, _): (HashMap<String, SubLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    subh
}

use std::error::Error;

pub fn calc_fd_lp(lp: &FeederLoadProf) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
    let mut po_lst = Vec::<(f32, usize)>::new();
    let mut ne_lst = Vec::<(f32, usize)>::new();
    for (di, dlp) in lp.days.iter().enumerate() {
        if let Some(dlp) = dlp {
            let mut po_pw = 0.0;
            let mut ne_pw = 0.0;
            for v in dlp.val.into_iter().flatten() {
                if v > 0.0 {
                    po_pw += v;
                }
                if v < 0.0 {
                    ne_pw += -v;
                }
            }
            if po_pw > 0.0 {
                po_lst.push((po_pw, di));
            }
            if ne_pw > 0.0 {
                ne_lst.push((ne_pw, di));
            }
        }
    }
    po_lst.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut po_lst2 = Vec::<usize>::new();
    for (_v, i) in po_lst {
        if let Some(dlp) = &lp.days[i]
            && !dlp.is_valid()
        {
            break;
        }
        po_lst2.push(i);
    }
    ne_lst.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut ne_lst2 = Vec::<usize>::new();
    for (_v, i) in ne_lst {
        if let Some(dlp) = &lp.days[i]
            && !dlp.is_valid()
        {
            break;
        }
        ne_lst2.push(i);
    }
    Ok((po_lst2, ne_lst2))
}

use crate::p03::p03_draw_sub_av;

pub fn test_lp24() -> Result<(), Box<dyn Error>> {
    let sblp = lp24_sb();
    println!("lp24 sb {}", sblp.len());
    for (k, sb) in sblp {
        println!("sub - {k}");
        for (fk, ldpf) in &sb.fdldp {
            let (pov, nev) = calc_fd_lp(ldpf)?;
            let _po_lp = p03_draw_sub_av(ldpf, &pov, "PO")?;
            let _ne_lp = p03_draw_sub_av(ldpf, &nev, "NE")?;
            println!("  {fk} po:{} ne:{}", pov.len(), nev.len());
        }
    }
    Ok(())
}
