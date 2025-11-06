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

    use headless_chrome::protocol::cdp::Page;
    use headless_chrome::types::Bounds;
    use headless_chrome::Browser;
    //use image::GenericImage;
    //use image::GenericImageView;
    use image::Rgba;
    //use image::Rgba;
    //use imageproc::drawing::draw_filled_rect_mut;
    //use imageproc::drawing::draw_hollow_polygon_mut;
    //use imageproc::drawing::draw_hollow_rect_mut;
    //use imageproc::pixelops::interpolate;
    //use imageproc::point::Point;
    //use regex::Regex;
    //use sglab02_lib::sg::mvline::latlong_utm;
    use sglab02_lib::sg::mvline::utm_latlong;
    //use sglab02_lib::sg::prc3::ld_p3_sub_inf;
    //use sglib03::subtype::SUB_TYPES;
    use sglib04::aoj::meter_pixel_to_zoom_lat;
    use sglib04::aoj::zoom_to_meter_pixel_lat;
    use std::{thread, time};

    // IMAGE CREATE BEGIN
    pub const MP_WW: f32 = 1800_f32;
    pub const MP_HH: f32 = 1733_f32 - 185_f32 * 2.0;
    pub const MP_MG: u32 = 72;
    pub const MP_UPDW: u32 = 185;

    let mg = MP_MG;
    let updw = MP_UPDW;
    let ww = MP_WW;
    let hh = MP_HH;

    let (w, h) = (mg as f32 + ww, updw as f32 * 2.0 + hh);

    let fst = assv[0].n1d.n1d_2_utm();
    let (mut x0, mut y0, mut x1, mut y1) = (fst.0, fst.1, fst.0, fst.1);
    println!("corner {x0},{y0}");
    for a in &assv {
        let pnt = a.n1d.n1d_2_utm();
        if pnt.0 == 0.0 || pnt.1 == 0.0 {
            continue;
        }
        //println!(" tr {},{}", pnt.0, pnt.1);
        x0 = x0.min(pnt.0);
        y0 = y0.min(pnt.1);
        x1 = x1.max(pnt.0);
        y1 = y1.max(pnt.1);
    }
    let (ox, oy) = ((x1 + x0) * 0.5f32, (y1 + y0) * 0.5f32);
    let wd = x1 - x0;
    let (o_ld, o_ln) = utm_latlong(ox, oy);
    let zm = meter_pixel_to_zoom_lat(wd, ww as u32, o_ld);
    let mtpx = zoom_to_meter_pixel_lat(zm, o_ld);

    let ex_x = mtpx * ww;
    let ex_y = mtpx * hh;
    let (sb_x, sb_y) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (xx, yy) = utm_latlong(sb_x, sb_y);
    let or_x = sb_x - ex_x / 2.0;
    let or_y = sb_y - ex_y / 2.0;
    let imdir = format!("{DNM}/img/");

    std::fs::create_dir_all(&imdir).expect("?");
    let blk = Rgba([0u8, 0u8, 0u8, 0u8]);
    let wht = Rgba([255u8, 255u8, 255u8, 0u8]);
    let red = Rgba([255u8, 0u8, 0u8, 0u8]);
    /*
    let cols = [
        Rgba([255u8, 0u8, 0u8, 0u8]),
        Rgba([0u8, 255u8, 0u8, 0u8]),
        Rgba([0u8, 0u8, 255u8, 0u8]),
        Rgba([255u8, 255u8, 0u8, 0u8]),
        Rgba([0u8, 255u8, 255u8, 0u8]),
    ];
    */
    let fimg1 = format!("{imdir}/{fdid}-1.jpeg");
    let fimg2 = format!("{imdir}/{fdid}-2.jpeg");
    loop {
        if !std::path::Path::new(fimg1.as_str()).exists() {
            println!("  wd:{wd} zm:{zm} ld,ln:{o_ld},{o_ln}");
            println!("    sb:{sb_x},{sb_y} dd:{ex_x},{ex_y}  or:{or_x},{or_y}");
            //println!("    offs:{} izns:{}", offs.len(), izns.len());

            let bnd = Bounds::Normal {
                left: None,
                top: None,
                width: Some(w.into()),
                //height: Some(h.into()),
                height: Some(w.into()),
            };
            let url = format!("https://www.google.pl/maps/@{xx},{yy},{zm}z");

            let browser = Browser::default().expect("browser");
            let tab = browser.new_tab().expect("new tab");
            if tab.navigate_to(&url).is_err() {
                println!("!!! fail to navigate to");
                continue;
            }
            if tab.set_bounds(bnd).is_err() {
                println!("!!! fail to set bound");
                continue;
            }
            if tab.wait_until_navigated().is_err() {
                println!("!!! fail to wait");
                continue;
            }

            let ten_millis = time::Duration::from_millis(2000);
            thread::sleep(ten_millis);
            let jpeg_data = tab
                .capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)
                .expect("capture");
            std::fs::write(&fimg1, jpeg_data).expect("image file");
            println!("img2 = {url} wrote {fimg1}");
        } else {
            println!("{fdid} image 2 skipped {fimg1}");
        }
        break;
    }
    use image::ImageReader;
    use imageproc::drawing::draw_filled_rect_mut;
    //use imageproc::drawing::draw_hollow_rect;
    use imageproc::rect::Rect;
    let ofs_x = 40f32;

    if let Ok(img) = ImageReader::open(&fimg1) {
        if let Ok(mut img) = img.decode() {
            let (w, h) = (img.width(), img.height());
            println!(" hh:{hh} h:{h} updw:{updw}");
            let mut img = img.crop(mg, updw, w - mg, h - updw * 2);
            // ============ add
            let x = (sb_x - or_x) * ww / ex_x - ofs_x;
            let y = (sb_y - or_y) * hh / ex_y;
            println!(
                " drw:{x},{y} {}/{} = {} hh:{hh} y:{y}",
                sb_y - or_y,
                ex_y,
                (sb_y - or_y) / ex_y
            );
            for a in assv.iter() {
                let pnt = a.n1d.n1d_2_utm();
                if pnt.0 == 0.0 || pnt.1 == 0.0 {
                    continue;
                }
                let x = (pnt.0 - or_x) * ww / ex_x - ofs_x;
                let y = (pnt.1 - or_y) * hh / ex_y;
                //println!("  draw {x},{y}");
                let rect = Rect::at(x as i32, y as i32).of_size(2, 2);
                draw_filled_rect_mut(&mut img, rect, red);
            }
            img.save(&fimg2).expect("?");
        }
    }

    // IMAGE CREATE END

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
