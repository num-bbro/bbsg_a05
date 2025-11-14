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
#[template(path = "fdw03.html")]
pub struct WebTemp {
    sbid: String,
    name: String,
    loads: Vec<String>,
}

pub const LD24DIR: &str = "/mnt/e/CHMBACK/pea-data/sbdrlp/2024";
//use crate::dcl::DNM;
//use crate::img::fda01::get_map;

pub async fn page(para: Query<Param>) -> WebTemp {
    let Some(ref fdid) = para.fdid else {
        return WebTemp::default();
    };
    //let sbid = format!("{}", &fdid[0..3]);
    let sbid = (fdid[0..3]).to_string();
    let dnm = format!("{LD24DIR}/{sbid}");
    //println!("dnm: {dnm}");
    let fdid = fdid.to_string();
    let name = format!("MAP {fdid}");

    let paths = std::fs::read_dir(dnm).unwrap();
    let mut loads = Vec::<String>::new();
    for path in paths.flatten() {
        let path = path.path();
        //let nm = path.path().file_stem().unwrap().display().to_string();
        //let nm = path.file_stem().unwrap();
        let nm = path.file_stem().unwrap().display().to_string();
        //let ex = path.extension().unwrap().display().to_string();
        //let ld = path.path().display();
        //loads.push(ld.to_string());
        //println!("LD: {nm} {ex}");
        loads.push(nm);
        //println!("Name: {}", path.unwrap().path().display())
    }
    loads.sort();
    WebTemp { sbid, name, loads }
}
