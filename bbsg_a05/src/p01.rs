use crate::dcl::AojObj;
use crate::dcl::SubAssObj;
use crate::dcl::SubAssObj2;
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

use crate::dcl::ProcEngine;
use sglab02_lib::sg::wk5::EvDistCalc;

pub fn p01_ana_test2() -> Result<(), Box<dyn Error>> {
    let smrt = Regex::new(r"[12].*").unwrap();
    let now = std::time::SystemTime::now();
    let _gpp = &GPPS;
    let g0 = ProcEngine::prep1();

    //let mut sub_sele = Vec::<SubAssInfo>::new();
    let mut sub_sele = Vec::<SubAssObj>::new();
    let mut mt_type = HashMap::<String, usize>::new();
    let mut db_sub = HashMap::<String, String>::new();
    let subhs = p01_chk();
    for ar in ar_list() {
        println!("{ar} {}", now.elapsed().unwrap().as_secs());
        let eg = ProcEngine::prep2(ar);
        let mut ca_rg = vec![0f32; eg.subs.len()];
        //let mut mains = HashMap::<String, usize>::new();
        //let (mut zcn, mut dnc, mut vcn, mut re1, mut re2, mut re3) = (0, 0, 0, 0, 0, 0);
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

            if let Some(slp) = g0.lp23.get(&sbid)
                && let Some(vs) = &slp.neg_rep.val
            {
                for v in vs.iter().flatten() {
                    mx21 = mx21.max(*v);
                }
            }
            if let Some(slp) = g0.lp24.get(&sbid)
                && let Some(vs) = &slp.neg_rep.val
            {
                for v in vs.iter().flatten() {
                    mx22 = mx22.max(*v);
                }
            }

            if let Some(slp) = g0.lp23.get(&sbid)
                && let Some(vs) = &slp.pos_rep.val
            {
                for v in vs.iter().flatten() {
                    mx23 = mx23.max(*v);
                }
            }
            if let Some(slp) = g0.lp24.get(&sbid)
                && let Some(vs) = &slp.pos_rep.val
            {
                for v in vs.iter().flatten() {
                    mx24 = mx24.max(*v);
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
                //ld21,
                //ld22,
                //ld23,
                //ld24,
                mx21,
                mx22,
                mx23,
                mx24,
                //av21,
                //av22,
                //av23,
                //av24,
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
                //for (_fid, tis) in &sft.feed {
                // loop feeders
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
                        //let mbs = &eg.bils;
                        let (mut am1, mut am2) = (0.0, 0.0);
                        if let Some(cn) = mt_type.get_mut(&mb.rate) {
                            *cn += 1;
                        } else {
                            mt_type.insert(mb.rate.to_string(), 1);
                        }
                        if smrt.captures(mb.rate.as_str()).is_some() && mb.main.is_empty() {
                            //if smvo.captures(mb.volt.as_str()).is_some() && mb.main.is_empty() {
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
            //println!("SBAS:  {} {}", sbas.sbid, aojv.len());
            //println!("rate: {mt_type:?}");
            //println!("  aojv:{:?}", aojv);
            sub_sele.push(sbas);
        } // end sub loop
          /*
          if cnt > 0 {
              break;
          }
              */
    } // end area loop
    let cfg = bincode::config::standard();
    let bin: Vec<u8> = bincode::encode_to_vec(&sub_sele, cfg).unwrap();
    let fnm = "/mnt/e/CHMBACK/pea-data/data2/p13_test2.bin";
    std::fs::write(fnm, bin).unwrap();
    println!("write to {fnm} - {}", sub_sele.len());

    /*
    for s in &sub_sele {
        println!("!! {} - {}", s.sbid, s.aojv.len());
    }
    */

    /*
    let (obj, _): (Vec<SubAssObj>, usize) = bincode::decode_from_slice(&encoded[..], cfg).unwrap();
    */
    Ok(())
}

pub fn p01_ana_test3() -> Result<(), Box<dyn Error>> {
    let cfg = bincode::config::standard();
    //let subs = get_sele_subs();

    let fnm = "/mnt/e/CHMBACK/pea-data/data2/p13_test2.bin";
    let bytes = std::fs::read(fnm).unwrap();
    let (obj, _): (Vec<SubAssObj>, usize) = bincode::decode_from_slice(&bytes[..], cfg).unwrap();
    println!("obj: {}", obj.len());
    let sbasv = obj;

    let fnm = "/mnt/e/CHMBACK/pea-data/repo2/p13_test3_raw.txt";
    write_sub_asses(&sbasv, fnm)?;
    let mut sbas0 = sbasv[0].clone();
    for s in &sbasv {
        sbas0.cpmw = sbas0.cpmw.min(s.cpmw);
        sbas0.mx21 = sbas0.mx21.min(s.mx21);
        sbas0.mx22 = sbas0.mx22.min(s.mx22);
        sbas0.mx23 = sbas0.mx23.min(s.mx23);
        sbas0.mx24 = sbas0.mx24.min(s.mx24);
        sbas0.se_s = sbas0.se_s.min(s.se_s);
        sbas0.se_l = sbas0.se_l.min(s.se_l);
        sbas0.se_2 = sbas0.se_2.min(s.se_2);
        sbas0.sell = sbas0.sell.min(s.sell);
        sbas0.gpp = sbas0.gpp.min(s.gpp);
        sbas0.evca = sbas0.evca.min(s.evca);
        sbas0.psat = sbas0.psat.min(s.psat);
        sbas0.vopw = sbas0.vopw.min(s.vopw);
        sbas0.vose = sbas0.vose.min(s.vose);
        sbas0.dens = sbas0.dens.min(s.dens);
        sbas0.zone = sbas0.zone.min(s.zone);
        sbas0.sorf = sbas0.sorf.min(s.sorf);
        sbas0.vspkw = sbas0.vspkw.min(s.vspkw);
        sbas0.sppmw = sbas0.sppmw.min(s.sppmw);
        sbas0.unbal = sbas0.unbal.min(s.unbal);
        sbas0.repln = sbas0.repln.min(s.repln);
    }
    let mut sbas1 = sbasv[0].clone();
    for s in &sbasv {
        sbas1.cpmw = sbas1.cpmw.max(s.cpmw);
        sbas1.mx21 = sbas1.mx21.max(s.mx21);
        sbas1.mx22 = sbas1.mx22.max(s.mx22);
        sbas1.mx23 = sbas1.mx23.max(s.mx23);
        sbas1.mx24 = sbas1.mx24.max(s.mx24);
        sbas1.se_s = sbas1.se_s.max(s.se_s);
        sbas1.se_l = sbas1.se_l.max(s.se_l);
        sbas1.se_2 = sbas1.se_2.max(s.se_2);
        sbas1.sell = sbas1.sell.max(s.sell);
        sbas1.gpp = sbas1.gpp.max(s.gpp);
        sbas1.evca = sbas1.evca.max(s.evca);
        sbas1.psat = sbas1.psat.max(s.psat);
        sbas1.vopw = sbas1.vopw.max(s.vopw);
        sbas1.vose = sbas1.vose.max(s.vose);
        sbas1.dens = sbas1.dens.max(s.dens);
        sbas1.zone = sbas1.zone.max(s.zone);
        sbas1.sorf = sbas1.sorf.max(s.sorf);
        sbas1.vspkw = sbas1.vspkw.max(s.vspkw);
        sbas1.sppmw = sbas1.sppmw.max(s.sppmw);
        sbas1.unbal = sbas1.unbal.max(s.unbal);
        sbas1.repln = sbas1.repln.max(s.repln);
    }
    let mn = sbas0;
    let mx = sbas1;
    let mut sbasv2 = Vec::<SubAssObj>::new();
    for s in &sbasv {
        let mut x = s.clone();
        x.cpmw = x.cpmw.max(mn.cpmw);
        x.mx21 = x.mx21.max(mn.mx21);
        x.mx22 = x.mx22.max(mn.mx22);
        x.mx23 = x.mx23.max(mn.mx23);
        x.mx24 = x.mx24.max(mn.mx24);
        x.se_s = x.se_s.max(mn.se_s);
        x.se_l = x.se_l.max(mn.se_l);
        x.se_2 = x.se_2.max(mn.se_2);
        x.sell = x.sell.max(mn.sell);
        x.gpp = x.gpp.max(mn.gpp);
        x.evca = x.evca.max(mn.evca);
        x.psat = x.psat.max(mn.psat);
        x.vopw = x.vopw.max(mn.vopw);
        x.vose = x.vose.max(mn.vose);
        x.dens = x.dens.max(mn.dens);
        x.zone = x.zone.max(mn.zone);
        x.sorf = x.sorf.max(mn.sorf);
        x.vspkw = x.vspkw.max(mn.vspkw);
        x.sppmw = x.sppmw.max(mn.sppmw);
        x.unbal = x.unbal.max(mn.unbal);
        x.repln = x.repln.max(mn.repln);
        x.cpmw = x.cpmw / mx.cpmw;
        x.mx21 = x.mx21 / mx.mx21;
        x.mx22 = x.mx22 / mx.mx22;
        x.mx23 = x.mx23 / mx.mx23;
        x.mx24 = x.mx24 / mx.mx24;
        x.se_s = x.se_s / mx.se_s;
        x.se_l = x.se_l / mx.se_l;
        x.se_2 = x.se_2 / mx.se_2;
        x.sell = x.sell / mx.sell;
        x.gpp = x.gpp / mx.gpp;
        x.evca = x.evca / mx.evca;
        x.psat = x.psat / mx.psat;
        x.vopw = x.vopw / mx.vopw;
        x.vose = x.vose / mx.vose;
        x.dens = x.dens / mx.dens;
        x.zone = x.zone / mx.zone;
        x.sorf = x.sorf / mx.sorf;
        x.vspkw = x.vspkw / mx.vspkw;
        x.sppmw = x.sppmw / mx.sppmw;
        x.unbal = x.unbal / mx.unbal;
        x.repln = x.repln / mx.repln;
        sbasv2.push(x);
    }
    //let fnm = "/mnt/e/CHMBACK/pea-data/repo2/p13_test3b.txt";
    //write_sub_asses2(&sbasv2, fnm)?;
    let mut lkup = HashMap::<String, usize>::new();
    for (i, v) in sbasv2.iter().enumerate() {
        lkup.insert(v.sbid.to_string(), i);
    }
    let mut sele = Vec::<SubAssObj2>::new();
    for s in &sbasv2 {
        let mut x = SubAssObj2::default();
        x.sbid = s.sbid.to_string();
        x.prov = s.prov.to_string();
        x.arid = s.arid.to_string();
        x.ev1 = s.evca;
        x.ev2 = s.psat;
        x.ev3 = s.vopw;
        x.ev4 = s.vose;
        let cpmw = if s.cpmw == 0.0 { 100.0 } else { s.cpmw };
        x.ev5 = s.vopw / cpmw;
        //x.ev5 = s.vopw / s.cpmw;
        x.re1 = s.se_2;
        x.re2 = s.vspkw;
        x.re3 = s.sppmw;
        x.en1 = s.se_s;
        x.en2 = s.se_l;
        x.en3 = s.unbal;
        x.en4 = s.dens;
        sele.push(x);
    }
    let mut ev5 = sele[0].ev5;
    for s in &sele {
        ev5 = ev5.max(s.ev5);
    }
    for s in &mut sele {
        //s.ev5 /= ev5;
        s.sum();
    }
    let fnm = "/mnt/e/CHMBACK/pea-data/repo2/p13_test3_norm.txt";
    write_sub_asses3(&sbasv2, &sele, &lkup, fnm)?;

    let mut uc1 = SubAssObj2::default();
    uc1.ev1 = 0.15;
    uc1.ev2 = 0.15;
    uc1.ev3 = 0.05;
    uc1.ev4 = 0.05;
    uc1.ev5 = 0.05;
    uc1.re1 = 0.15;
    uc1.re2 = 0.05;
    uc1.re3 = 0.05;
    uc1.en1 = 0.10;
    uc1.en2 = 0.05;
    uc1.en3 = 0.10;
    uc1.en4 = 0.05;
    let mut sele1 = sele.clone();
    for x in &mut sele1 {
        x.ev1 *= uc1.ev1;
        x.ev2 *= uc1.ev2;
        x.ev3 *= uc1.ev3;
        x.ev4 *= uc1.ev4;
        x.ev5 *= uc1.ev5;
        x.re1 *= uc1.re1;
        x.re2 *= uc1.re2;
        x.re3 *= uc1.re3;
        x.en1 *= uc1.en1;
        x.en2 *= uc1.en2;
        x.en3 *= uc1.en3;
        x.en4 *= uc1.en4;
        x.sum();
    }
    let mut rnks = Vec::<(usize, f32)>::new();
    for (i, x) in sele1.iter().enumerate() {
        rnks.push((i, x.sum));
    }
    rnks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (i, (o, _)) in rnks.iter().enumerate() {
        sele1[*o].rank = i + 1;
    }
    let fnm = "/mnt/e/CHMBACK/pea-data/repo2/p13_test3_uc1_r.txt";
    write_sub_asses3(&sbasv2, &sele1, &lkup, fnm)?;

    let mut uc1_p = sele1.clone();
    //uc1_p.sort_by(|a, b| a.prov.cmp(&b.prov));
    uc1_p.sort_by(|a, b| {
        let aa = format!("{}_{}_{}", a.arid, a.prov, a.sbid);
        let bb = format!("{}_{}_{}", b.arid, b.prov, b.sbid);
        aa.cmp(&bb)
    });
    let fnm = "/mnt/e/CHMBACK/pea-data/repo2/p13_test3_uc1_p.txt";
    write_sub_asses3(&sbasv2, &uc1_p, &lkup, fnm)?;

    let mut uc2 = SubAssObj2::default();
    uc2.ev1 = 0.05;
    uc2.ev2 = 0.05;
    uc2.ev3 = 0.10;
    uc2.ev4 = 0.10;
    uc2.ev5 = 0.15;
    uc2.re1 = 0.05;
    uc2.re2 = 0.15;
    uc2.re3 = 0.10;
    uc2.en1 = 0.05;
    uc2.en2 = 0.10;
    uc2.en3 = 0.05;
    uc2.en4 = 0.05;
    let mut sele2 = sele.clone();
    for x in &mut sele2 {
        x.ev1 *= uc2.ev1;
        x.ev2 *= uc2.ev2;
        x.ev3 *= uc2.ev3;
        x.ev4 *= uc2.ev4;
        x.ev5 *= uc2.ev5;
        x.re1 *= uc2.re1;
        x.re2 *= uc2.re2;
        x.re3 *= uc2.re3;
        x.en1 *= uc2.en1;
        x.en2 *= uc2.en2;
        x.en3 *= uc2.en3;
        x.en4 *= uc2.en4;
        x.sum();
    }
    let mut rnks = Vec::<(usize, f32)>::new();
    for (i, x) in sele2.iter().enumerate() {
        rnks.push((i, x.sum));
    }
    rnks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (i, (o, _)) in rnks.iter().enumerate() {
        sele2[*o].rank = i + 1;
    }
    let fnm = "/mnt/e/CHMBACK/pea-data/repo2/p13_test3_uc2_r.txt";
    write_sub_asses3(&sbasv2, &sele2, &lkup, fnm)?;

    let mut uc2_p = sele2.clone();
    //uc1_p.sort_by(|a, b| a.prov.cmp(&b.prov));
    uc2_p.sort_by(|a, b| {
        let aa = format!("{}_{}_{}", a.arid, a.prov, a.sbid);
        let bb = format!("{}_{}_{}", b.arid, b.prov, b.sbid);
        aa.cmp(&bb)
    });
    let fnm = "/mnt/e/CHMBACK/pea-data/repo2/p13_test3_uc2_p.txt";
    write_sub_asses3(&sbasv2, &uc2_p, &lkup, fnm)?;
    Ok(())
}

fn write_sub_asses(sbasv: &Vec<SubAssObj>, fnm: &str) -> Result<(), Box<dyn Error>> {
    let mut x = String::new();
    use std::fmt::Write;
    for s in sbasv {
        write!(x, "{}", s.sbid)?;
        write!(x, "\t{}", s.sbth)?;
        write!(x, "\t{}", s.sben)?;
        write!(x, "\t{}", s.prov)?;
        write!(x, "\t{}", s.arid)?;
        write!(x, "\t{}", s.aojv.len())?;
        write!(x, "\t")?;
        for (i, a) in s.aojv.iter().enumerate() {
            if i > 0 {
                write!(x, ", ")?;
            }
            write!(x, "{}", a.name)?;
        }
        write!(x, "\t{}", s.cpmw)?;
        write!(x, "\t{}", s.mx21)?;
        write!(x, "\t{}", s.mx22)?;
        write!(x, "\t{}", s.mx23)?;
        write!(x, "\t{}", s.mx24)?;
        //write!(x, "\t{}", s.av21)?;
        //write!(x, "\t{}", s.av22)?;
        //write!(x, "\t{}", s.av23)?;
        //write!(x, "\t{}", s.av24)?;
        write!(x, "\t{}", s.se_s)?;
        write!(x, "\t{}", s.se_l)?;
        write!(x, "\t{}", s.se_2)?;
        write!(x, "\t{}", s.sell)?;
        write!(x, "\t{}", s.gpp)?;
        write!(x, "\t{}", s.evca * 100000.0)?;
        write!(x, "\t{}", s.psat)?;
        write!(x, "\t{}", s.vopw)?;
        write!(x, "\t{}", s.vose)?;
        write!(x, "\t{}", s.dens)?;
        write!(x, "\t{}", s.zone)?;
        write!(x, "\t{}", s.sorf)?;
        write!(x, "\t{}", s.vspkw)?;
        write!(x, "\t{}", s.sppmw)?;
        write!(x, "\t{}", s.unbal)?;
        write!(x, "\t{}", s.repln)?;
        writeln!(x)?;
    }
    println!("write to {fnm}");
    std::fs::write(fnm, x)?;

    Ok(())
}

fn write_sub_asses3(
    sbasv: &Vec<SubAssObj>,
    sv: &Vec<SubAssObj2>,
    lkup: &HashMap<String, usize>,
    fnm: &str,
) -> Result<(), Box<dyn Error>> {
    let mut x = String::new();
    use std::fmt::Write;
    for v in sv {
        let i = lkup.get(&v.sbid).unwrap();
        let s = &sbasv[*i];
        //for (s, v) in sbasv.iter().zip(sv.iter()) {
        write!(x, "{}", s.sbid)?;
        write!(x, "\t{}", s.sbth)?;
        write!(x, "\t{}", s.sben)?;
        write!(x, "\t{}", s.prov)?;
        write!(x, "\t{}", v.arid)?;
        write!(x, "\t{}", s.aojv.len())?;
        write!(x, "\t")?;
        for (i, a) in s.aojv.iter().enumerate() {
            if i > 0 {
                write!(x, ", ")?;
            }
            write!(x, "{}", a.name)?;
        }
        write!(x, "\t{}", v.ev1)?;
        write!(x, "\t{}", v.ev2)?;
        write!(x, "\t{}", v.ev3)?;
        write!(x, "\t{}", v.ev4)?;
        write!(x, "\t{}", v.ev5)?;
        write!(x, "\t{}", v.re1)?;
        write!(x, "\t{}", v.re2)?;
        write!(x, "\t{}", v.re3)?;
        write!(x, "\t{}", v.en1)?;
        write!(x, "\t{}", v.en2)?;
        write!(x, "\t{}", v.en3)?;
        write!(x, "\t{}", v.en4)?;
        write!(x, "\t{}", v.sum)?;
        write!(x, "\t{}", v.rank)?;
        write!(x, "\t{}", s.note)?;
        writeln!(x)?;
    }
    println!("write to {fnm}");
    std::fs::write(fnm, x)?;

    Ok(())
}

use crate::p08::ld_pv_ca_mp;

pub fn ev_distr(ev_adx: &[(&str, f64, f64)]) -> HashMap<String, EvDistCalc> {
    let mut pv_ca_mp = ld_pv_ca_mp();
    //let mut pv_ca_mp = load_pvcamp();
    //println!("AAAA {}", pv_ca_mp.len());
    let mut pv_ca_mp2 = HashMap::new();
    let mut tt = 0f64;
    for v in pv_ca_mp.values() {
        //for (_k, v) in &pv_ca_mp {
        tt += v;
        //println!("{k} - {v}");
    }
    println!("total car: {tt}");
    pv_ca_mp.insert("กรุงเทพมหานคร".to_string(), 967297.0);
    for (k, v) in &pv_ca_mp {
        let mut kk = k.to_string();
        let mut vv = *v;
        if k == "ยะลา"
            && let Some(v2) = pv_ca_mp.get("สาขา อ.เบตง")
        {
            //let v1 = *v2;
            vv += *v2;
        }
        if kk == " พระนครศรีอยุธยา" {
            kk = "พระนครศรีอยุธยา".to_string();
        }
        if kk == "แม่ฮองสอน" {
            kk = "แม่ฮ่องสอน".to_string();
        }
        if kk == "สาขา อ.เบตง" {
            //print!("NO BETONG\n");
        } else {
            //print!("'{}' - {}\n", kk, vv);
            pv_ca_mp2.insert(kk.clone(), vv);
            //pv_ca_cn2.insert(kk, 0);
        }
    }

    //let ev_adx = ev_prov_adjust();
    //let ev_adx = &EV_PRV_ADJ_1;
    let mut tk0 = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let ts = adx.0.to_string();
        if let Some(nn) = pv_ca_mp2.get_mut(&ts) {
            let tk = *nn * ev_adx[i].2 / 100.0;
            *nn -= tk;
            tk0 += tk;
        }
    }
    let mut ass_sm = 0.0;
    for (i, adx) in ev_adx.iter().enumerate() {
        let ts = adx.0.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&ts) {
            let ad = tk0 * ev_adx[i].1 / 100.0;
            ass_sm += ev_adx[i].1;
            *cn += ad;
        } else {
            println!("no adj {}", adx.0);
        }
    }

    println!("assign %{}", ass_sm);

    let mut pv_car_reg_mp = HashMap::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        if ["กรุงเทพมหานคร,นนทบุรี,สมุทรปราการ"].contains(&k.as_str())
        {
            continue;
        }
        let pv_ca_reg = EvDistCalc {
            id: k.to_string(),
            ev_no: *v as f32,
            ..Default::default()
        };
        total += pv_ca_reg.ev_no;
        pv_car_reg_mp.insert(k.to_string(), pv_ca_reg);
    }

    for v in pv_car_reg_mp.values_mut() {
        if total > 0.0 {
            v.ev_pc = v.ev_no / total;
        }
    }
    pv_car_reg_mp
}
