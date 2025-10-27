/*
pub fn p13_cnl_trs(ar: &str) -> Result<Vec<CnlTrans>, Box<dyn Error>> {
pub fn p13_mt_bil(ar: &str) -> Result<Vec<MeterBill>, Box<dyn Error>> {
pub fn p13_cnl_mt(ar: &str) -> Result<Vec<CnlData>, Box<dyn Error>> {
pub fn p13_mt2bil(ar: &str) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
pub fn p13_nodes(ar: &str) -> Result<HashMap<u64, NodeInfo>, Box<dyn Error>> {
pub fn p13_gis_db(ar: &str, ly: &str) -> Result<Vec<HashMap<String, DbfData>>, Box<dyn Error>> {
*/
use std::error::Error;

use regex::Regex;
use sglab02_lib::sg::gis1::ar_list;
use sglab02_lib::sg::mvline::latlong_utm;
use sglab02_lib::sg::mvline::utm_latlong;
use sglab02_lib::sg::prc5::sub_inf;
use sglib03::subtype::SUB_TYPES;
use sglib04::aoj::sub_latlong_adjust;
use sglib04::geo1::find_node;
use sglib04::geo1::n1d_2_utm;
use sglib04::geo1::utm_2_n1d;
//use sglib04::geo2::SubFeedTrans;
use bincode::{Decode, Encode};
use sglib04::ld1::p13_cnl_mt;
use sglib04::ld1::p13_cnl_trs;
use sglib04::ld1::p13_gis_db;
use sglib04::ld1::p13_mt2bil;
use sglib04::ld1::p13_mt_bil;
use sglib04::ld1::p13_nodes;
use std::collections::HashMap;

#[derive(Encode, Decode, PartialEq, Debug, Clone, Default)]
pub struct SubFeedTrans {
    pub sbid: String,
    pub conf: String,
    pub n1d_s: u64,
    pub n1d_f: u64,
    //pub feed: HashMap<String, Vec<String>>,
    pub feed: HashMap<String, Vec<usize>>,
}

