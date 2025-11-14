use crate::dcl::Geo;
use crate::dcl::PeaAssVar;
use crate::dcl::PeaSub;
use crate::dcl::DNM;
use axum::extract::Query;
use axum::http::header::{self, HeaderMap};
use axum::response::IntoResponse;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::types::Bounds;
use headless_chrome::Browser;
use image::open;
use image::ImageReader;
use image::Rgba;
use image::RgbaImage;
use image_blend::pixelops::pixel_mult;
use image_blend::DynamicChops;
use imageproc::drawing::draw_filled_circle_mut;
use imageproc::drawing::draw_line_segment_mut;
use serde::Deserialize;
use serde_json::Value;
//use sglab02_lib::sg::mvline::latlong_utm;
use sglab02_lib::sg::mvline::utm_latlong;
//use sglib04::aoj::meter_pixel_to_zoom_lat;
use sglib04::aoj::zoom_to_meter_pixel_lat;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::{thread, time};
pub const MP_WW: f32 = 1800_f32;
pub const MP_HH: f32 = 1733_f32 - 185_f32 * 2.0;
pub const MP_MG: u32 = 72;
pub const MP_UPDW: u32 = 185;
use std::error::Error;
use std::path::Path;

pub fn meter_pixel_to_zoom_lat_2(dx: f32, px: u32, lat: f32) -> u32 {
    let mut z0 = 25u32;
    for z in (0u32..=24u32).rev() {
        let d1 = zoom_to_meter_pixel_lat(z0, lat) * px as f32;
        //println!("  {z} {d1} {dx}");
        if d1 > dx {
            return z;
        }
        z0 = z;
    }
    0
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub fld: Option<String>,
    pub sbid: Option<String>,
    pub fdid: Option<String>,
}

pub struct PeaAssDrawMap<'a> {
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
    pub sub: &'a PeaSub,
}

impl<'a> PeaAssDrawMap<'a> {
    pub fn new(assv: &'a Vec<PeaAssVar>, sub: &'a PeaSub) -> Self {
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
        let pnt = sub.n1d_f.n1d_2_utm();
        x0 = x0.min(pnt.0);
        x1 = x1.max(pnt.0);
        y0 = y0.min(pnt.1);
        y1 = y1.max(pnt.1);

        let (ox, oy) = ((x1 + x0) * 0.5f32, (y1 + y0) * 0.5f32);
        let wd = x1 - x0;
        let (o_ld, _o_ln) = utm_latlong(ox, oy);
        let zm = meter_pixel_to_zoom_lat_2(wd, ww as u32, o_ld);
        let mtpx = zoom_to_meter_pixel_lat(zm, o_ld);

        let ex_x = mtpx * ww;
        let ex_y = mtpx * hh;
        let (o_x, o_y) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (xx, yy) = utm_latlong(o_x, o_y);
        let or_x = o_x - ex_x / 2.0;
        let or_y = o_y - ex_y / 2.0;
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
            sub,
        }
    }
}

