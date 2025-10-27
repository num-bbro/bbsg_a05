use std::error::Error;

//use askama::Template;
//use askama_web::WebTemplate;
use axum::routing::get;

/*
#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

async fn hello() -> HelloTemplate {
    HelloTemplate {
        name: "world".to_string(),
    }
}
*/

pub trait Pan {
    fn san(v: &str) -> String;
    fn pan0(&self) -> String;
    fn pan2(&self) -> String;
    fn pan3(&self) -> String;
}

impl Pan for f32 {
    fn san(v: &str) -> String {
        let v = v
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",");
        v
    }
    fn pan0(&self) -> String {
        let v = format!("{self:.2}");
        let f = v[..v.len() - 3].to_string();
        let v = Self::san(&f);
        v.to_string()
    }
    fn pan2(&self) -> String {
        let v = format!("{self:.2}");
        let n = v[v.len() - 3..].to_string();
        let f = v[..v.len() - 3].to_string();
        let v = Self::san(&f);
        format!("{v}{n}")
    }
    fn pan3(&self) -> String {
        let v = format!("{self:.3}");
        let n = v[v.len() - 4..].to_string();
        let f = v[..v.len() - 4].to_string();
        let v = Self::san(&f);
        format!("{v}{n}")
    }
}
use sglib05::c04::Pea;
use sglib05::c04::PeaAssVar;
use std::sync::OnceLock;

pub const SCURV_YEAR_BEG: usize = 2021;
pub const SSHOW_YEAR_BEG: usize = 2025;
pub const SSHOW_YEAR_END: usize = 2039;
pub const EV_AT_2050: f32 = 6_000_000f32;
pub const EV_HR_DAY: f32 = 3.0;
pub const EV_MWH_BAHT: f32 = 1000f32;
pub const RE_SCURV_BEG: usize = 2018;

pub fn get_scurv() -> Vec<f32> {
    let mut curv = Vec::<f32>::new();
    for y in SSHOW_YEAR_BEG..=SSHOW_YEAR_END {
        let a = (y - SCURV_YEAR_BEG) as f32;
        let b = a - 14f32;
        //let c = b * 0.3f32;
        let c = b * 0.41f32;
        //let d = c + 0.0f32;
        let d = c + 1.205f32;
        let d = -d;
        let e = d.exp();
        let f = 1f32 / (1f32 + e);
        //let g = f.powf(1f32);
        let g = f.powf(1.1f32);
        curv.push(g);
    }
    curv
}

pub fn get_scurv_re() -> Vec<f32> {
    let mut curv = Vec::<f32>::new();
    for y in SSHOW_YEAR_BEG..=SSHOW_YEAR_END {
        let a = (y - RE_SCURV_BEG) as f32;
        let b = a - 14f32;
        let c = b * 0.3f32;
        //let c = b * 0.41f32;
        let d = c + 0.0f32;
        //let d = c + 1.205f32;
        let d = -d;
        let e = d.exp();
        let f = 1f32 / (1f32 + e);
        let g = f.powf(1f32);
        //let g = f.powf(1.1f32);
        curv.push(g);
    }
    curv
}

pub static SBRW00: OnceLock<Vec<PeaAssVar>> = OnceLock::new();
pub fn get_sbrw00() -> &'static Vec<PeaAssVar> {
    SBRW00.get_or_init(sbrw00_init)
}
fn sbrw00_init() -> Vec<PeaAssVar> {
    let dnm = "/mnt/e/CHMBACK/pea-data/c01_pea";
    let buf = std::fs::read(format!("{dnm}/000-sbrw.bin")).unwrap();
    let (ass, _): (Vec<PeaAssVar>, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    ass
}

pub static PEA00: OnceLock<Pea> = OnceLock::new();
pub fn get_pea00() -> &'static Pea {
    PEA00.get_or_init(pea00_init)
}
fn pea00_init() -> Pea {
    let dnm = "/mnt/e/CHMBACK/pea-data/c01_pea";
    let buf = std::fs::read(format!("{dnm}/000_pea.bin")).unwrap();
    let (pea, _): (Pea, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    println!("pea: {}", pea.aream.len());
    pea
}

pub static PVRW00: OnceLock<Vec<PeaAssVar>> = OnceLock::new();
pub fn get_pvrw00() -> &'static Vec<PeaAssVar> {
    PVRW00.get_or_init(pvrw00_init)
}
fn pvrw00_init() -> Vec<PeaAssVar> {
    let dnm = "/mnt/e/CHMBACK/pea-data/c01_pea";
    let buf = std::fs::read(format!("{dnm}/000-pvrw.bin")).unwrap();
    let (ass, _): (Vec<PeaAssVar>, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    println!("pv: {}", ass.len());
    ass
}

pub async fn run1() -> Result<(), Box<dyn Error>> {
    println!("run1");
    let app = axum::Router::new()
        // sub
        .route("/sba01", get(crate::sba01::sba01))
        .route("/sba02", get(crate::sba02::sba02))
        // sub
        .route("/sb01", get(crate::sb01::sb01))
        .route("/sb02", get(crate::sb02::sb02))
        .route("/sb03", get(crate::sb03::sb03))
        .route("/sb04", get(crate::sb04::sb04))
        .route("/sb05", get(crate::sb05::sb05))
        // trans
        .route("/tr01", get(crate::tr01::tr01))
        .route("/tr02", get(crate::tr02::tr02))
        .route("/tr03", get(crate::tr03::tr03))
        .route("/tr04", get(crate::tr04::tr04))
        .route("/tr05", get(crate::tr05::tr05))
        .route("/tr06", get(crate::tr06::tr06))
        // ___
        .route("/a01", get(crate::a01::a01))
        .route("/a02", get(crate::a02::a02))
        .route("/a03", get(crate::a03::a03))
        .route("/q02", get(crate::q02::q02))
        .route("/p02", get(crate::p02::p02))
        .route("/p03", get(crate::p03::p03))
        .route("/p04", get(crate::p04::p04))
        .route("/p05", get(crate::p05::p05))
        .route("/p06", get(crate::p06::p06))
        .route("/p07", get(crate::p07::p07))
        .route("/p08", get(crate::p08::p08))
        .route("/m01", get(crate::m01::m01))
        .route("/m02", get(crate::m02::m02))
        .route("/", get(crate::p02::p02));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
