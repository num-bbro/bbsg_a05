use geo::Area;
use geo::Polygon;
//use geo::{point, Polygon};
use geo_types::{coord, LineString};
//use regex::Regex;
use sglab02_lib::sg::gis1::ar_list;
use sglib04::geo3::PopuDenseSave;
use sglib04::ld1::p13_am_po_de;
use sglib04::ld1::p13_mu_po_de;
use std::collections::HashMap;
use std::error::Error;

pub fn p06_amp_chk2() -> Result<(), Box<dyn Error>> {
    let mut ampo = 0f32;
    let mut mupo = 0f32;
    let mut mu_dn = HashMap::<String, (f32, f32)>::new();
    let (mut mmx, mut mmn) = (10f32, 10f32);
    let (mut amx, mut amn) = (10f32, 10f32);
    let mut mu_sz = HashMap::<u32, usize>::new();
    let mut am_sz = HashMap::<u32, usize>::new();
    for ar in ar_list() {
        let mu = p13_mu_po_de(ar)?;
        for dn in &mu {
            let key = dn.key.to_string();
            if let Some((_po, aa)) = mu_dn.get_mut(&key) {
                *aa += dn.area;
            } else {
                mupo += dn.popu;
                mu_dn.insert(key, (dn.popu, dn.area));
            }
        }
    }
    for (k, (p, a)) in &mu_dn {
        let a = a / 1_000f32;
        let pd = p / a * 2.5f32;
        let sz = match pd {
            0f32..15f32 => 1u32,
            15f32..30f32 => 2u32,
            30f32..70f32 => 3u32,
            70f32..200f32 => 4u32,
            _ => 5u32,
        };
        println!("ท.{k} {p} {a} {pd:.1}");
        mmx = mmx.max(pd);
        mmn = mmn.min(pd);
        if let Some(cn) = mu_sz.get_mut(&sz) {
            *cn += 1;
        } else {
            mu_sz.insert(sz, 1usize);
        }
    }

    let mut am_dn = HashMap::<String, (f32, f32)>::new();
    for ar in ar_list() {
        let mu = p13_am_po_de(ar)?;
        for dn in &mu {
            let key = dn.key.to_string();
            if let Some((_po, aa)) = am_dn.get_mut(&key) {
                *aa += dn.area;
            } else {
                ampo += dn.popu;
                am_dn.insert(key, (dn.popu, dn.area));
            }
        }
    }
    for (k, (p, a)) in &am_dn {
        let a = a / 1_000f32;
        let pd = p / a * 0.6f32;
        let sz = match pd {
            0f32..30f32 => 1u32,
            30f32..60f32 => 2u32,
            60f32..150f32 => 3u32,
            150f32..500f32 => 4u32,
            _ => 5u32,
        };
        println!("อ.{k} {p} {a} {pd:.1}");
        amx = amx.max(pd);
        amn = amn.min(pd);
        if let Some(cn) = am_sz.get_mut(&sz) {
            *cn += 1;
        } else {
            am_sz.insert(sz, 1usize);
        }
    }
    println!("people: a:{ampo} m:{mupo}");
    println!("minmax: a:{amn}-{amx} m:{mmn}-{mmx}");
    println!("mu_sz: {mu_sz:?}");
    println!("am_sz: {am_sz:?}");
    Ok(())
}

pub fn p06_amp_chk1() -> Result<(), Box<dyn Error>> {
    println!("amp chk");
    let (mut amx, mut amn) = (20000f32, 20000f32);
    let (mut mmx, mut mmn) = (7000f32, 7000f32);
    let mut am_sz = HashMap::<u32, u32>::new();
    let mut mu_sz = HashMap::<u32, u32>::new();
    //let re = Regex::new(r"-เชียงใหม่").unwrap();
    for ar in ar_list() {
        println!("{ar}");
        let mu = p13_mu_po_de(ar)?;
        println!("================ muni");
        for d in &mu {
            let dn: &PopuDenseSave = d;
            //let pk = dn.popu / (dn.area / 1_000f32);
            let mut aa = 0f32;
            for gon in &dn.gons {
                let mut lines = vec![];
                for (x, y) in gon {
                    lines.push(coord! { x: *x, y: *y, });
                }
                let line_string = LineString::new(lines);
                let polygon = Polygon::new(line_string.clone(), vec![]);
                aa += polygon.unsigned_area();
            }
            aa /= 1_000_000f32;
            let pk = dn.popu / aa;
            mmx = mmx.max(dn.popu);
            mmn = mmn.min(dn.popu);
            let sz = match dn.popu {
                0f32..3000f32 => 1u32,
                3000f32..4500f32 => 2u32,
                4500f32..7000f32 => 3u32,
                7000f32..12000f32 => 4u32,
                _ => 5u32,
            };
            if let Some(cn) = mu_sz.get_mut(&sz) {
                *cn += 1;
            } else {
                mu_sz.insert(sz, 1);
            }
            println!(
                "{} p:{} a:{} d:{} = {pk} aa:{aa}",
                dn.key, dn.popu, dn.area, dn.dens
            );
        }
        let am = p13_am_po_de(ar)?;
        println!("================ amp");
        for d in &am {
            let dn: &PopuDenseSave = d;
            let mut aa = 0f32;
            for gon in &dn.gons {
                let mut lines = vec![];
                for (x, y) in gon {
                    lines.push(coord! { x: *x, y: *y, });
                }
                let line_string = LineString::new(lines);
                let polygon = Polygon::new(line_string.clone(), vec![]);
                aa += polygon.unsigned_area();
            }
            aa /= 1_000_000f32;
            let pk = dn.popu / aa;
            amx = amx.max(dn.popu);
            amn = amn.min(dn.popu);
            let sz = match dn.popu {
                0f32..30000f32 => 1u32,
                30000f32..48000f32 => 2u32,
                48000f32..70000f32 => 3u32,
                70000f32..90000f32 => 4u32,
                _ => 5u32,
            };
            if let Some(cn) = am_sz.get_mut(&sz) {
                *cn += 1;
            } else {
                am_sz.insert(sz, 1);
            }
            println!(
                "{} p:{} a:{} d:{} = {pk} aa:{aa}",
                dn.key, dn.popu, dn.area, dn.dens
            );
        }
    }
    println!("mu:{mmn} - {mmx}");
    println!("{mu_sz:?}");

    println!("am:{amn} - {amx}");
    println!("{am_sz:?}");

    Ok(())
}
