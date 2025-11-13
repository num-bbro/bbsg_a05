use axum::extract::Query;
use axum::http::header::{self, HeaderMap};
use axum::response::IntoResponse;
use serde::Deserialize;
pub const MP_WW: f32 = 1800_f32;
pub const MP_HH: f32 = 1733_f32 - 185_f32 * 2.0;
pub const MP_MG: u32 = 72;
pub const MP_UPDW: u32 = 185;
use crate::dcl::DNM;
use crate::img::fda01::get_img;

#[derive(Deserialize)]
pub struct QueryParams {
    pub fld: Option<String>,
    pub sbid: Option<String>,
    pub fdid: Option<String>,
}

pub async fn get_image(para: Query<QueryParams>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    let Some(fdid) = &para.fdid else {
        return (headers, Vec::<u8>::new());
    };

    let f02 = format!("{DNM}/fdimg1/{fdid}-sa02.jpeg");
    let img = get_img(fdid.as_str(), "satellite", f02.as_str()).unwrap_or_default();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
    (headers, img)
}
