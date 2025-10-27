use bincode::{Decode, Encode};
use chrono::{Datelike, NaiveDateTime};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

pub const DAY_VAL_PNTS: usize = 24 * 4;
pub const ONE_DAY_SEC: usize = 60 * 60 * 24;
pub const ONE_TIK_SEC: usize = 15 * 60;

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct DayLoadProf {
    pub mon: u32,
    pub wdy: u32,
    pub mdt: u32,
    pub cnt: usize,
    pub val: [Option<f32>; DAY_VAL_PNTS],
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct FeederLoadProf {
    pub year: u32,
    pub sub: String,
    pub feed: String,
    pub name: String,
    pub days: Vec<Option<DayLoadProf>>,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct SubLoadProf {
    pub year: u32,
    pub sub: String,
    pub feeds: Vec<String>,
    pub fdldp: HashMap<String, FeederLoadProf>,
    pub ldpf: Option<FeederLoadProf>,
}

pub fn p02_read_lp(yr: &str) -> Result<(), Box<dyn Error>> {
    let year = yr.parse::<u32>().unwrap();
    let dtt0 = NaiveDateTime::parse_from_str(&format!("{yr}-01-01 00:00"), "%Y-%m-%d %H:%M")?;
    let t0 = dtt0.and_utc().timestamp_millis();
    //let mut fdmp = HashMap::<String, Box<FeederLoadRaw>>::new();
    //let mut sbmp = HashMap::<String, i32>::new();
    let tmpt = Regex::new(r"[0-2][0-9]:(00|15|30|45)").unwrap();
    let vap0 = Regex::new(r"^-?[0-9]+$").unwrap();
    let vap1 = Regex::new(r"^-?[0-9]+\.[0-9]+$").unwrap();
    let lp_dr = format!("/mnt/e/CHMBACK/pea-data/loadprofile{yr}");
    let mut lps = HashMap::<String, FeederLoadProf>::new();
    for m in 1..=12 {
        let mn = format!("{}/Load Profile {yr}{:02}.csv", lp_dr, m);
        println!("m:{}", mn);
        if let Ok(mut rdr) = csv::Reader::from_path(&mn) {
            let (mut er1, mut er2, mut er3) = (0, 0, 0);
            let mut rc1 = 0;
            for rc in rdr.records().flatten() {
                if let (Some(sb), Some(nm), Some(fd), Some(dt), Some(tm), Some(va)) = (
                    rc.get(0),
                    rc.get(1),
                    rc.get(2),
                    rc.get(3),
                    rc.get(4),
                    rc.get(5),
                ) {
                    rc1 += 1;
                    let sub = sb.trim().to_string();
                    let feed = fd.trim().to_string();
                    let name = nm.to_string();
                    let fd = format!("{sub}_{feed}");
                    let dtf = format!("{} {}", dt, tm);
                    //println!("dtf: {}", dtf);
                    if let Ok(dttm) = NaiveDateTime::parse_from_str(dtf.as_str(), "%Y-%m-%d %H:%M")
                    {
                        if !tmpt.is_match(tm) {
                            continue;
                        }
                        if va == "NULL" {
                            er3 += 1;
                            continue;
                        } else if vap0.is_match(va) || vap1.is_match(va) {
                        } else {
                            er1 += 1;
                            println!("3:{rc1} v:{va} - {dttm} {sb} {fd}");
                            continue;
                        }
                        let Ok(v) = va.parse::<f32>() else {
                            er2 += 1;
                            //println!("3:{rc1} v:{va} - {dttm} {sb} {fd}");
                            continue;
                        };
                        lps.entry(fd.to_string()).or_insert_with(|| FeederLoadProf {
                            year,
                            sub,
                            feed,
                            name,
                            days: Vec::<_>::new(),
                        });
                        if let Some(lp) = lps.get_mut(&fd) {
                            let wdy = dttm.weekday() as u32;
                            let mdt = dttm.day();
                            let mon = dttm.month();
                            let t1 = dttm.and_utc().timestamp_millis();
                            let dtsec = (t1 - t0) / 1000;
                            let dtsec = dtsec as usize;
                            let dtday = dtsec / ONE_DAY_SEC;
                            let dtpnt = (dtsec - dtday * ONE_DAY_SEC) / ONE_TIK_SEC;
                            while dtday >= lp.days.len() {
                                lp.days.push(None);
                            }
                            if let Some(dlp) = &mut lp.days[dtday] {
                                dlp.val[dtpnt] = Some(v);
                            } else {
                                let mut val = [None; DAY_VAL_PNTS];
                                val[dtpnt] = Some(v);
                                let dlp = DayLoadProf {
                                    wdy,
                                    mdt,
                                    mon,
                                    cnt: 0,
                                    val,
                                };
                                lp.days[dtday] = Some(dlp);
                            }
                        }
                    } // end of date time calculation
                } else {
                    println!("   error: {:?}", rc);
                }
            } // end loop month
            println!("   === c:{rc1} e1:{er1} e2:{er2} e3:{er3} fd:{}", lps.len());
        } else {
            println!("  !!! could not open {}", mn);
        } // end if file exists
    }
    println!("feeder {}", lps.len());
    let bin: Vec<u8> = bincode::encode_to_vec(&lps, bincode::config::standard()).unwrap();
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_read_lp_{yr}.bin");
    std::fs::write(fnm, bin).unwrap();

    Ok(())
}

pub fn p02_lp_pro(yr: &str) -> Result<(), Box<dyn Error>> {
    let year = yr.parse::<u32>().unwrap();
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_read_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (mut lps, _): (HashMap<String, FeederLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    println!("lps: {}", lps.len());
    let mut subh = HashMap::<String, SubLoadProf>::new();
    for lp in lps.values_mut() {
        for dlp in lp.days.iter_mut().flatten() {
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
            /*
                    ..Default::default()
            pub year: u32,
            pub sub: String,
            pub feeds: Vec<String>,
            pub fdldp: HashMap<String, FeederLoadProf>,
            pub lppf: Option<FeederLoadProf>,
                    */
        });
        let Some(sub) = subh.get_mut(&lp.sub) else {
            continue;
        };
        sub.fdldp.insert(lp.feed.to_string(), lp.clone());
    }
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

pub fn p02_draw_lp(yr: &str) -> Result<(), Box<dyn Error>> {
    let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p02_sub_lp_{yr}.bin");
    let bytes = std::fs::read(fnm).unwrap();
    let (subh, _): (HashMap<String, SubLoadProf>, usize) =
        bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
    println!("sub {}", subh.len());
    //println!("subh: {}", subh.len());
    let mut keys: Vec<_> = subh.keys().collect();
    keys.sort();
    for sb in &keys {
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

use ab_glyph::FontVec;
use ab_glyph::PxScale;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct LoadProf {
    pub lb1: String,
    pub lb2: String,
    pub fnm: String,
    pub val: [Option<f32>; DAY_VAL_PNTS],
    pub sz: (usize, usize),
    pub rf: Vec<(String, f32)>,
}

impl DrawLoadProf for LoadProf {
    fn sz(&self) -> (usize, usize) {
        if self.sz == (0, 0) {
            Self::SIZE
        } else {
            self.sz
        }
    }
    fn lb1(&self) -> String {
        self.lb1.to_string()
    }
    fn lb2(&self) -> String {
        self.lb2.clone()
    }
    fn fnm(&self) -> String {
        self.fnm.to_string()
    }
    fn val(&self) -> [Option<f32>; DAY_VAL_PNTS] {
        self.val.clone()
    }
    fn rf(&self) -> Vec<(String, f32)> {
        self.rf.clone()
    }
}

pub trait DrawLoadProf {
    fn lb1(&self) -> String;
    fn lb2(&self) -> String;
    fn fnm(&self) -> String;
    fn val(&self) -> [Option<f32>; DAY_VAL_PNTS];
    const SIZE: (usize, usize) = (400, 300);
    fn sz(&self) -> (usize, usize) {
        Self::SIZE
    }
    fn mg(&self) -> (usize, usize, usize, usize) {
        (48, 40, 45, 25) // top,bottom,left,right
    }
    fn tik(&self) -> Vec<f32> {
        let mut tk = 0f32;
        let mut tiks = vec![];
        let val = self.val();
        let val = &val;
        if val.is_empty() {
            println!("val empty {val:?}");
            return tiks;
        }
        let Some(ymx) = val
            .iter()
            .flatten()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
        else {
            //println!("error {val:?}");
            return tiks;
        };
        let ymx = *ymx;
        let ymn = val
            .iter()
            .flatten()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let ymn = ymn.abs();
        let dy = (ymx + 4f32) / 5f32;
        let dy = if ymn > ymx { (ymn + 4f32) / 5f32 } else { dy };

        for _i in 0..10 {
            tk += dy;
            tiks.push(-tk);
            if tk > ymn {
                break;
            }
        }
        tiks.reverse();
        //println!("{}.tik1 - {tiks:?}", self.sub());
        let mut tk = 0f32;
        tiks.push(tk);
        for _i in 0..10 {
            tk += dy;
            tiks.push(tk);
            if tk > ymx {
                break;
            }
        }
        //println!("{}.tik2 - {tiks:?}", self.sub());
        tiks
    }
    fn si(&self) -> (usize, usize) {
        let (wd, hg) = self.sz();
        let (mt, mb, ml, mr) = self.mg();
        (wd - ml - mr, hg - mt - mb)
    }
    fn image(&self) -> RgbImage {
        let (wd, hg) = self.sz();
        RgbImage::new(wd as u32, hg as u32)
    }
    fn wht(&self) -> Rgb<u8> {
        Rgb([255u8, 255u8, 255u8])
    }
    fn blk(&self) -> Rgb<u8> {
        Rgb([0u8, 0u8, 0u8])
    }
    fn grn(&self) -> Rgb<u8> {
        Rgb([0u8, 130u8, 0u8])
    }
    fn blu(&self) -> Rgb<u8> {
        Rgb([0u8, 0u8, 100u8])
    }
    fn yel(&self) -> Rgb<u8> {
        Rgb([200u8, 200u8, 0u8])
    }
    fn red(&self) -> Rgb<u8> {
        Rgb([130u8, 0u8, 0u8])
    }
    fn gr1(&self) -> Rgb<u8> {
        Rgb([230u8, 230u8, 230u8])
    }
    fn gr2(&self) -> Rgb<u8> {
        Rgb([242u8, 242u8, 242u8])
    }
    fn gr3(&self) -> Rgb<u8> {
        Rgb([150u8, 150u8, 150u8])
    }
    fn rf(&self) -> Vec<(String, f32)> {
        Vec::<(String, f32)>::new()
    }
    //--- drawing
    fn draw_prof(&self) -> Result<Vec<u8>, String> {
        let fnm = self.fnm();
        let val = self.val();
        let mut image = self.image();
        let (wd, hg) = self.sz();
        let (mt, mb, ml, mr) = self.mg();
        let (wdi, hgi) = (wd - ml - mr, hg - mt - mb);
        let tik = self.tik();
        if tik.is_empty() {
            return Err("draw_prof Error #1".into());
        }
        let vwd = tik[tik.len() - 1] - tik[0];
        let hgrt = hgi as f32 / vwd;
        let zrlv = (0f32 - tik[0]) / vwd;
        let ory = (mt + hgi) as i32 - (hgi as f32 * zrlv) as i32;
        let scl = PxScale::from(24.0);
        let sc2 = PxScale::from(28.0);
        let font_vec = Vec::from(include_bytes!("THSarabunNew.ttf") as &[u8]);
        let font = FontVec::try_from_vec(font_vec).expect("Font Vec");
        let (wht, blk, _grn) = (self.wht(), self.blk(), self.grn());
        let (gr1, gr2, gr3) = (self.gr1(), self.gr2(), self.gr3());
        let wdrt = wdi as f32 / DAY_VAL_PNTS as f32;

        //println!("..1");
        // all day
        draw_filled_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(wd as u32, hg as u32),
            wht,
        );
        // morning
        draw_filled_rect_mut(
            &mut image,
            Rect::at(ml as i32, mt as i32).of_size(wdi as u32 / 4_u32, hgi as u32),
            gr2,
        );
        // night
        draw_filled_rect_mut(
            &mut image,
            Rect::at((ml + wdi * 3 / 4) as i32, mt as i32).of_size(wdi as u32 / 4_u32, hgi as u32),
            gr2,
        );
        //println!("..2");

        // x tick & label
        let tik_ev = DAY_VAL_PNTS / 12;
        for i in 0..=DAY_VAL_PNTS {
            //if i % 4 != 0 {
            if i % tik_ev != 0 {
                continue;
            }
            let xi = i as f32 * wdrt + ml as f32;
            //let y1 = (hg - mb) as f32;
            let y1 = ory as f32;
            let y2 = y1 + 5f32;
            draw_line_segment_mut(&mut image, (xi, y1), (xi, y2), blk);
            let (y1, y2) = (mt as f32, (mt + hgi) as f32);
            draw_line_segment_mut(&mut image, (xi, y1), (xi, y2), gr1);

            //let hi = i / 2;
            let hi = i / 4;
            let hi = format!("{:02}น", hi);
            let xi = xi as i32 - 8;
            let yi = y2 as i32;
            draw_text_mut(&mut image, blk, xi, yi, scl, &font, &hi);
        }

        for (_nm, v) in self.rf() {
            let yi = ory as f32 - v * hgrt;
            let (x1, x2) = (ml as f32, (wd - mr) as f32);
            //println!("nm:{} v:{} x1:{x1} x2:{x2} yi:{yi}", nm, v);
            draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), self.yel());
            //draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), grn);
        }

        // y ticks & label
        for v in &tik {
            let yi = ory as f32 - v * hgrt;
            let x1 = ml as f32;
            let x2 = x1 - 5f32;
            draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), blk);

            let (x1, x2) = (ml as f32, (wd - mr) as f32);
            draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), gr1);

            let x1 = ml as f32;
            let x2 = x1 - 5f32;
            let va = format!("{:.1}", v);
            let xi = x2 as i32 - 15 - va.len() as i32 * 4;
            let yi = yi as i32 - 10;
            draw_text_mut(&mut image, blk, xi, yi, scl, &font, &va);
        }

        let col = self.grn();
        for (di, v) in val.iter().enumerate() {
            //let di = di % 48;
            if let Some(v) = v {
                let di = di % DAY_VAL_PNTS;
                let xi = di as f32 * wdrt + ml as f32;
                let yi = ory as f32 - v * hgrt;
                let x2 = xi + wdrt; //10f32;
                let _dy = v * hgrt;
                draw_line_segment_mut(&mut image, (xi, yi), (x2, yi), col);
                draw_line_segment_mut(&mut image, (xi + wdrt, yi), (xi + wdrt, ory as f32), col);
                draw_line_segment_mut(&mut image, (xi, yi), (xi, ory as f32), col);
            }
        }

        draw_text_mut(&mut image, blk, 20, 12, sc2, &font, &self.lb1());
        draw_text_mut(&mut image, blk, 180, 12, sc2, &font, &self.lb2());
        // border lines

        // up bar
        let (x1, y1, x2, y2) = (ml as f32, mt as f32, (wd - ml) as f32, mt as f32);
        draw_line_segment_mut(&mut image, (x1, y1), (x2, y2), gr1);
        // low bar
        let (x1, y1, x2, y2) = (
            ml as f32,
            (mt + hgi) as f32,
            (wd - ml) as f32,
            (mt + hgi) as f32,
        );
        draw_line_segment_mut(&mut image, (x1, y1), (x2, y2), gr1);

        // left pipe
        let (x1, y1, x2, y2) = (ml as f32, mt as f32, ml as f32, (mt + hgi) as f32);
        draw_line_segment_mut(&mut image, (x1 - 1f32, y1), (x2 - 1f32, y2), gr3);

        // right pipe
        let (x1, y1, x2, y2) = (
            (ml + wdi) as f32,
            mt as f32,
            (ml + wdi) as f32,
            (mt + hgi) as f32,
        );
        draw_line_segment_mut(&mut image, (x1 + 1f32, y1), (x2 + 1f32, y2), gr1);

        let x1 = ml as f32;
        let x2 = x1 + wdi as f32; //10f32;
        let yi = ory as f32;
        draw_line_segment_mut(&mut image, (x1, yi), (x2, yi), blk);

        //println!("..3 {fnm}");
        // show data
        if image.save(&fnm).is_ok() {
            //println!("..4 {fnm}");
            if let Ok(mut f) = File::open(&fnm) {
                if let Ok(mt) = fs::metadata(&fnm) {
                    //println!("..5 {}", mt.len());
                    let mut buffer = vec![0; mt.len() as usize];
                    //if let Ok(_) = f.read(&mut buffer) {
                    if f.read(&mut buffer).is_ok() {
                        return Ok(buffer);
                    }
                }
            }
        }

        Ok(vec![])
    }
}