pub fn load_ggmap(gm: &PeaAssDrawMap, basemap: &str, fpath: &str) {
    let frdrw = format!("{fpath}rw.jpeg");
    loop {
        let frdrw = frdrw.as_str();
        if !std::path::Path::new(frdrw).exists() {
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
            println!("{url}");
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

        use std::fmt::Write;
        let mut jsmp = String::new();
        writeln!(jsmp, "{{ \"map\": [").expect("?");

        let (t_x, t_y) = (0, 0);
        let fcx = map.ww / map.ex_x;
        let fcy = map.ww / map.ex_x;
        let mut img0 = RgbaImage::new(w, h);
        let mut fg = false;
        for a in map.assv.iter() {
            let pno = format!("หม้อแปลง: {}", a.peano);
            let pnt = a.n1d.n1d_2_utm();
            let (lat, lon) = a.n1d.n1d_2_latlon();
            let x = (pnt.0 - map.or_x) * fcx - map.ofs_x;
            let y = (pnt.1 - map.or_y) * fcy;
            let (ix, iy) = (x as i32 + t_x, h as i32 - y as i32 + t_y);
            let (fx, fy) = (ix as f32, iy as f32);
            draw_filled_circle_mut(&mut img0, (ix, iy), 8, Rgba([0, 180, 255, 255]));
            draw_line_segment_mut(
                &mut img0,
                (fx, fy - 10.0),
                (fx, fy + 10.0),
                Rgba([0, 180, 0, 255]),
            );
            draw_line_segment_mut(
                &mut img0,
                (fx - 10.0, fy),
                (fx + 10.0, fy),
                Rgba([0, 180, 0, 255]),
            );
            if fg {
                write!(jsmp, ", ").expect("?");
            }
            writeln!(
                jsmp,
                "{{ \"x\": {ix}, \"y\": {iy}, \"rad\": 10, \"name\": \"{pno}\", \"lat\": {lat}, \"lon\": {lon} }}"
            )
            .expect("?");
            fg = true;
        }

        let ll1 = map.sub.n1d_f.n1d_2_latlon();
        let ll2 = map.sub.n1d_s.n1d_2_latlon();
        let (lat, lon) = ll1;
        let (mdx, mdy) = (map.xx, map.yy);
        println!("SUB: {},{}  {},{}", ll1.0, ll1.1, ll2.0, ll2.1);

        println!("MID: {mdx},{mdy}");
        println!("t:{t_x},{t_y}  w:{w} h:{h} fac:{fcx},{fcy}");

        let pnt = map.sub.n1d_f.n1d_2_utm();
        let x = (pnt.0 - map.or_x) * fcx - map.ofs_x;
        let y = (pnt.1 - map.or_y) * fcy;
        let (ix, iy) = (x as i32 + t_x, h as i32 - y as i32 + t_y);
        let (fx, fy) = (ix as f32, iy as f32);
        draw_filled_circle_mut(&mut img0, (ix, iy), 20, Rgba([255, 150, 255, 255]));

        draw_line_segment_mut(
            &mut img0,
            (fx - 50.0, fy - 50.0),
            (fx + 50.0, fy + 50.0),
            Rgba([180, 180, 0, 255]),
        );
        draw_line_segment_mut(
            &mut img0,
            (fx - 50.0, fy + 50.0),
            (fx + 50.0, fy - 50.0),
            Rgba([180, 150, 0, 255]),
        );

        if fg {
            write!(jsmp, ", ").expect("?");
        }
        let sno = format!("สถานีไฟฟ้า: {}", map.sub.sbid);
        writeln!(
            jsmp,
            "{{ \"x\": {ix}, \"y\": {iy}, \"rad\": 20, \"name\": \"{sno}\", \"lat\": {lat}, \"lon\": {lon} }}"
        )
        .expect("?");

        let frd01 = format!("{}01.png", fpath);
        img0.save(&frd01).expect("?");

        let mut img1 = open(&frdcp).unwrap();
        let img2 = open(&frd01).unwrap();
        img1.blend(&img2, pixel_mult, true, false).unwrap();
        img1.save(frd02).expect("?");

        writeln!(jsmp, "]}}").expect("?");
        //println!("map: {jsmp}");
        //let v: Value = serde_json::from_str(jsmp.as_str()).expect("?");
        //println!("jsmp: {v:?}");
        let fmp02 = format!("{}02.json", fpath);
        std::fs::write(fmp02, jsmp).unwrap();
    }
}

pub fn make_img(fdid: &str, mptp: &str, f02: &str) -> Result<(), Box<dyn Error>> {
    let sbid = &fdid[0..3];
    let p02 = Path::new(&f02);
    let nm0 = (&f02[f02.len() - 9..f02.len() - 7]).to_string();
    println!("f02:{f02} nm0:{nm0}");
    let ppr = p02.parent().unwrap();
    println!("'{f02}' does not exist in {ppr:?}");
    if !ppr.exists() {
        std::fs::create_dir_all(ppr).expect("?");
    }
    let Ok(buf) = std::fs::read(format!("{DNM}/{sbid}-rw4.bin")) else {
        return Err("ERROR".into());
    };
    // ==== read rw3 data
    let Ok((assv0, _)): Result<(Vec<PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        return Err("ERROR".into());
    };
    let assv = assv0
        .iter()
        .filter(|a| a.fdid == fdid && a.own == "P")
        .cloned()
        .collect::<Vec<_>>();
    let Ok(buf) = std::fs::read(format!("{DNM}/{sbid}.bin")) else {
        return Err("ERROR".into());
    };
    let (peasb, _): (PeaSub, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    let map = PeaAssDrawMap::new(&assv, &peasb);
    let ppr0 = ppr.to_str().unwrap();
    //let fpath = format!("{}/{}-rd", ppr0, fdid);
    let fpath = format!("{ppr0}/{fdid}-{nm0}");
    println!("=== 1 : {p02:?} exists : {}", p02.exists());
    load_ggmap(&map, mptp, fpath.as_str());
    draw_ass_map(&map, fpath.as_str());
    println!("=== 2 : {p02:?} exists : {}", p02.exists());
    Ok(())
}

pub fn get_img(fdid: &str, mptp: &str, f02: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let p02 = Path::new(&f02);
    if !p02.exists() {
        make_img(fdid, mptp, f02)?;
    }
    if p02.exists() {
        let mut f = File::open(p02).expect("no file found");
        let metadata = fs::metadata(p02).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read_exact(&mut buffer).expect("buffer overflow");
        return Ok(buffer);
    }
    Err("ERROR".into())
}

pub fn get_map(fdid: &str, mptp: &str, f02: &str) -> Result<Value, Box<dyn Error>> {
    let p02 = Path::new(&f02);
    if !p02.exists() {
        make_img(fdid, mptp, f02)?;
    }
    if p02.exists() {
        let mp = std::fs::read_to_string(p02).expect("?");
        let v: Value = serde_json::from_str(mp.as_str()).expect("?");
        return Ok(v);
    }
    Err("ERROR".into())
}

pub async fn get_image(para: Query<QueryParams>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    let Some(fdid) = &para.fdid else {
        return (headers, Vec::<u8>::new());
    };

    let f02 = format!("{DNM}/fdimg1/{fdid}-rd02.jpeg");
    let img = get_img(fdid.as_str(), "roadmap", f02.as_str()).unwrap_or_default();
    /*
    let m02 = format!("{DNM}/fdimg1/{fdid}-rd02.sgmp");
    let map = get_map(fdid.as_str(), "roadmap", m02.as_str()).unwrap_or_default();
    println!("map: {map}");
    */

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
    (headers, img)
}
