use crate::dcl::AojObj;
use crate::dcl::ProcEngine;
use crate::dcl::SubAssObj;
use crate::utl::get_tr_den;
use crate::utl::get_tr_sorf;
use crate::utl::get_tr_volta;
use crate::utl::get_tr_zone;
use crate::utl::mon_kwh_2_kw;
use crate::utl::p01_chk;
use crate::utl::trf_kva_2_kw;
use regex::Regex;
use sglab02_lib::sg::gis1::ar_list;
use sglib04::geo4::GPPS;
use std::collections::HashMap;
use std::error::Error;

pub fn p05_ana_1() -> Result<(), Box<dyn Error>> {
    let smrt = Regex::new(r"[12].*").unwrap();
    let now = std::time::SystemTime::now();
    let _gpp = &GPPS;
    let g0 = ProcEngine::prep1();

    let mut sub_sele = Vec::<SubAssObj>::new();
    let mut mt_type = HashMap::<String, usize>::new();
    let mut db_sub = HashMap::<String, String>::new();
    let subhs = p01_chk();
    for ar in ar_list() {
        println!("{ar} {}", now.elapsed().unwrap().as_secs());
        let eg = ProcEngine::prep2(ar);
        let mut ca_rg = vec![0f32; eg.subs.len()];
        for (si, sft) in eg.subs.iter().enumerate() {
            let sbid = sft.sbid.to_string();
            let sb = &sft.sbid;
            if let Some(a) = db_sub.get(sb) {
                println!("DBL SUB {sb} - {a} == {ar}");
            } else {
                db_sub.insert(sb.to_string(), ar.to_string());
            }
            let note = if subhs.contains(sb) { 1 } else { 0 };
            let mut mx21 = 0.0f32;
            let mut mx22 = 0.0f32;
            let mut mx23 = 0.0f32;
            let mut mx24 = 0.0f32;

            if let Some(slp) = g0.lp23.get(&sbid) {
                if let Some(vs) = &slp.neg_rep.val {
                    for v in vs.iter().flatten() {
                        mx21 = mx21.max(*v);
                    }
                }
            }
            if let Some(slp) = g0.lp24.get(&sbid) {
                if let Some(vs) = &slp.neg_rep.val {
                    for v in vs.iter().flatten() {
                        mx22 = mx22.max(*v);
                    }
                }
            }

            if let Some(slp) = g0.lp23.get(&sbid) {
                if let Some(vs) = &slp.pos_rep.val {
                    for v in vs.iter().flatten() {
                        mx23 = mx23.max(*v);
                    }
                }
            }
            if let Some(slp) = g0.lp24.get(&sbid) {
                if let Some(vs) = &slp.pos_rep.val {
                    for v in vs.iter().flatten() {
                        mx24 = mx24.max(*v);
                    }
                }
            }

            let Some(sf) = g0.sbif.get(&sft.sbid) else {
                continue;
            };
            let cpmw = sf.mvxn as f32;
            let pv = g0.sb2pv(&sft.sbid);
            let sbid = sb.to_string();
            let sbth = sf.name.to_string();
            let sben = sf.enam.to_string();
            let prov = pv.to_string();
            let ev_pc = g0.evpv.get(&pv).unwrap().ev_pc;
            let evca = ev_pc * 3.0;
            ca_rg[si] = ev_pc;
            let Some(gpp) = GPPS.get(&pv) else {
                continue;
            };
            let gpp = *gpp as f32;
            let arid = ar.to_string();
            let mut sbas = SubAssObj {
                sbid,
                sbth,
                sben,
                prov,
                arid,
                evca,
                gpp,
                mx21,
                mx22,
                mx23,
                mx24,
                cpmw,
                note,
                ..Default::default()
            };
            let vsp = &eg.vssb[si];
            if !vsp.is_empty() {
                for pi in vsp {
                    let pp = &eg.vsps[*pi];
                    let Some(kw) = pp.kw else {
                        continue;
                    };
                    sbas.vspkw += kw;
                }
            }
            let spp = &eg.spsb[si];
            if !spp.is_empty() {
                for pi in spp {
                    let pp = &eg.spps[*pi];
                    let Some(mw) = pp.mw else {
                        continue;
                    };
                    sbas.sppmw += mw;
                }
            }
            let repl = &eg.resb[si];
            if !repl.is_empty() {
                for pi in repl {
                    let pp = &eg.repl[*pi];
                    let Some(pwmw) = pp.pwmw else {
                        continue;
                    };
                    sbas.repln += pwmw;
                }
            }
            println!("{sb} -> {pv}");
            let mut aoj_tr = HashMap::<usize, usize>::new();
            for tis in sft.feed.values() {
                for ti in tis {
                    // loop of transf
                    let tr = &eg.ctrs[*ti];
                    let aotr = &eg.aotr[*ti];
                    for ai in aotr {
                        let ai = *ai;
                        if let Some(cn) = aoj_tr.get_mut(&ai) {
                            *cn += 1;
                        } else {
                            aoj_tr.insert(ai, 1);
                        }
                    }
                    let dnk = get_tr_den(*ti, &eg);
                    let znk = get_tr_zone(*ti, &eg);
                    let sok = get_tr_sorf(*ti, &eg);
                    sbas.dens += dnk;
                    sbas.zone += znk;
                    sbas.sorf += sok;
                    let tcm = &eg.cmts[tr.ix];
                    let Some(ow) = &tcm.tr_own else {
                        continue;
                    };
                    if ow == "PEA" {
                        sbas.trpe += 1;
                    } else {
                        sbas.trcu += 1;
                    }
                    let (vopw, vose) = get_tr_volta(*ti, &eg);
                    sbas.vopw += vopw;
                    sbas.vose += vose;
                    let (mut se_s, mut se_l, mut sell, mut se_2) = (0.0, 0.0, 0.0, 0.0);
                    let (mut se_a, mut se_b, mut se_c) = (0.0, 0.0, 0.0);
                    for mi in &tr.mts {
                        // loop meter
                        let bl = &eg.m2bs[*mi];
                        if bl.is_empty() {
                            continue;
                        }
                        let mb = &eg.bils[bl[0]];
                        let (mut am1, mut am2) = (0.0, 0.0);
                        if let Some(cn) = mt_type.get_mut(&mb.rate) {
                            *cn += 1;
                        } else {
                            mt_type.insert(mb.rate.to_string(), 1);
                        }
                        if smrt.captures(mb.rate.as_str()).is_some() && mb.main.is_empty() {
                            sbas.mt13 += 1;
                            am1 = mb.kwh18;
                        } else {
                            sbas.mt45 += 1;
                            am2 = mb.kwh18;
                        }
                        se_s += am1;
                        se_l += am2;
                        sell += am1 + am2;
                        se_2 += if (am1 + am2) > 200.0 { am1 + am2 } else { 0.0 };
                        if ow == "PEA" {
                            sbas.mtpe += 1;
                        } else {
                            sbas.mtcu += 1;
                        }
                        let mt = &eg.cmts[*mi];
                        let Some(ref phs) = mt.mt_phs else {
                            continue;
                        };
                        let phs = phs.to_string();
                        match phs.as_str() {
                            "A" => se_a += am1 + am2,
                            "B" => se_b += am1 + am2,
                            "C" => se_c += am1 + am2,
                            _ => {}
                        }
                    } // end meter
                    let se_p = se_a + se_b + se_c;
                    if se_a < se_p && se_b < se_p && se_c < se_p {
                        let ab = (se_a - se_b).abs();
                        let bc = (se_b - se_c).abs();
                        let ca = (se_c - se_a).abs();
                        sbas.unbal += (ab + bc + ca) * 0.5;
                    }
                    let Some(kv) = &tcm.tr_kva else {
                        continue;
                    };
                    if *kv == 0.0 {
                        continue;
                    }
                    let kw = trf_kva_2_kw(*kv);
                    if kw == 0.0 {
                        println!("======================kw {kv:?} ==================");
                    }
                    let psat = mon_kwh_2_kw(sell) / kw;
                    if psat.is_nan() {
                        println!("======================Nan=={sell} ================");
                    }
                    sbas.sell += sell;
                    sbas.se_s += se_s;
                    sbas.se_l += se_l;
                    sbas.se_2 += se_2;
                    sbas.psat += psat;
                } // end trans
            }
            let mut aojs: Vec<(usize, usize)> = aoj_tr.into_iter().map(|(k, v)| (v, k)).collect();
            aojs.sort_by(|a, b| b.0.cmp(&a.0));
            let mut aojv = Vec::<AojObj>::new();
            for (v, ai) in aojs {
                let ao = &eg.aojs[ai];
                let Some(ref code) = ao.code else {
                    continue;
                };
                let Some(ref sht_name) = ao.sht_name else {
                    continue;
                };
                let Some(ref office) = ao.office else {
                    continue;
                };
                let Some(ref pea) = ao.pea else {
                    continue;
                };
                let Some(ref aoj_sz) = ao.aoj_sz else {
                    continue;
                };
                let Some(ref reg) = ao.reg else {
                    continue;
                };
                let Some(ref name) = ao.name else {
                    continue;
                };
                let Some(ref level) = ao.level else {
                    continue;
                };
                let trcn = v;
                //aojv.push((eg.aojs[ai].clone(), v));
                let aoj = AojObj {
                    code: code.to_string(),
                    sht_name: sht_name.to_string(),
                    office: office.to_string(),
                    pea: pea.to_string(),
                    aoj_sz: aoj_sz.to_string(),
                    reg: reg.to_string(),
                    name: name.to_string(),
                    level: *level,
                    trcn,
                };
                aojv.push(aoj);
            }
            sbas.aojv = aojv;
            sub_sele.push(sbas);
        } // end sub loop
    } // end area loop
    let cfg = bincode::config::standard();
    let bin: Vec<u8> = bincode::encode_to_vec(&sub_sele, cfg).unwrap();
    let fnm = "/mnt/e/CHMBACK/pea-data/data2/p05_ana_1.bin";
    std::fs::write(fnm, bin).unwrap();
    println!("write to {fnm} - {}", sub_sele.len());
    Ok(())
}
