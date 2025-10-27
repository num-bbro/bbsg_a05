use crate::dcl::Pan;
use crate::dcl::PeaAssVar;
use crate::dcl::VarType;
use crate::dcl::EV_AT_2050;
use crate::dcl::EV_HR_DAY;
use crate::dcl::EV_MWH_BAHT;
use crate::dcl::SSHOW_YEAR_BEG;
use crate::dcl::SSHOW_YEAR_END;
use crate::utl::get_pea00;
use crate::utl::get_sbrw00;
use crate::utl::get_scurv;
use askama::Template;
use askama_web::WebTemplate;

pub struct WebTempRow {
    sbid: String,
    pvid: String,
    note: String,
    evrt: f32,
    evgr: Vec<f32>,
}

#[derive(Template, WebTemplate)]
#[template(path = "p05.html")]
pub struct WebTemp {
    name: String,
    rows: Vec<WebTempRow>,
}

pub async fn p05() -> WebTemp {
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
    for (i, sr) in sbrw.iter().enumerate() {
        let note = format!("{}", sr.v[VarType::TakeNote as usize].v);
        if note != "1" {
            continue;
        }
        if sr.sbid == "SQB" {
            continue;
        }
        let evrt = evcs[i];

        //use fast_math;
        let mut evgr = get_scurv();
        for ev in evgr.iter_mut() {
            *ev *= evrt * EV_AT_2050;
            *ev *= 0.007;
            *ev *= 365.0 * EV_HR_DAY;
            *ev *= EV_MWH_BAHT;
        }
        let rw = WebTempRow {
            sbid: sr.sbid.to_string(),
            pvid: sr.pvid.to_string(),
            note,
            evrt,
            evgr,
        };
        rows.push(rw);
    }

    WebTemp {
        name: "Year Baht for Charge EV - p04".to_string(),
        rows,
    }
}
