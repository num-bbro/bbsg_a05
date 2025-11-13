use crate::p02::DayLoadProf;
use crate::p02::DrawLoadProf;
use crate::p02::FeederLoadProf;
use crate::p02::LoadProf;
use crate::p02::SubLoadProf;
use crate::p02::DAY_VAL_PNTS;
use bincode::{Decode, Encode};
use std::collections::HashMap;
use std::error::Error;
pub const LOAD_PROF_MIN_DAYS: usize = 5;
pub const LOAD_PROF_MAX_DAYS: usize = 30;
pub const LOAD_PROF_MIN_HOUR: usize = 18;

impl DayLoadProf {
    pub fn is_valid(&self) -> bool {
        let mut hrls = vec![];
        for (vi, v) in self.val.iter().enumerate() {
            if v.is_some() {
                let hr = vi / 4;
                if !hrls.contains(&hr) {
                    hrls.push(vi);
                }
            }
        }
        hrls.len() > LOAD_PROF_MIN_HOUR
    }
}

pub fn p03_calc_lp_1(lp: &FeederLoadProf) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
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
        if let Some(dlp) = &lp.days[i] {
            if !dlp.is_valid() {
                break;
            }
        }
        po_lst2.push(i);
    }
    ne_lst.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut ne_lst2 = Vec::<usize>::new();
    for (_v, i) in ne_lst {
        if let Some(dlp) = &lp.days[i] {
            if !dlp.is_valid() {
                break;
            }
        }
        ne_lst2.push(i);
    }
    Ok((po_lst2, ne_lst2))
}

#[derive(Encode, Decode, PartialEq, Debug, Clone, Default)]
pub struct LoadProfRepr {
    pub val: Option<[Option<f32>; DAY_VAL_PNTS]>,
    pub rep: Option<Vec<usize>>,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone, Default)]
pub struct SubLoadProfRepr {
    pub sub: String,
    pub year: String,
    pub pos_rep: LoadProfRepr,
    pub neg_rep: LoadProfRepr,
}

pub fn p03_draw_sub_av(
    ldpf: &FeederLoadProf,
    po: &[usize],
    pn: &str,
) -> Result<LoadProfRepr, Box<dyn Error>> {
    let mut lpre = LoadProfRepr::default();
    let s = ldpf.sub.to_string();
    let f = ldpf.feed.to_string();
    let yr = ldpf.year.to_string();
    if po.len() >= LOAD_PROF_MIN_DAYS {
        let mut avls = Vec::<usize>::new();
        for ii in po.iter().take(LOAD_PROF_MAX_DAYS) {
            avls.push(*ii);
        }
        let mut avdlp = ldpf.days[avls[0]].clone();
        for ii in &avls[1..] {
            let fdlp = &ldpf.days[*ii];
            if let (Some(slp), Some(flp)) = (&mut avdlp, fdlp) {
                for (sva, fva) in slp.val.iter_mut().zip(flp.val.iter()) {
                    if let (None, Some(fva)) = (&sva, &fva) {
                        *sva = Some(*fva);
                    } else if let (Some(sv), Some(fv)) = (&sva, &fva) {
                        *sva = Some(*sv + *fv);
                    }
                }
            }
        }
        let ln = avls.len() as f32;
        if let Some(slp) = &mut avdlp {
            for sva in slp.val.iter_mut() {
                if let Some(sv) = &sva {
                    *sva = Some(*sv / ln);
                }
            }
            let val = slp.val;
            lpre.val = Some(slp.val);
            lpre.rep = Some(avls.clone());
            let dnm = format!("/mnt/e/CHMBACK/pea-data/sbdrlp/{yr}/{s}");
            std::fs::create_dir_all(&dnm)?;
            let fnm = format!("{dnm}/{s}-{f}-{pn}.jpg");
            //println!(" -> {fnm}");
            let sz = (400, 300);
            let rf = Vec::<(String, f32)>::new();

            let drlp = LoadProf {
                lb1: s.to_string(),
                lb2: yr.to_string(),
                fnm,
                val,
                sz,
                rf,
            };
            drlp.draw_prof()?;
        }
    }
    Ok(lpre)
}
//use crate::p03::SubLoadProfRepr;

