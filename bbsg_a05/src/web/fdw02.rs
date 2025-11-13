use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use serde::Deserialize;
//use serde_json::Value;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub fdid: Option<String>,
}

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "fdw02.html")]
pub struct WebTemp {
    fdid: String,
    name: String,
    json: Vec<(i32, i32, i32, String, f32, f32)>,
}

use crate::dcl::DNM;
use crate::img::fda01::get_map;

pub async fn page(para: Query<Param>) -> WebTemp {
    let Some(ref fdid) = para.fdid else {
        return WebTemp::default();
    };
    let fdid = fdid.to_string();
    let name = format!("MAP {fdid}");
    let m02 = format!("{DNM}/fdimg1/{fdid}-sa02.json");
    let mut json = Vec::<_>::new();
    let jsn = get_map(fdid.as_str(), "satellite", m02.as_str()).unwrap_or_default();
    if let Some(rows) = jsn["map"].as_array() {
        for row in rows.iter() {
            let x = row["x"].as_i64().unwrap() as i32;
            let y = row["y"].as_i64().unwrap() as i32;
            let rad = row["rad"].as_i64().unwrap() as i32;
            let name = row["name"].as_str().unwrap().to_string();
            let lat = row["lat"].as_f64().unwrap() as f32;
            let lon = row["lon"].as_f64().unwrap() as f32;
            //println!("{lat}, {lon} {rad} {name}");
            json.push((x, y, rad, name, lat, lon));
        }
    }
    WebTemp { fdid, json, name }
}
