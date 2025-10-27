//use crate::p02::DAY_VAL_PNTS;
//use crate::p02::ONE_DAY_SEC;
//use crate::p02::ONE_TIK_SEC;
//use bincode::{Decode, Encode};
//use chrono::{Datelike, NaiveDateTime};
use crate::p02::FeederLoadProf;
use crate::p02::SubLoadProf;
use iterstats::Iterstats;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

pub fn p07_lp_pro(yr: &str) -> Result<(), Box<dyn Error>> {
    let year = yr.parse::<u32>().unwrap();
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_read_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (mut lps, _): (HashMap<String, FeederLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    println!("lps: {}", lps.len());
    let mut subh = HashMap::<String, SubLoadProf>::new();
    let (mut c1, mut c2, mut c3) = (0, 0, 0);
    for lp in lps.values_mut() {
        for dlp in lp.days.iter_mut().flatten() {
            let g = dlp.val.into_iter().flatten().collect::<Vec<_>>();
            let m = g.iter().mean();
            let s = g.iter().stddev();
            for ov in dlp.val.iter_mut() {
                c3 += 1;
                if let Some(v) = &ov {
                    if *v > 10f32 || *v < -10f32 {
                        *ov = None;
                        c1 += 1;
                    } else {
                        let z = (*v - m) / s;
                        if z.abs() > 2.5 {
                            *ov = None;
                            c2 += 1;
                        }
                    }
                }
            }
            dlp.cnt = 0;
            for va in &dlp.val {
                if va.is_some() {
                    dlp.cnt += 1;
                }
            }
        }
        let sub = lp.sub.to_string();
        subh.entry(sub.to_string()).or_insert_with(|| SubLoadProf {
            year,
            sub,
            feeds: Vec::<_>::new(),
            fdldp: HashMap::<_, _>::new(),
            ldpf: None,
        });
        let Some(sub) = subh.get_mut(&lp.sub) else {
            continue;
        };
        sub.fdldp.insert(lp.feed.to_string(), lp.clone());
    }
    println!("{c3} {c1} {c2}");

    for sub in subh.values_mut() {
        //let mut keys: Vec<_> = sub.fdldp.clone().into_iter().map(|(k, _)| k).collect();
        let mut keys: Vec<_> = sub.fdldp.clone().into_keys().collect();
        keys.sort();
        sub.feeds = keys;
    }
    let re = Regex::new(r"[0-1][0-9][VW][BR]01$").unwrap();
    let mut cn = 0;
    for (_sb, sub) in &mut subh {
        //for sb in &keys {
        //let sb = sb.to_string();
        //let sub: &mut SubLoadProf = subh.get_mut(&sb).unwrap();
        //let sub = subh.get(&sb).unwrap();
        //println!("sub: {}", sub.sub);
        let mut sbldpf = None;
        let mut fds = Vec::<String>::new();
        let mut fc1 = 0;
        let mut fc2 = 0;
        let mut ffs = vec![];
        for f in &sub.feeds {
            ffs.push(f.to_string());
            if !re.is_match(f) {
                continue;
            }
            fc1 += 1;
            fds.push(f.to_string());
            let flp = sub.fdldp.get(f).unwrap();
            if sbldpf.is_none() {
                fc2 += 1;
                let mut slp = flp.clone();
                slp.feed = "".to_string();
                for day in slp.days.iter_mut() {
                    *day = None;
                }
                sbldpf = Some(slp);
            }
            /*
            let _cn = flp
                .days
                .clone()
                .into_iter()
                .flatten()
                .filter(|d| d.cnt == d.val.len())
                .count();
            */
        }
        if sbldpf.is_none() {
            cn += 1;
            println!("{cn}.sbldpf non in {_sb} {fc1} {fc2} = {ffs:?}");
        }
        // load profile for substation
        if let Some(sbldpf) = &mut sbldpf {
            for (di, day) in sbldpf.days.iter_mut().enumerate() {
                for f in &fds {
                    let flp = sub.fdldp.get(f).unwrap();
                    if di >= flp.days.len() {
                        continue;
                    }
                    /*
                    if sb == "CMC" && di == 126 {
                        println!("CMC {f} {di:03}");
                    }
                    */
                    if let Some(fdy) = &flp.days[di] {
                        if let Some(day0) = day {
                            for (mut sv, fv) in day0.val.iter_mut().zip(fdy.val.iter()) {
                                if fv.is_none() {
                                } else if let (Some(sv), Some(fv)) = (&mut sv, &fv) {
                                    *sv += *fv;
                                } else if let Some(fv) = &fv {
                                    *sv = Some(*fv);
                                }
                            }
                        } else {
                            *day = Some(fdy.clone());
                        }
                    } // end if load exists
                } // end feed loop
            } // end sub day loop
        } // end sub load profile
        sub.ldpf = sbldpf;
    }
    // save to file
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
    let bin: Vec<u8> = bincode::encode_to_vec(&subh, bincode::config::standard()).unwrap();
    std::fs::write(&fnm, bin).unwrap();
    println!("write to {fnm}");

    Ok(())
}