pub fn p03_load_lp(yr: &str) -> HashMap<String, SubLoadProfRepr> {
    //let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p03_lp_repr_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (subh, _): (HashMap<String, SubLoadProfRepr>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    subh
}

pub fn p03_calc_lp2(yr: &str, s: &str) -> Result<(), Box<dyn Error>> {
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (subh, _): (HashMap<String, SubLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();

    let mut p03_lp_repr = HashMap::<String, SubLoadProfRepr>::new();
    //for (s, sub) in subh {
    {
        let sub = subh.get(s).unwrap();
        if let Some(ldpf) = &sub.ldpf {
            print!("{s} ");
            let (po, ne) = p03_calc_lp_1(ldpf)?;
            let sbre = SubLoadProfRepr {
                sub: s.to_string(),
                year: yr.to_string(),
                pos_rep: p03_draw_sub_av(ldpf, &po, "PO")?,
                neg_rep: p03_draw_sub_av(ldpf, &ne, "NE")?,
            };
            p03_lp_repr.insert(s.to_string(), sbre);

            print!("  p:{} [", po.len());
            for (pi, ii) in po.iter().enumerate() {
                if pi >= LOAD_PROF_MIN_DAYS {
                    break;
                }
                if pi > 0 {
                    print!(",");
                }
                print!("{ii}");
            }
            print!("] n:{} [", ne.len());
            for (ni, ii) in ne.iter().enumerate() {
                if ni >= LOAD_PROF_MIN_DAYS {
                    break;
                }
                if ni > 0 {
                    print!(",");
                }
                print!("{ii}");
            }
            print!("]");
            println!();
        }
        for ldpf in sub.fdldp.values() {
            let f = &ldpf.feed;
            //p03_calc_lp_1(ldpf)?;
            let (po, ne) = p03_calc_lp_1(ldpf)?;
            let sbre = SubLoadProfRepr {
                sub: s.to_string(),
                year: yr.to_string(),
                pos_rep: p03_draw_sub_av(ldpf, &po, "PO")?,
                neg_rep: p03_draw_sub_av(ldpf, &ne, "NE")?,
            };
            p03_lp_repr.insert(format!("{s}-{f}"), sbre);
        }
    }
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p03_lp_repr_{yr}.bin");
    println!("write pre {} to {fnm}", p03_lp_repr.len());
    let bin: Vec<u8> = bincode::encode_to_vec(&p03_lp_repr, bincode::config::standard()).unwrap();
    std::fs::write(fnm, bin).unwrap();
    Ok(())
}

pub fn p03_calc_lp_3(lp: &FeederLoadProf) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
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
        if let Some(dlp) = &lp.days[i] {
            if !dlp.is_valid() {
                break;
            }
        }
        po_lst2.push(i);
    }
    ne_lst.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut ne_lst2 = Vec::<usize>::new();
    for (_v, i) in ne_lst {
        if let Some(dlp) = &lp.days[i] {
            if !dlp.is_valid() {
                break;
            }
        }
        ne_lst2.push(i);
    }
    Ok((po_lst2, ne_lst2))
}

