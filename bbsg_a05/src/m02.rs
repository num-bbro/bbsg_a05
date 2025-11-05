use crate::dcl::Pan;
use crate::utl::get_pea00;
use crate::utl::get_sbrw00;
use askama::Template;
use askama_web::WebTemplate;
//use crate::c04::Pea;
use crate::dcl::PeaAssVar;
use crate::dcl::VarType;

pub struct WebTempRow {
    sbid: String,
    pvid: String,
    unba: f32, // UnbalPowLossKw
    ovld: f32, // CntTrSatLoss
    povl: f32, // CntTrUnbalLoss
    smal: f32, // SmallSell
}

#[derive(Template, WebTemplate)]
#[template(path = "m02.html")]
pub struct WebTemp {
    name: String,
    rows: Vec<WebTempRow>,
}

pub async fn m02() -> WebTemp {
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
    /*
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
    */
    for (_i, sr) in sbrw.iter().enumerate() {
        let note = format!("{}", sr.v[VarType::TakeNote as usize].v);
        if note != "1" {
            continue;
        }
        if sr.sbid == "SQB" {
            continue;
        }
        /*
        let evrt = evcs[i];

        //use fast_math;
        let mut evgr = get_scurv();
        for ev in evgr.iter_mut() {
            *ev *= evrt * EV_AT_2050;
            *ev *= 0.007;
            *ev *= 365.0 * EV_HR_DAY;
        }
        */
        let unba = sr.v[VarType::MaxPosPowSub as usize].v;
        let ovld = sr.v[VarType::MaxNegPowSub as usize].v;
        let povl = sr.v[VarType::VsppMv as usize].v;
        let smal = sr.v[VarType::SolarEnergy as usize].v;

        let rw = WebTempRow {
            sbid: sr.sbid.to_string(),
            pvid: sr.pvid.to_string(),
            unba,
            ovld,
            povl,
            smal,
        };
        rows.push(rw);
    }

    WebTemp {
        name: "MV SOLAR POWER - m02".to_string(),
        rows,
    }
}