pub fn p04_form_sub() -> Result<(), Box<dyn Error>> {
    println!("form sub");
    let adjxy = sub_latlong_adjust();

    let mut sb_nid_cf = HashMap::<String, (u64, String)>::new();
    let re = Regex::new(r"q=([0-9]+\.[0-9]+),([0-9]+\.[0-9]+)").unwrap();
    for (sb, cf, gm) in &SUB_TYPES {
        if let Some(cap) = re.captures_iter(gm).next() {
            let x = &cap[1];
            let y = &cap[2];
            let mut xx = x.parse::<f32>().unwrap();
            let mut yy = y.parse::<f32>().unwrap();
            let sbid = sb.to_string();
            if let Some((x1, y1)) = adjxy.get(&sbid) {
                xx += x1;
                yy += y1;
            }
            let (sb_x, sb_y) = latlong_utm(xx, yy);
            let n1d = utm_2_n1d(sb_x, sb_y);
            sb_nid_cf.insert(sb.to_string(), (n1d, cf.to_string()));
        }
    }

    println!("sb_nid_cf:{}", sb_nid_cf.len());

    let sf = sub_inf();
    println!("sf: {}", sf.len());
    for ar in ar_list() {
        let ly = "DS_MVConductor";
        println!("{ar}");
        let ctrs = p13_cnl_trs(ar)?;
        println!("   ctr:{}", ctrs.len());
        let bil = p13_mt_bil(ar)?;
        println!("   bil:{}", bil.len());
        let fcmt = p13_cnl_mt(ar)?;
        println!("   met:{}", fcmt.len());
        let fm2b = p13_mt2bil(ar)?;
        println!("   m2b:{}", fm2b.len());
        let fnds = p13_nodes(ar)?;
        println!("   nds:{}", fnds.len());
        let gdb = p13_gis_db(ar, ly)?;
        println!("   gdb:{}", gdb.len());

        let mut n1ds = vec![];
        let mut eqnds = vec![];
        //let mut mvnds = HashMap::<String, Vec<u64>>::new();
        for (n1d, ndif) in &fnds {
            n1ds.push(*n1d);
            for gn in &ndif.nodes {
                if gn.ly == ly {
                    eqnds.push(*n1d);
                }
            }
        }
        n1ds.sort();
        eqnds.sort();
        println!("\n  prepare nodes data Breaker: {}", eqnds.len());

        let mut nosbh = HashMap::<String, usize>::new();
        let mut bcn = 0;
        let mut sb_fd_tr_hm = HashMap::<String, SubFeedTrans>::new();
        for (ti, ctr) in ctrs.iter().enumerate() {
            //let trid = ctr.trid.to_string();
            let cmt = &fcmt[ctr.ix];
            if let Some(fid) = &cmt.tr_fid {
                let sbid = &fid[0..3];
                let sbid = sbid.to_string();
                let Some(sf) = sf.get(&sbid) else {
                    continue;
                };
                if sf.arid != ar {
                    continue;
                }
                if let Some((nd, cf)) = sb_nid_cf.get(&sbid) {
                    let fid = fid.to_string();
                    if let Some(sb_fd_tr) = sb_fd_tr_hm.get_mut(&sbid) {
                        if let Some(fd_tr) = sb_fd_tr.feed.get_mut(&fid) {
                            fd_tr.push(ti);
                            //fd_tr.push(trid.to_string());
                        } else {
                            sb_fd_tr.feed.insert(fid.to_string(), vec![ti]);
                            //.insert(fid.to_string(), vec![trid.to_string()]);
                        }
                    } else {
                        let n1d_s = *nd;
                        //println!("SB {sbid} {n1d_s} - hvnds:{}", hvnds.len());
                        let n1d_f = find_node(n1d_s, &eqnds);
                        let (sx, sy) = n1d_2_utm(n1d_s);
                        let (fx, fy) = n1d_2_utm(n1d_f);
                        let (dx, dy) = ((sx - fx).abs(), (sy - fy).abs());
                        let conf = cf.to_string();
                        if dx + dy > 200.0 {
                            let (st, sl) = utm_latlong(sx, sy);
                            let (ft, fl) = utm_latlong(fx, fy);
                            println!(
                                "{ar}-{sbid} == ({dx:.2},{dy:.2}) => surv:{st},{sl} find:{ft},{fl}"
                            );
                        }
                        let feed = HashMap::from([(fid.to_string(), vec![ti])]);
                        //HashMap::from([(fid.to_string(), vec![trid.to_string()])]);
                        let sb_fd_tr = SubFeedTrans {
                            sbid: sbid.to_string(),
                            n1d_s,
                            n1d_f,
                            conf,
                            feed,
                        };
                        sb_fd_tr_hm.insert(sbid.to_string(), sb_fd_tr);
                    }
                } else if let Some(cn) = nosbh.get_mut(&sbid) {
                    *cn += 1;
                } else {
                    nosbh.insert(sbid.to_string(), 1);
                }
            }
            //println!("{ar} n1d:{} mts:{}", ctr.n1d, ctr.mts.len());
            for mi in &ctr.mts {
                let _mt = &fcmt[*mi];
                let m2b = &fm2b[*mi];
                bcn += m2b.len();
                //println!("  mt: {:?} bil:{}", mt.mt_pea, m2b.len());
            }
        }
        println!("NO found {}", nosbh.len());

        let mut sb_fd_tr = Vec::<SubFeedTrans>::new();
        //let mut keys = sb_fd_tr_hm.keys().to_vec();
        //let mut keys: Vec<String> = sb_fd_tr_hm.iter().map(|(k, _)| k.to_string()).collect();
        let mut keys: Vec<String> = sb_fd_tr_hm.keys().map(|k| k.to_string()).collect();
        keys.sort();
        for k in keys {
            let sub = sb_fd_tr_hm.get(&k).unwrap().clone();
            sb_fd_tr.push(sub);
        }
        let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p11_{ar}_sb_fd_tr.bin");
        let bin: Vec<u8> = bincode::encode_to_vec(&sb_fd_tr, bincode::config::standard()).unwrap();
        println!(" write to {fnm} - {}", sb_fd_tr.len());
        std::fs::write(fnm, bin).unwrap();

        println!("  bcn:{bcn}");
    }
    Ok(())
}
