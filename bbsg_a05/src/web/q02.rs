use crate::dcl::Pan;
use askama::Template;
use askama_web::WebTemplate;

pub struct WebTempRow {
    sbid: String,
    pvid: String,
    pwmx: f32,
    pwrt: f32,
    regr: Vec<f32>,
}

#[derive(Template, WebTemplate)]
#[template(path = "q02.html")]
pub struct WebTemp {
    name: String,
    rows: Vec<WebTempRow>,
}

use crate::dcl::PeaAssVar;
use crate::dcl::VarType;
use crate::dcl::SSHOW_YEAR_BEG;
use crate::dcl::SSHOW_YEAR_END;
use crate::utl::get_pea00;
use crate::utl::get_sbrw00;
use crate::utl::trf_kva_2_kw;

pub const RE_MV2HV_RATIO: f32 = 0.0986;
use crate::utl::get_scurv_re;

pub async fn q02() -> WebTemp {
    let sbrw = get_sbrw00();
    let pea = get_pea00();
    let mut aids: Vec<_> = pea.aream.keys().collect();
    aids.sort();
    for aid in aids {
        let Some(ar) = pea.aream.get(aid) else {
            continue;
        };
        let mut pids: Vec<_> = ar.provm.keys().collect();
        pids.sort();
        for pid in pids {
            let Some(prov) = ar.provm.get(pid) else {
                continue;
            };
            let mut pvas = PeaAssVar::from(0u64);
            pvas.arid = aid.to_string();
            pvas.pvid = pid.to_string();
            //println!("  pv:{pid}");
            let mut sids: Vec<_> = prov.subm.keys().collect();
            sids.sort();
            for sid in sids {
                let Some(_sb) = prov.subm.get(sid) else {
                    continue;
                };
                //println!("sb:{sid}");
            }
        }
    }
    let mut rows = Vec::<WebTempRow>::new();
    let mut evcs = vec![0f32; sbrw.len()];
    let evs: f32 = sbrw
        .iter()
        .map(|sr| sr.v[VarType::NoHmChgEvTr as usize].v)
        .sum();
    //println!("evs: {evs} {}", sbrw.len());
    for (i, sr) in sbrw.iter().enumerate() {
        //println!("  {}", evcs.len());
        evcs[i] = sr.v[VarType::NoHmChgEvTr as usize].v / evs;
    }
    for (_i, sr) in sbrw.iter().enumerate() {
        let note = format!("{}", sr.v[VarType::TakeNote as usize].v);
        if note != "1" {
            continue;
        }
        if sr.sbid == "SQB" {
            continue;
        }
        let pwmx = sr.v[VarType::SubPowCap as usize].v;
        let pwmx = trf_kva_2_kw(pwmx);
        //let pwrt = RE_MV2HV_RATIO * 0.5f32;
        let pwrt = RE_MV2HV_RATIO;

        //use fast_math;
        let mut regr = get_scurv_re();
        for re in regr.iter_mut() {
            *re *= pwrt * pwmx;
        }
        let rw = WebTempRow {
            sbid: sr.sbid.to_string(),
            pvid: sr.pvid.to_string(),
            pwmx,
            pwrt,
            regr,
        };
        rows.push(rw);
    }

    WebTemp {
        name: "Solar Energy % - q02".to_string(),
        rows,
    }
}