pub fn p03_calc_lp3(yr: &str) -> Result<(), Box<dyn Error>> {
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (subh, _): (HashMap<String, SubLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    let mut p03_lp_repr = HashMap::<String, SubLoadProfRepr>::new();
    let mut dy_cn = HashMap::<usize, usize>::new();
    for (s, sub) in subh {
        if let Some(ldpf) = &sub.ldpf {
            print!("{s} ");
            let (po, ne) = p03_calc_lp_3(ldpf)?;
            let sbre = SubLoadProfRepr {
                sub: s.to_string(),
                year: yr.to_string(),
                pos_rep: p03_draw_sub_av(ldpf, &po, "PO")?,
                neg_rep: p03_draw_sub_av(ldpf, &ne, "NE")?,
            };
            p03_lp_repr.insert(s.to_string(), sbre);

            print!("  p:{} [", po.len());
            for (pi, ii) in po.iter().enumerate() {
                if pi >= LOAD_PROF_MIN_DAYS {
                    break;
                }
                if let Some(cn) = dy_cn.get_mut(ii) {
                    *cn += 1;
                } else {
                    dy_cn.insert(*ii, 1);
                }
                if pi > 0 {
                    print!(",");
                }
                print!("{ii}");
            }
            print!("] n:{} [", ne.len());
            for (ni, ii) in ne.iter().enumerate() {
                if ni >= LOAD_PROF_MIN_DAYS {
                    break;
                }
                if let Some(cn) = dy_cn.get_mut(ii) {
                    *cn += 1;
                } else {
                    dy_cn.insert(*ii, 1);
                }
                if ni > 0 {
                    print!(",");
                }
                print!("{ii}");
            }
            print!("]");
            println!();
        }
        for ldpf in sub.fdldp.values() {
            let f = &ldpf.feed;
            //p03_calc_lp_1(ldpf)?;
            let (po, ne) = p03_calc_lp_3(ldpf)?;
            let sbre = SubLoadProfRepr {
                sub: s.to_string(),
                year: yr.to_string(),
                pos_rep: p03_draw_sub_av(ldpf, &po, "PO")?,
                neg_rep: p03_draw_sub_av(ldpf, &ne, "NE")?,
            };
            p03_lp_repr.insert(format!("{s}-{f}"), sbre);
        }
    }
    //println!("{dy_cn:?}");
    let mut dycn = dy_cn.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
    dycn.sort_by(|a, b| b.1.cmp(&a.1));
    for (d, c) in dycn {
        println!("{d} - {c}");
    }
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p03_lp_repr_{yr}.bin");
    println!("write pre {} to {fnm}", p03_lp_repr.len());
    let bin: Vec<u8> = bincode::encode_to_vec(&p03_lp_repr, bincode::config::standard()).unwrap();
    std::fs::write(fnm, bin).unwrap();
    Ok(())
}

pub fn p03_calc_lp(yr: &str) -> Result<(), Box<dyn Error>> {
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (subh, _): (HashMap<String, SubLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    let mut p03_lp_repr = HashMap::<String, SubLoadProfRepr>::new();
    for (s, sub) in subh {
        if let Some(ldpf) = &sub.ldpf {
            print!("{s} ");
            let (po, ne) = p03_calc_lp_1(ldpf)?;
            let sbre = SubLoadProfRepr {
                sub: s.to_string(),
                year: yr.to_string(),
                pos_rep: p03_draw_sub_av(ldpf, &po, "PO")?,
                neg_rep: p03_draw_sub_av(ldpf, &ne, "NE")?,
            };
            p03_lp_repr.insert(s.to_string(), sbre);

            print!("  p:{} [", po.len());
            for (pi, ii) in po.iter().enumerate() {
                if pi >= LOAD_PROF_MIN_DAYS {
                    break;
                }
                if pi > 0 {
                    print!(",");
                }
                print!("{ii}");
            }
            print!("] n:{} [", ne.len());
            for (ni, ii) in ne.iter().enumerate() {
                if ni >= LOAD_PROF_MIN_DAYS {
                    break;
                }
                if ni > 0 {
                    print!(",");
                }
                print!("{ii}");
            }
            print!("]");
            println!();
        }
        for ldpf in sub.fdldp.values() {
            let f = &ldpf.feed;
            //p03_calc_lp_1(ldpf)?;
            let (po, ne) = p03_calc_lp_1(ldpf)?;
            let sbre = SubLoadProfRepr {
                sub: s.to_string(),
                year: yr.to_string(),
                pos_rep: p03_draw_sub_av(ldpf, &po, "PO")?,
                neg_rep: p03_draw_sub_av(ldpf, &ne, "NE")?,
            };
            p03_lp_repr.insert(format!("{s}-{f}"), sbre);
        }
    }
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p03_lp_repr_{yr}.bin");
    println!("write pre {} to {fnm}", p03_lp_repr.len());
    let bin: Vec<u8> = bincode::encode_to_vec(&p03_lp_repr, bincode::config::standard()).unwrap();
    std::fs::write(fnm, bin).unwrap();
    Ok(())
}

use std::path::Path;

pub fn p03_draw_all() -> Result<(), Box<dyn Error>> {
    for yr in ["2023", "2024"] {
        println!("{yr}");
        let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
        let bytes = std::fs::read(fnm).unwrap();
        let (subh, _): (HashMap<String, SubLoadProf>, usize) =
            bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
        println!("sub {}", subh.len());
        //println!("subh: {}", subh.len());
        let mut keys: Vec<_> = subh.keys().collect();
        keys.sort();
        for sb in &keys {
            //let ddr = format!("/mnt/e/CHMBACK/pea-data/lpdrw/{yr}/{sb}/000_01-01");
            let ddr = format!("/mnt/e/CHMBACK/pea-data/lpdrw/{yr}/{sb}/364_12-31");
            if Path::new(&ddr).exists() {
                println!("{yr}-{sb} {ddr} exists");
                continue;
            }
            let sb = sb.to_string();
            let sub = subh.get(&sb).unwrap();
            //println!("s: {}", sub.sub);
            /*
            let mut fc = 0;
            for f in &sub.feeds {
                let f = f.to_string();
                let flp = sub.fdldp.get(&f).unwrap();
                println!("  f:{f} - {}", flp.days.len());
                for (di, dlp) in flp.days.iter().enumerate() {
                    if let Some(dlp) = dlp {
                        let lb1 = format!("{sb}-{f}");
                        let mon = dlp.mon;
                        let mdt = dlp.mdt;
                        let dnm = format!(
                            "/mnt/e/CHMBACK/pea-data/lpdrw/{yr}/{sb}/{di:03}_{mon:02}-{mdt:02}/{lb1}.jpg"
                        );
                        if !Path::new(&dnm).exists() {
                            fc += 1;
                        }
                    }
                }
            }
            if fc == 0 {
                println!("{yr}-{sb} exists");
                continue;
            }
            */
            println!(">>> {yr}-{sb} NEW");
            for f in &sub.feeds {
                let f = f.to_string();
                let flp = sub.fdldp.get(&f).unwrap();
                println!("  f:{f} - {}", flp.days.len());
                for (di, dlp) in flp.days.iter().enumerate() {
                    if let Some(dlp) = dlp {
                        let lb1 = format!("{sb}-{f}");
                        let mon = dlp.mon;
                        let mdt = dlp.mdt;
                        let lb2 = format!("{yr}-{mon:02}-{mdt:02}");
                        let val = dlp.val;
                        let dnm = format!(
                            "/mnt/e/CHMBACK/pea-data/lpdrw/{yr}/{sb}/{di:03}_{mon:02}-{mdt:02}"
                        );
                        std::fs::create_dir_all(&dnm)?;
                        let fnm = format!("{dnm}/{lb1}.jpg");
                        //println!("{lb1} {lb2} -> {fnm}");
                        let sz = (400, 300);
                        let rf = Vec::<(String, f32)>::new();

                        let drlp = LoadProf {
                            lb1,
                            lb2,
                            fnm,
                            val,
                            sz,
                            rf,
                        };
                        drlp.draw_prof()?;
                    }
                }
            }
            if let Some(ldpf) = &sub.ldpf {
                for (di, dlp) in ldpf.days.iter().enumerate() {
                    if let Some(dlp) = dlp {
                        //let lb1 = format!("{sb}");
                        let lb1 = sb.to_string();
                        let mon = dlp.mon;
                        let mdt = dlp.mdt;
                        let lb2 = format!("{yr}-{mon:02}-{mdt:02}");
                        let val = dlp.val;
                        let dnm = format!(
                            "/mnt/e/CHMBACK/pea-data/lpdrw/{yr}/{sb}/{di:03}_{mon:02}-{mdt:02}"
                        );
                        std::fs::create_dir_all(&dnm)?;
                        let fnm = format!("{dnm}/{lb1}.jpg");
                        //println!("{lb1} {lb2} -> {fnm}");
                        let sz = (400, 300);
                        let rf = Vec::<(String, f32)>::new();

                        let drlp = LoadProf {
                            lb1,
                            lb2,
                            fnm,
                            val,
                            sz,
                            rf,
                        };
                        drlp.draw_prof()?;
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn p03_draw_slp(yr: &str, sb: &str) -> Result<(), Box<dyn Error>> {
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (subh, _): (HashMap<String, SubLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    println!("sub {}", subh.len());
    //println!("subh: {}", subh.len());
    let mut keys: Vec<_> = subh.keys().collect();
    keys.sort();
    //for sb in &keys {
    {
        let sb = sb.to_string();
        let sub = subh.get(&sb).unwrap();
        println!("s: {}", sub.sub);
        if let Some(ldpf) = &sub.ldpf {
            for (di, dlp) in ldpf.days.iter().enumerate() {
                if let Some(dlp) = dlp {
                    let lb1 = format!("{sb}");
                    let mon = dlp.mon;
                    let mdt = dlp.mdt;
                    let lb2 = format!("{yr}-{mon:02}-{mdt:02}");
                    let val = dlp.val;
                    let dnm = format!(
                        "/mnt/e/CHMBACK/pea-data/lpdrw/{yr}/{sb}/{di:03}_{mon:02}-{mdt:02}"
                    );
                    std::fs::create_dir_all(&dnm)?;
                    let fnm = format!("{dnm}/{lb1}.jpg");
                    //println!("{lb1} {lb2} -> {fnm}");
                    let sz = (400, 300);
                    let rf = Vec::<(String, f32)>::new();

                    let drlp = LoadProf {
                        lb1,
                        lb2,
                        fnm,
                        val,
                        sz,
                        rf,
                    };
                    drlp.draw_prof()?;
                }
            }
        } else {
            println!("  NO SUB SUM");
        }
        for f in &sub.feeds {
            let f = f.to_string();
            let flp = sub.fdldp.get(&f).unwrap();
            println!("  f:{f} - {}", flp.days.len());
            for (di, dlp) in flp.days.iter().enumerate() {
                if let Some(dlp) = dlp {
                    let lb1 = format!("{sb}-{f}");
                    let mon = dlp.mon;
                    let mdt = dlp.mdt;
                    let lb2 = format!("{yr}-{mon:02}-{mdt:02}");
                    let val = dlp.val;
                    let dnm = format!(
                        "/mnt/e/CHMBACK/pea-data/lpdrw/{yr}/{sb}/{di:03}_{mon:02}-{mdt:02}"
                    );
                    std::fs::create_dir_all(&dnm)?;
                    let fnm = format!("{dnm}/{lb1}.jpg");
                    //println!("{lb1} {lb2} -> {fnm}");
                    let sz = (400, 300);
                    let rf = Vec::<(String, f32)>::new();

                    let drlp = LoadProf {
                        lb1,
                        lb2,
                        fnm,
                        val,
                        sz,
                        rf,
                    };
                    drlp.draw_prof()?;
                }
            }
        }
    }

    Ok(())
}
