use crate::p01::Pan;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use serde::Deserialize;
use sglib05::c04::PeaAssVar;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub sbid: Option<String>,
}

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "tr01.html")]
pub struct WebTemp {
    sbid: String,
    name: String,
    assv: Vec<PeaAssVar>,
    norv: Vec<PeaAssVar>,
    maxv: Vec<PeaAssVar>,
    evs: Vec<PeaAssVar>,
}
use sglib05::c04::VarType;
use sglib05::c04::DNM;
use sglib05::c04::WE_EV;

pub async fn tr01(para: Query<Param>) -> WebTemp {
    let Some(ref sbid) = para.sbid else {
        println!("NO SBID");
        return WebTemp::default();
    };
    println!("para:{sbid:?}");
    // ============================
    // ==== read rw3 data
    let Ok(buf) = std::fs::read(format!("{DNM}/{sbid}-rw2.bin")) else {
        println!("NO rw3.bin file: {sbid}");
        return WebTemp::default();
    };
    // ==== read rw3 data
    let Ok((mut assv, _)): Result<(Vec<sglib05::c04::PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode rw3: {sbid}");
        return WebTemp::default();
    };

    // ============================
    // ==== read nor data
    let Ok(buf) = std::fs::read(format!("{DNM}/{sbid}-nor.bin")) else {
        println!("NO nor.bin file: {sbid}");
        return WebTemp::default();
    };
    // ==== decode nor data
    let Ok((mut norv, _)): Result<(Vec<sglib05::c04::PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode nor: {sbid}");
        return WebTemp::default();
    };

    // ============================
    // ==== read max data
    let Ok(buf) = std::fs::read(format!("{DNM}/pea-mx.bin")) else {
        println!("NO nor.bin file: {sbid}");
        return WebTemp::default();
    };
    // ==== decode max data
    let Ok((maxv, _)): Result<(Vec<sglib05::c04::PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode max: {sbid}");
        return WebTemp::default();
    };

    // ============================
    // ==== read evs data
    let Ok(buf) = std::fs::read(format!("{DNM}/{sbid}-ev.bin")) else {
        println!("NO ev.bin file: {sbid}");
        return WebTemp::default();
    };
    // ==== decode nor data
    let Ok((mut evs, _)): Result<(Vec<sglib05::c04::PeaAssVar>, usize), _> =
        bincode::decode_from_slice(&buf[..], bincode::config::standard())
    else {
        println!("Failed to decode ev: {sbid}");
        return WebTemp::default();
    };

    assv.sort_by(|b, a| {
        a.v[VarType::NewCarRegVp01.tousz()]
            .v
            .partial_cmp(&b.v[VarType::NewCarRegVp01.tousz()].v)
            .unwrap()
    });
    norv.sort_by(|b, a| {
        a.v[VarType::NewCarRegVp01.tousz()]
            .v
            .partial_cmp(&b.v[VarType::NewCarRegVp01.tousz()].v)
            .unwrap()
    });
    evs.sort_by(|b, a| {
        a.v[VarType::NewCarRegVp01.tousz()]
            .v
            .partial_cmp(&b.v[VarType::NewCarRegVp01.tousz()].v)
            .unwrap()
    });

    WebTemp {
        sbid: sbid.to_string(),
        name: "tr01".to_string(),
        assv,
        norv,
        maxv,
        evs,
    }
}
