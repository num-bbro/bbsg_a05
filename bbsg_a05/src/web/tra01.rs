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
use headless_chrome::protocol::cdp::Page;
use headless_chrome::types::Bounds;
use headless_chrome::Browser;
use image::open;
use image::ImageReader;
//use image::Rgb;
//use image::RgbImage;
use image::Rgba;
use image::RgbaImage;
use image_blend::pixelops::pixel_mult;
use image_blend::DynamicChops;
use imageproc::drawing::draw_filled_circle_mut;
use imageproc::drawing::draw_line_segment_mut;
use sglab02_lib::sg::mvline::utm_latlong;
use sglib04::aoj::meter_pixel_to_zoom_lat;
use sglib04::aoj::zoom_to_meter_pixel_lat;
use std::collections::HashMap;
use std::{thread, time};
pub const MP_WW: f32 = 1800_f32;
pub const MP_HH: f32 = 1733_f32 - 185_f32 * 2.0;
pub const MP_MG: u32 = 72;
pub const MP_UPDW: u32 = 185;

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
    se_fld: VarType,
    shwfld: Vec<VarType>,
}

pub fn load_ggmap(gm: &PeaAssDrawMap, basemap: &str, fpath: &str) {
    let frdrw = format!("{fpath}rw.jpeg");
    loop {
        let frdrw = frdrw.as_str();
        if !std::path::Path::new(frdrw).exists() {
            //println!("    offs:{} izns:{}", offs.len(), izns.len());

            let bnd = Bounds::Normal {
                left: None,
                top: None,
                width: Some(gm.w.into()),
                height: Some(gm.w.into()),
            };
            let url = "https://www.google.com/maps/@?api=1&map_action=map";
            let url = format!(
                "{url}&center={},{}&zoom={}&basemap={basemap}",
                gm.xx, gm.yy, gm.zm,
            );
            //"https://www.google.com/maps/@?api=1&map_action=map&center={},{}&zoom={}&basemap={basemap}"
            //"https://www.google.com/maps/@?api=1&map_action=map&center={xx},{yy}&zoom={zm}&basemap=satellite"
            //"https://www.google.com/maps/@?api=1&map_action=map&center={xx},{yy}&zoom={zm}"

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
            std::fs::write(frdrw, jpeg_data).expect("image file");
            println!("img2 = {url} wrote {frdrw}");
        } else {
            println!("image 2 skipped {frdrw}");
        }
        break;
    }
}

pub struct PeaAssDrawMap<'a> {
    //pub fpath: &'a str,
    pub mg: u32,
    pub updw: u32,
    pub ww: f32,
    pub hh: f32,
    pub ex_x: f32,
    pub ex_y: f32,
    pub or_x: f32,
    pub or_y: f32,
    pub assv: &'a Vec<PeaAssVar>,
    pub ofs_x: f32,
    pub w: f32,
    pub xx: f32,
    pub yy: f32,
    pub zm: u32,
}

impl<'a> PeaAssDrawMap<'a> {
    pub fn new(assv: &'a Vec<PeaAssVar>) -> Self {
        let mg = MP_MG;
        let updw = MP_UPDW;
        let ww = MP_WW;
        let hh = MP_HH;

        let (w, _h) = (mg as f32 + ww, updw as f32 * 2.0 + hh);

        let fst = assv[0].n1d.n1d_2_utm();
        let (mut x0, mut y0, mut x1, mut y1) = (fst.0, fst.1, fst.0, fst.1);
        for a in assv.iter().take(assv.len() - 1) {
            let pnt = a.n1d.n1d_2_utm();
            x0 = x0.min(pnt.0);
            y0 = y0.min(pnt.1);
            x1 = x1.max(pnt.0);
            y1 = y1.max(pnt.1);
        }
        let (ox, oy) = ((x1 + x0) * 0.5f32, (y1 + y0) * 0.5f32);
        let wd = x1 - x0;
        let (o_ld, _o_ln) = utm_latlong(ox, oy);
        let zm = meter_pixel_to_zoom_lat(wd, ww as u32, o_ld);
        let mtpx = zoom_to_meter_pixel_lat(zm, o_ld);

        let ex_x = mtpx * ww;
        let ex_y = mtpx * hh;
        let (sb_x, sb_y) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (xx, yy) = utm_latlong(sb_x, sb_y);
        let or_x = sb_x - ex_x / 2.0;
        let or_y = sb_y - ex_y / 2.0;
        let ofs_x = 40f32;

        PeaAssDrawMap {
            mg,
            updw,
            ww,
            hh,
            ex_x,
            ex_y,
            or_x,
            or_y,
            assv,
            ofs_x,
            w,
            xx,
            yy,
            zm,
        }
    }
}

pub fn draw_ass_map(map: &PeaAssDrawMap, fpath: &str) {
    let frdrw = format!("{}rw.jpeg", fpath);
    let frd02 = format!("{}02.jpeg", fpath);
    println!("frd02: {frd02}");
    println!("frdrw: {frdrw}");
    if !std::path::Path::new(&frd02).exists()
        && let Ok(img) = ImageReader::open(&frdrw)
        && let Ok(mut img) = img.decode()
    {
        let (w, h) = (img.width(), img.height());
        let img = img.crop(map.mg, map.updw, w - map.mg, h - map.updw * 2);
        let frdcp = format!("{}cp.jpeg", fpath);
        img.save(&frdcp).expect("?");

        let (w, h) = (img.width(), img.height());

        let mut img0 = RgbaImage::new(w, h);
        for a in map.assv.iter() {
            let pnt = a.n1d.n1d_2_utm();
            let x = (pnt.0 - map.or_x) * map.ww / map.ex_x - map.ofs_x;
            let y = (pnt.1 - map.or_y) * map.hh / map.ex_y;
            let (ix, iy) = (x as i32, y as i32);
            draw_filled_circle_mut(&mut img0, (ix, iy), 5, Rgba([0, 0, 255, 255]));
            draw_line_segment_mut(
                &mut img0,
                (x, y - 10.0),
                (x, y + 10.0),
                Rgba([255, 0, 0, 255]),
            );
            draw_line_segment_mut(
                &mut img0,
                (x - 10.0, y),
                (x + 10.0, y),
                Rgba([255, 0, 0, 255]),
            );
        }
        let frd01 = format!("{}01.png", fpath);
        img0.save(&frd01).expect("?");

        let mut img1 = open(&frdcp).unwrap();
        let img2 = open(&frd01).unwrap();
        img1.blend(&img2, pixel_mult, true, false).unwrap();
        img1.save(frd02).expect("?");
    }
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
        "".to_string()
        //format!("{sbid}01")
    };
    let Some(se_fld) = fldm.get(&fld) else {
        println!("NO SELECTED FIELD");
        return WebTemp::default();
    };
    let name = format!("FIELD {fld}");
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
        .filter(|a| fdid.is_empty() || (a.fdid == fdid && a.own == "P"))
        .cloned()
        .collect::<Vec<_>>();
    let mut sumv = PeaAssVar::from(0u64);
    let mut assv = Vec::<PeaAssVar>::new();
    for ass in assv0 {
        sumv.add(&ass);
        assv.push(ass);
    }

    let sbif = ld_sub_info();
    let Some(sbinf) = sbif.get(&sbid) else {
        println!("NO rw3.bin file:");
        return WebTemp::default();
    };

    assv.push(sumv);

    WebTemp {
        name,
        assv,
        fld,
        sbid,
        fdid,
        sbinf: sbinf.clone(),
        se_fld: se_fld.clone(),
        shwfld: SHOW_FLDS.to_vec(),
    }
}
