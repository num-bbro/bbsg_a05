use axum::extract::Query;
use axum::http::header::{self, HeaderMap};
use axum::response::IntoResponse;
use serde::Deserialize;
pub const MP_WW: f32 = 1800_f32;
pub const MP_HH: f32 = 1733_f32 - 185_f32 * 2.0;
pub const MP_MG: u32 = 72;
pub const MP_UPDW: u32 = 185;

#[derive(Deserialize)]
pub struct QueryParams {
    pub sbid: Option<String>,
    pub load: Option<String>,
}

use crate::web::fdw03::LD24DIR;
use std::fs;
use std::fs::File;
use std::io::Read;

pub async fn get_image(para: Query<QueryParams>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    let Some(sbid) = &para.sbid else {
        return (headers, Vec::<u8>::new());
    };
    let Some(load) = &para.load else {
        return (headers, Vec::<u8>::new());
    };
    let fimg = format!("{LD24DIR}/{sbid}/{load}.jpg");
    //println!("sbid:{sbid} load:{load} dir:{LD24DIR} :{fimg}");

    let mut f = File::open(&fimg).expect("no file found");
    let metadata = fs::metadata(&fimg).expect("unable to read metadata");
    let mut img = vec![0; metadata.len() as usize];
    f.read_exact(&mut img).expect("buffer overflow");

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
    (headers, img)
}
