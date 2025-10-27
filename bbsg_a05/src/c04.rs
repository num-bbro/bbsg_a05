use crate::dcl::*;
use crate::p01::get_tr_volta;
use crate::p01::trf_kva_2_kw;
use crate::p01::AojObj;
use crate::p01::ProcEngine;
use regex::Regex;
use sglib04::geo4::GPPS;
use std::collections::HashMap;
use std::error::Error;

///
/// g0 = ProcEngine::prep1();
///
/// evpv: p13_ev_distr(&EV_PRV_ADJ_1)
///
/// sbif: sub_inf()
///
/// lp23: p03_load_lp("2023")
///
/// lp24: p03_load_lp("2024")
///
/// for (sb, sf) in &g0.sbif {
///
///   for id in &aids {
///
/// dnm = "/mnt/e/CHMBACK/pea-data/c01_pea";
///
/// bin: `Vec<u8>` = bincode::encode_to_vec(&pea, bincode::config::standard())
///
/// write(format!("{dnm}/000_pea.bin"), bin)
///
/// bin: `Vec<u8>` = bincode::encode_to_vec(&sb, bincode::config::standard())
///
/// std::fs::write(format!("{dnm}/{}.bin", sb.sbid), bin)
///
pub fn stage_01() -> Result<(), Box<dyn Error>> {
    //let dnm = "/mnt/e/CHMBACK/pea-data/c01_pea";
    let g0 = ProcEngine::prep5();
    let pea = c01_chk_01_01(DNM, &g0)?;
    c01_chk_01_02(&pea, DNM, &g0)?;

    let bin: Vec<u8> = bincode::encode_to_vec(&pea, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000_pea.bin"), bin).unwrap();
    println!("write2 to 000_pea.bin");

    Ok(())
}

pub fn c01_chk_01_01(dnm: &str, g0: &ProcEngine) -> Result<Pea, Box<dyn Error>> {
    std::fs::create_dir_all(dnm)?;
    let mut pea = Pea::default();
    for (sb, sf) in &g0.sbif {
        let ar = sf.arid.to_string();
        let ar_e = pea.aream.entry(ar).or_insert_with(|| PeaArea {
            arid: sf.arid.to_string(),
            ..Default::default()
        });
        let pv_e = ar_e
            .provm
            .entry(sf.prov.to_string())
            .or_insert_with(|| PeaProv {
                pvnm: sf.prov.to_string(),
                ..Default::default()
            });

        let Some(ev) = g0.evpv.get(&sf.prov) else {
            continue;
        };
        let Some(gpp) = GPPS.get(&sf.prov) else {
            continue;
        };
        pv_e.evpc = ev.ev_pc;
        pv_e.gppv = *gpp as f32;

        let _sb_e = pv_e.subm.entry(sb.to_string()).or_insert_with(|| PeaSub {
            sbid: sb.to_string(),
            name: sf.name.clone(),
            enam: sf.enam.clone(),
            area: sf.area.clone(),
            arid: sf.arid.clone(),
            volt: sf.volt.clone(),
            cate: sf.cate.clone(),
            egat: sf.egat.clone(),
            state: sf.state.clone(),
            conf: sf.conf.clone(),
            trax: sf.trax.clone(),
            mvax: sf.mvax.clone(),
            feed: sf.feed.clone(),
            feno: sf.feno,
            feeders: sf.feeders.clone(),
            trxn: sf.trxn,
            mvxn: sf.mvxn,
            prov: sf.prov.clone(),
            ..Default::default()
        });
    }

    let bin: Vec<u8> = bincode::encode_to_vec(&pea, bincode::config::standard()).unwrap();
    std::fs::write(format!("{dnm}/000_pea.bin"), bin).unwrap();
    println!("write to 000_pea.bin");

    Ok(pea)
}

pub fn c01_chk_01_02(pea: &Pea, dnm: &str, g0: &ProcEngine) -> Result<(), Box<dyn Error>> {
    let smrt = Regex::new(r"[12].*").unwrap();
    //let aream = pea.aream.clone();
    let mut aids: Vec<_> = pea.aream.keys().collect();
    aids.sort();

    let mut aoj_sbv = HashMap::<String, Vec<String>>::new();
    for id in &aids {
        println!("ar:{id}");
        let id = id.to_string();
        let eg = ProcEngine::prep2(&id);
        let Some(ar) = pea.aream.get(&id) else {
            continue;
        };
        //let provm = ar.provm.clone();
        let mut pids: Vec<_> = ar.provm.keys().collect();
        pids.sort();
        for id in &pids {
            let pid = id.to_string();
            let Some(pr) = ar.provm.get(&pid) else {
                continue;
            };
            //////////////////////////////////////////////
            //////////////////////////////////////////////
            //////////////////////////////////////////////
            // province
            // Car registration
            let Some(ev) = g0.evpv.get(&pid) else {
                continue;
            };
            let Some(gpp) = GPPS.get(&pid) else {
                continue;
            };
            println!("  p:{id}");
            println!("    ev rt: {}", ev.ev_pc);
            println!("    gpp : {}", gpp);

            //let mut subm = pr.subm.clone();
            let mut sids: Vec<_> = pr.subm.keys().collect();
            sids.sort();
            for id in &sids {
                let sbid = id.to_string();
                let sid = id.to_string();
                let Some(sb) = pr.subm.get(&sid) else {
                    println!("          ====================== NO1 {sid}");
                    continue;
                };
                let is = eg
                    .subs
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.sbid == sbid)
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();
                //println!("    s:{id} is:{is:?}");
                if is.is_empty() {
                    println!("          ====================== NO2 {sid}");
                    continue;
                }
                let si = is[0];
                let sub = &eg.subs[si];

                ///////////////////////////////////////////////
                //////////////////////////////////////////////
                //  Substation Info
                let mut sb = sb.clone();
                sb.sbtp = sub.conf.to_string();
                sb.n1d_s = sub.n1d_s;
                sb.n1d_f = sub.n1d_f;
                // substation info
                if let Some(slp) = g0.lp23.get(&sbid) {
                    sb.lp_rep_23 = slp.clone();
                }
                if let Some(slp) = g0.lp24.get(&sbid) {
                    sb.lp_rep_24 = slp.clone();
                }

                ////// collect VSPP under substation
                let mut vspps = vec![];
                let vsp = &eg.vssb[si];
                if !vsp.is_empty() {
                    for pi in vsp {
                        vspps.push(*pi);
                    }
                }
                sb.vspps = vspps;

                let mut spps = vec![];
                let spp = &eg.spsb[si];
                if !spp.is_empty() {
                    for pi in spp {
                        spps.push(*pi);
                    }
                }
                sb.spps = spps;

                let mut repls = vec![];
                let repl = &eg.resb[si];
                if !repl.is_empty() {
                    for pi in repl {
                        repls.push(*pi);
                    }
                }
                sb.repls = repls;

                println!("     ID:{}", sub.sbid);
                let mut fds = sub.feed.keys().collect::<Vec<_>>();
                fds.sort();
                let mut aoj_tr = HashMap::<usize, usize>::new();
                for fid in fds {
                    let fid = fid.to_string();
                    //println!("      {fid}");
                    let Some(tis) = sub.feed.get(&fid) else {
                        continue;
                    };
                    let fd = sb.feedm.entry(fid.to_string()).or_insert_with(|| PeaFeed {
                        fdid: fid.to_string(),
                        ..Default::default()
                    });
                    for ti in tis {
                        let tr = &eg.ctrs[*ti];
                        let t1d = tr.n1d;
                        let trs = fd.tranm.entry(t1d).or_insert_with(|| PeaTrans {
                            trid: tr.trid.to_string(),
                            pea: tr.pea.to_string(),
                            n1d: tr.n1d,
                            n1d_f: tr.n1d_f,
                            ix: tr.ix,
                            lix: tr.lix,
                            mts: tr.mts.clone(),
                            ..Default::default()
                        });

                        ////////////// AojObj
                        trs.aojs = eg.aotr[*ti].clone();
                        for ai in &trs.aojs {
                            let ai = *ai;
                            if let Some(cn) = aoj_tr.get_mut(&ai) {
                                *cn += 1;
                            } else {
                                aoj_tr.insert(ai, 1);
                            }
                        }
                        trs.amps = eg.amtr[*ti].clone();
                        trs.muns = eg.mutr[*ti].clone();
                        trs.zons = eg.zntr[*ti].clone();
                        trs.sols = if eg.sotr.len() > *ti {
                            eg.sotr[*ti].clone()
                        } else {
                            Vec::<_>::new()
                        };

                        let tcm = &eg.cmts[tr.ix];
                        let Some(ow) = &tcm.tr_own else {
                            continue;
                        };
                        trs.own = ow.clone();
                        trs.vols = eg.votr[*ti].clone();
                        let (vopw, vose) = get_tr_volta(*ti, &eg);
                        trs.vopw = vopw;
                        trs.vose = vose;
                        trs.from_cmt(tcm);
                        //println!("        trs: {}", trs.n1d);

                        let Some(kv) = &tcm.tr_kva else {
                            continue;
                        };
                        if *kv == 0.0 {
                            continue;
                        }
                        trs.kw = trf_kva_2_kw(*kv);

                        for mi in &tr.mts {
                            let mt = &eg.cmts[*mi];
                            let mb = &eg.m2bs[*mi];
                            if mb.is_empty() {
                                continue;
                            }
                            let bl = &eg.bils[mb[0]];
                            let tp = if smrt.captures(bl.rate.as_str()).is_some() {
                                MeterAccType::Small
                            } else {
                                MeterAccType::Large
                            };
                            let mut met = PeaMeter::default();
                            met.from_cmt(mt);
                            met.from_bil(bl);
                            met.met_type = tp;
                            trs.mets.push(met);
                        }
                    } //end of trans
                } // end of feeder
                let mut aojs: Vec<(usize, usize)> =
                    aoj_tr.into_iter().map(|(k, v)| (v, k)).collect();
                aojs.sort_by(|a, b| b.0.cmp(&a.0));
                let mut aojv = Vec::<AojObj>::new();
                for (v, ai) in aojs {
                    let ao = &eg.aojs[ai];
                    let code = ao.code.clone().unwrap_or("".to_string());
                    let sht_name = ao.sht_name.clone().unwrap_or("".to_string());
                    let office = ao.office.clone().unwrap_or("".to_string());
                    let pea = ao.pea.clone().unwrap_or("".to_string());
                    let aoj_sz = ao.aoj_sz.clone().unwrap_or("".to_string());
                    let reg = ao.reg.clone().unwrap_or("".to_string());
                    let name = ao.name.clone().unwrap_or("".to_string());
                    let level = ao.level.unwrap_or(0f32);
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
                        level,
                        trcn,
                    };
                    aojv.push(aoj);

                    let aojsb = aoj_sbv.entry(code.to_string()).or_default();
                    aojsb.push(sbid.to_string());
                } // end aoj loop
                sb.aojv = aojv;

                let bin: Vec<u8> =
                    bincode::encode_to_vec(&sb, bincode::config::standard()).unwrap();
                std::fs::write(format!("{dnm}/{}.bin", sb.sbid), bin).unwrap();
                println!("       ===== write to {}.bin", sb.sbid);
            } // end sub loop
        } // end province loop
    } // end area loop

    let bin: Vec<u8> = bincode::encode_to_vec(&aoj_sbv, bincode::config::standard()).unwrap();
    std::fs::write(format!("{dnm}/aoj_sbv.bin"), bin).unwrap();
    println!("       ===== write to aoj_sbv.bin");

    Ok(())
}
