use crate::dcl::*;
use crate::utl::p01_chk;
use crate::utl::*;
use crate::wrt::write_trn_ass_02;
//use num::pow::Pow;
use std::collections::HashMap;
use std::error::Error;

use crate::ben1::ben_bess_calc;
use crate::cst1::cst_bes_imp;
use crate::cst1::cst_bes_ins;
use crate::cst1::cst_bes_op;
use crate::cst1::cst_comm_imp;
use crate::cst1::cst_comm_ins;
use crate::cst1::cst_comm_op;
use crate::cst1::cst_m1p_imp;
use crate::cst1::cst_m1p_ins;
use crate::cst1::cst_m1p_op;
use crate::cst1::cst_m3p_imp;
use crate::cst1::cst_m3p_ins;
use crate::cst1::cst_m3p_op;
use crate::cst1::cst_plfm_imp;
use crate::cst1::cst_plfm_ins;
use crate::cst1::cst_plfm_op;
use crate::cst1::cst_reinvest;
use crate::cst1::cst_tr_imp;
use crate::cst1::cst_tr_ins;
use crate::cst1::cst_tr_op;
use crate::p08::ld_sub_calc;
use sglib04::web1::OP_YEAR_END;
use sglib04::web1::OP_YEAR_START;

pub const CALL_CENTER_COST_UP: f32 = 0.04f32;
pub const ASSET_WORTH_RATIO: f32 = 0.2f32;
pub const MODEL_ENTRY_RATIO: f32 = 0.05f32;
pub const MODEL_ENTRY_COST: f32 = 1000f32;
pub const RENEW_HOUR_PER_DAY: f32 = 4.0;
pub const RENEW_SAVE_PER_MWH: f32 = 500f32;
pub const PEAK_POWER_RATIO: f32 = 0.3;
pub const UNBAL_LOSS_CLAIM_RATE: f32 = 0.6;
pub const TRANS_REPL_CLAIM_RATE: f32 = 0.6;
pub const UNBAL_REPL_CLAIM_RATE: f32 = 0.6;
pub const NOTEC_LOSS_CLAIM_RATE: f32 = 0.6;

pub const NON_TECH_LOSS_RATIO: f32 = 0.02;
//pub const UNBAL_HOUR_PER_DAY: f32 = 4.0;
pub const UNBAL_HOUR_PER_DAY: f32 = 2.0;
pub const SAVE_LOSS_UNIT_PRICE: f32 = 4.0;
pub const TRANS_REPL_UNIT_PRICE: f32 = 150_000f32;
pub const TRANS_REPL_WITHIN_YEAR: f32 = 5.0;

pub const UNBAL_CALC_FACTOR: f32 = 1.0;
pub const REINVEST_RATE: f32 = 0.01;

//use sglib04::web1::ENERGY_GRW_RATE;

/// ประมวลผลรวมเพื่อเกณฑ์การคัดเลือก
/// summery transformaters to substation
pub fn stage_03() -> Result<(), Box<dyn Error>> {
    println!("===== STAGE 3 =====");
    let buf = std::fs::read(format!("{DNM}/000_pea.bin")).unwrap();
    let (pea, _): (Pea, usize) =
        bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
    let mut aids: Vec<_> = pea.aream.keys().collect();
    aids.sort();
    //println!("..1");
    let subhs = p01_chk();
    //println!("..1.1");
    //let sbtr = ld_sb_tr0();
    let sbtr = ld_sub_calc();
    //println!("..1.2");
    //println!("sbtr: {}", sbtr.len());
    let mut emp = Vec::<(u32, f32)>::new();
    for y in OP_YEAR_START..=OP_YEAR_END {
        emp.push((y, 0f32));
    }
    let resc = re_scurv();
    //
    //let mut pvcn = 0;
    //println!("..2");
    let mut v_pvas = Vec::<PeaAssVar>::new();
    let mut v_sbas = Vec::<PeaAssVar>::new();
    //let mut sbas_mx = PeaAssVar::default();
    let mut sbas_mx = PeaAssVar::from(0u64);
    for aid in aids {
        //println!("..3");
        let Some(ar) = pea.aream.get(aid) else {
            continue;
        };
        //println!("..4");
        let mut pids: Vec<_> = ar.provm.keys().collect();
        pids.sort();
        for pid in pids {
            //println!("..5");
            let Some(prov) = ar.provm.get(pid) else {
                continue;
            };
            //println!("..6");
            let mut pvas = PeaAssVar::from(0u64);
            pvas.arid = aid.to_string();
            pvas.pvid = pid.to_string();
            println!("  pv:{pid}");
            let mut sids: Vec<_> = prov.subm.keys().collect();
            sids.sort();
            for sid in sids {
                let Some(_sb) = prov.subm.get(sid) else {
                    continue;
                };
                // --- sub
                let Ok(buf) = std::fs::read(format!("{DNM}/{sid}.bin")) else {
                    //println!("PEA {sid} sub load error");
                    continue;
                };
                let (sb, _): (PeaSub, usize) =
                    bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
                //println!("PEA SUB {sid} - {}", peasb.aojv.len());

                // --- sub row data 3
                let Ok(buf) = std::fs::read(format!("{DNM}/{sid}-rw3.bin")) else {
                    continue;
                };
                let (v_tras_raw, _): (Vec<PeaAssVar>, usize) =
                    bincode::decode_from_slice(&buf[..], bincode::config::standard()).unwrap();
                if v_tras_raw.is_empty() {
                    println!("    {sid} - NO data ");
                    continue;
                }
                let tras = &v_tras_raw[0];
                let mut sbas = PeaAssVar::from(0u64);
                sbas.arid = aid.to_string();
                sbas.pvid = pid.to_string();
                sbas.sbid = tras.sbid.to_string();
                let note = if subhs.contains(&sbas.sbid) {
                    1f32
                } else {
                    0f32
                };

                let mut m_aoj = HashMap::<String, String>::new();
                for tras in &v_tras_raw {
                    sbas.add(tras);
                    let aoj = tras.aoj.clone();
                    m_aoj.entry(aoj.clone()).or_insert_with(|| aoj.clone());
                }
                sbas.v[VarType::EnGrowth.tousz()].v /= v_tras_raw.len() as f32;
                let mut aoj = String::new();
                for v in m_aoj.values() {
                    //for (_, v) in &m_aoj {
                    use std::fmt::Write;
                    if !aoj.is_empty() {
                        write!(aoj, ",").unwrap();
                    }
                    write!(aoj, "{}", v).unwrap();
                }
                sbas.aoj = "AOJ".to_string();
                sbas.aoj = aoj;
                sbas.aojv = sb.aojv.clone();
                sbas.copy(tras, VarType::NewCarReg);
                sbas.copy(tras, VarType::Gpp);
                sbas.copy(tras, VarType::MaxPosPowSub);
                sbas.copy(tras, VarType::MaxNegPowSub);
                sbas.copy(tras, VarType::VsppMv);
                sbas.copy(tras, VarType::SppHv);
                sbas.copy(tras, VarType::BigLotMv);
                sbas.copy(tras, VarType::BigLotHv);
                sbas.copy(tras, VarType::SubPowCap);
                sbas.copy(tras, VarType::SolarEnergy);
                sbas.copy(tras, VarType::PowTrSat);
                let solar = sbas.v[VarType::SolarEnergy as usize].v;
                if solar > 0f32 {
                    println!(">>>>>>>>>>> {sid} solar: {solar} =============");
                }

                // re-calculation of value
                sbas.v[VarType::LvPowSatTr as usize].v =
                    sbas.v[VarType::PkPowTr as usize].v / z2o(sbas.v[VarType::PwCapTr as usize].v);
                sbas.v[VarType::CntLvPowSatTr as usize].v =
                    if sbas.v[VarType::LvPowSatTr as usize].v > 0.8f32 {
                        1f32
                    } else {
                        0f32
                    };
                sbas.v[VarType::ChgStnCap as usize].v = sbas.v[VarType::ChgStnCapTr as usize].v;
                sbas.v[VarType::ChgStnSell as usize].v = sbas.v[VarType::ChgStnSellTr as usize].v;
                sbas.v[VarType::MvPowSatTr as usize].v = sbas.v[VarType::MaxPosPowSub as usize].v
                    / z2o(sbas.v[VarType::SubPowCap as usize].v);
                sbas.v[VarType::MvVspp as usize].v = sbas.v[VarType::VsppMv as usize].v;
                sbas.v[VarType::HvSpp as usize].v = sbas.v[VarType::SppHv as usize].v;
                sbas.v[VarType::SmallSell as usize].v = sbas.v[VarType::SmallSellTr as usize].v;
                sbas.v[VarType::LargeSell as usize].v = sbas.v[VarType::LargeSellTr as usize].v;
                sbas.v[VarType::UnbalPow as usize].v = sbas.v[VarType::UnbalPowTr as usize].v;
                let v = sbas.v[VarType::UnbalPowTr as usize].v
                    / z2o(sbas.v[VarType::PwCapTr as usize].v);
                sbas.v[VarType::CntUnbalPow as usize].v = if v > 0.5f32 { 1f32 } else { 0f32 };
                // end of recalculation

                sbas.v[VarType::TakeNote as usize].v = note;
                sbas_mx.max(&sbas);
                let Some(sbtr) = sbtr.get(&sbas.sbid) else {
                    continue;
                };
                //if let Some(sbtr) = sbtr.get(&sbas.sbid) {
                let engr = sbas.v[VarType::EnGrowth.tousz()].v;
                let pwmx = sbas.v[VarType::MaxPosPowSub.tousz()].v;
                let (mut sub, mut svg, mut dif, mut eng, bescap) =
                    ben_bess_calc(sbtr, &sb, engr, pwmx);
                let nobess = if bescap > 0f32 { 1.0 } else { 0.0 };
                sbas.v[VarType::BessMWh.tousz()].v = bescap;
                sbas.v[VarType::NoBess.tousz()].v = nobess;
                let mut ben8 = crate::ben1::ben_bill_accu(sbtr);
                let mut ben9 = crate::ben1::ben_cash_flow(sbtr);
                let mut ben10 = crate::ben1::ben_dr_save(sbtr);
                let mut ben15 = crate::ben1::ben_boxline_save(sbtr);
                let mut ben16 = crate::ben1::ben_work_save(sbtr);
                let mut ben17 = crate::ben1::ben_sell_meter(sbtr);
                let mut ben18 = crate::ben1::ben_emeter(sbtr);
                let mut ben19 = crate::ben1::ben_mt_read(sbtr);
                let mut ben20 = crate::ben1::ben_mt_disconn(sbtr);
                let mut ben21 = crate::ben1::ben_tou_sell(sbtr);
                let mut ben22 = crate::ben1::ben_tou_read(sbtr);
                let mut ben23 = crate::ben1::ben_tou_update(sbtr);
                let mut ben24 = crate::ben1::ben_outage_labor(sbtr);
                let mut ben25 = crate::ben1::ben_reduce_complain(sbtr);
                let mut ben26 = crate::ben1::ben_asset_value(sbtr);
                let mut ben27 = crate::ben1::ben_model_entry(sbtr);

                sbas.v[VarType::FirBilAccu.tousz()].v = ben8.iter().sum();
                sbas.v[VarType::FirCashFlow.tousz()].v = ben9.iter().sum();
                sbas.v[VarType::FirDRSave.tousz()].v = ben10.iter().sum();
                sbas.v[VarType::FirBatSubSave.tousz()].v = sub.iter().sum();
                sbas.v[VarType::FirBatSvgSave.tousz()].v = svg.iter().sum();
                sbas.v[VarType::FirBatEnerSave.tousz()].v = eng.iter().sum();
                sbas.v[VarType::FirBatPriceDiff.tousz()].v = dif.iter().sum();
                sbas.v[VarType::FirMetBoxSave.tousz()].v = ben15.iter().sum();
                sbas.v[VarType::FirLaborSave.tousz()].v = ben16.iter().sum();
                sbas.v[VarType::FirMetSell.tousz()].v = ben17.iter().sum();
                sbas.v[VarType::FirEMetSave.tousz()].v = ben18.iter().sum();
                sbas.v[VarType::FirMetReadSave.tousz()].v = ben19.iter().sum();
                sbas.v[VarType::FirMetDisSave.tousz()].v = ben20.iter().sum();
                sbas.v[VarType::FirTouSell.tousz()].v = ben21.iter().sum();
                sbas.v[VarType::FirTouReadSave.tousz()].v = ben22.iter().sum();
                sbas.v[VarType::FirTouUpdateSave.tousz()].v = ben23.iter().sum();
                sbas.v[VarType::FirOutLabSave.tousz()].v = ben24.iter().sum();
                sbas.v[VarType::FirComplainSave.tousz()].v = ben25.iter().sum();
                sbas.v[VarType::FirAssetValue.tousz()].v = ben26.iter().sum();
                sbas.v[VarType::FirDataEntrySave.tousz()].v = ben27.iter().sum();

                sbas.vy[VarType::FirBilAccu.tousz()].append(&mut ben8);
                sbas.vy[VarType::FirCashFlow.tousz()].append(&mut ben9);
                sbas.vy[VarType::FirDRSave.tousz()].append(&mut ben10);
                sbas.vy[VarType::FirBatSubSave.tousz()].append(&mut sub);
                sbas.vy[VarType::FirBatSvgSave.tousz()].append(&mut svg);
                sbas.vy[VarType::FirBatEnerSave.tousz()].append(&mut eng);
                sbas.vy[VarType::FirBatPriceDiff.tousz()].append(&mut dif);
                sbas.vy[VarType::FirMetBoxSave.tousz()].append(&mut ben15);
                sbas.vy[VarType::FirLaborSave.tousz()].append(&mut ben16);
                sbas.vy[VarType::FirMetSell.tousz()].append(&mut ben17);
                sbas.vy[VarType::FirEMetSave.tousz()].append(&mut ben18);
                sbas.vy[VarType::FirMetReadSave.tousz()].append(&mut ben19);
                sbas.vy[VarType::FirMetDisSave.tousz()].append(&mut ben20);
                sbas.vy[VarType::FirTouSell.tousz()].append(&mut ben21);
                sbas.vy[VarType::FirTouReadSave.tousz()].append(&mut ben22);
                sbas.vy[VarType::FirTouUpdateSave.tousz()].append(&mut ben23);
                sbas.vy[VarType::FirOutLabSave.tousz()].append(&mut ben24);
                sbas.vy[VarType::FirComplainSave.tousz()].append(&mut ben25);
                sbas.vy[VarType::FirAssetValue.tousz()].append(&mut ben26);
                sbas.vy[VarType::FirDataEntrySave.tousz()].append(&mut ben27);

                let nome1 = sbas.v[VarType::NoMet1Ph.tousz()].v;
                let nome3 = sbas.v[VarType::NoMet3Ph.tousz()].v;
                let notr = sbas.v[VarType::NoTr.tousz()].v;
                let nodev = nome1 + nome3 + notr + nobess;
                sbas.v[VarType::NoDevice.tousz()].v = nodev;
                sbas.vy[VarType::CstMet1pIns.tousz()].append(&mut cst_m1p_ins(sbtr, nome1));
                sbas.vy[VarType::CstMet3pIns.tousz()].append(&mut cst_m3p_ins(sbtr, nome3));
                sbas.vy[VarType::CstTrIns.tousz()].append(&mut cst_tr_ins(sbtr, notr));
                sbas.vy[VarType::CstBessIns.tousz()].append(&mut cst_bes_ins(sbtr, bescap));
                sbas.vy[VarType::CstPlfmIns.tousz()].append(&mut cst_plfm_ins(sbtr, nodev));
                sbas.vy[VarType::CstCommIns.tousz()].append(&mut cst_comm_ins(sbtr, nodev));

                sbas.vy[VarType::CstMet1pImp.tousz()].append(&mut cst_m1p_imp(sbtr, nome1));
                sbas.vy[VarType::CstMet3pImp.tousz()].append(&mut cst_m3p_imp(sbtr, nome3));
                sbas.vy[VarType::CstTrImp.tousz()].append(&mut cst_tr_imp(sbtr, notr));
                sbas.vy[VarType::CstBessImp.tousz()].append(&mut cst_bes_imp(sbtr, bescap));
                sbas.vy[VarType::CstPlfmImp.tousz()].append(&mut cst_plfm_imp(sbtr, nodev));
                sbas.vy[VarType::CstCommImp.tousz()].append(&mut cst_comm_imp(sbtr, nodev));

                sbas.vy[VarType::CstMet1pOp.tousz()].append(&mut cst_m1p_op(sbtr, nome1));
                sbas.vy[VarType::CstMet3pOp.tousz()].append(&mut cst_m3p_op(sbtr, nome3));
                sbas.vy[VarType::CstTrOp.tousz()].append(&mut cst_tr_op(sbtr, notr));
                sbas.vy[VarType::CstBessOp.tousz()].append(&mut cst_bes_op(sbtr, bescap));
                sbas.vy[VarType::CstPlfmOp.tousz()].append(&mut cst_plfm_op(sbtr, nodev));
                sbas.vy[VarType::CstCommOp.tousz()].append(&mut cst_comm_op(sbtr, nodev));

                sbas.v[VarType::CstMet1pIns.tousz()].v =
                    sbas.vy[VarType::CstMet1pIns.tousz()].iter().sum();
                sbas.v[VarType::CstMet3pIns.tousz()].v =
                    sbas.vy[VarType::CstMet3pIns.tousz()].iter().sum();
                sbas.v[VarType::CstTrIns.tousz()].v =
                    sbas.vy[VarType::CstTrIns.tousz()].iter().sum();
                sbas.v[VarType::CstBessIns.tousz()].v =
                    sbas.vy[VarType::CstBessIns.tousz()].iter().sum();
                sbas.v[VarType::CstPlfmIns.tousz()].v =
                    sbas.vy[VarType::CstPlfmIns.tousz()].iter().sum();
                sbas.v[VarType::CstCommIns.tousz()].v =
                    sbas.vy[VarType::CstCommIns.tousz()].iter().sum();

                sbas.v[VarType::CstMet1pImp.tousz()].v =
                    sbas.vy[VarType::CstMet1pImp.tousz()].iter().sum();
                sbas.v[VarType::CstMet3pImp.tousz()].v =
                    sbas.vy[VarType::CstMet3pImp.tousz()].iter().sum();
                sbas.v[VarType::CstTrImp.tousz()].v =
                    sbas.vy[VarType::CstTrImp.tousz()].iter().sum();
                sbas.v[VarType::CstBessImp.tousz()].v =
                    sbas.vy[VarType::CstBessImp.tousz()].iter().sum();
                sbas.v[VarType::CstPlfmImp.tousz()].v =
                    sbas.vy[VarType::CstPlfmImp.tousz()].iter().sum();
                sbas.v[VarType::CstCommImp.tousz()].v =
                    sbas.vy[VarType::CstCommImp.tousz()].iter().sum();

                sbas.v[VarType::CstMet1pOp.tousz()].v =
                    sbas.vy[VarType::CstMet1pOp.tousz()].iter().sum();
                sbas.v[VarType::CstMet3pOp.tousz()].v =
                    sbas.vy[VarType::CstMet3pOp.tousz()].iter().sum();
                sbas.v[VarType::CstTrOp.tousz()].v = sbas.vy[VarType::CstTrOp.tousz()].iter().sum();
                sbas.v[VarType::CstBessOp.tousz()].v =
                    sbas.vy[VarType::CstBessOp.tousz()].iter().sum();
                sbas.v[VarType::CstPlfmOp.tousz()].v =
                    sbas.vy[VarType::CstPlfmOp.tousz()].iter().sum();
                sbas.v[VarType::CstCommOp.tousz()].v =
                    sbas.vy[VarType::CstCommOp.tousz()].iter().sum();

                let mut pwmx = 0f32;
                if let Some(reps) = &sb.lp_rep_24.pos_rep.val {
                    for vv in reps.iter().flatten() {
                        pwmx = pwmx.max(*vv);
                    }
                };

                //let pwmx = sbas.v[VarType::SubPowCap as usize].v;
                //let pwmx = trf_kva_2_kw(pwmx);
                //let pwrt = RE_MV2HV_RATIO * 0.5f32;
                for (i, rt) in resc.iter().enumerate() {
                    // re 4 hours in one day
                    // all year 365 days
                    // unit price per mwh
                    let rerev = if i < 3 {
                        0f32
                    } else {
                        rt * pwmx
                            * PEAK_POWER_RATIO
                            * RENEW_HOUR_PER_DAY
                            * 365.0
                            * RENEW_SAVE_PER_MWH
                    };
                    //let rerev = rt * pwrt * pwmx;
                    sbas.vy[VarType::FirMvReThb.tousz()].push(rerev);
                }

                sbas.v[VarType::FirMvReThb.tousz()].v =
                    sbas.vy[VarType::FirMvReThb.tousz()].iter().sum();

                // loss occur 4 hours per day
                // loss value 4.0 thb per kw
                // loss 365 days per year
                //let unb_los = sbas.v[VarType::UnbalPowLossKw.tousz()].v * 4.0 * 4.0 * 365.0;
                let unb_los = sbas.v[VarType::UnbalPowLossKw.tousz()].v
                    * UNBAL_HOUR_PER_DAY
                    * SAVE_LOSS_UNIT_PRICE
                    * UNBAL_CALC_FACTOR
                    * 365.0;
                //let unb_los = sbas.v[VarType::UnbalPowLossKw.tousz()].v * 4.0 * 4.0;
                //
                // claim save ratio 0.5
                let mut los_sav = unb_los * UNBAL_LOSS_CLAIM_RATE;
                //
                // transformer may die within 5 years
                // unit price for replace transformers
                // claim save ratio 0.5
                let mut tr_sav = sbas.v[VarType::CntTrSatLoss.tousz()].v / TRANS_REPL_WITHIN_YEAR
                    * TRANS_REPL_UNIT_PRICE
                    * TRANS_REPL_CLAIM_RATE;
                let mut ubt_sav = sbas.v[VarType::CntTrUnbalLoss.tousz()].v
                    / TRANS_REPL_WITHIN_YEAR
                    * TRANS_REPL_UNIT_PRICE
                    * UNBAL_REPL_CLAIM_RATE;
                // all 12 months
                // factor 0.9 since it was peak month data
                // unit price
                // non techincal loss factor = 0.01
                //let mut all_sel = sbas.v[VarType::AllSellTr.tousz()].v * 0.9 * 4f32 * 0.01;
                //let mut all_sel = sbas.v[VarType::AllSellTr.tousz()].v;
                //let mut all_sel = sbas.v[VarType::AllSellTr.tousz()].v * 0.01 * 12.0 * 4f32 * 0.9;
                let mut all_sel = sbas.v[VarType::AllSellTr.tousz()].v
                    * NON_TECH_LOSS_RATIO
                    * 12.0 // in one year
                    * SAVE_LOSS_UNIT_PRICE
                    * NOTEC_LOSS_CLAIM_RATE;
                //sbas.v[VarType::AllSellTr.tousz()].v * 12.0 * 0.9 * 4_000f32 * 0.01;
                use sglib04::web1::ENERGY_GRW_RATE;
                for i in 0..15 {
                    los_sav *= 1.0 + ENERGY_GRW_RATE;
                    tr_sav *= 1.0 + ENERGY_GRW_RATE;
                    ubt_sav *= 1.0 + ENERGY_GRW_RATE;
                    all_sel *= 1.0 + ENERGY_GRW_RATE;
                    //all_sel = 0.0;
                    let (los, tr, ubt, all) = if i < 3 {
                        (0.0, 0.0, 0.0, 0.0)
                    } else {
                        (los_sav, tr_sav, ubt_sav, all_sel)
                    };
                    sbas.vy[VarType::FirUnbSave.tousz()].push(los);
                    sbas.vy[VarType::FirTrSatSave.tousz()].push(tr);
                    sbas.vy[VarType::FirTrPhsSatSave.tousz()].push(ubt);
                    sbas.vy[VarType::FirNonTechLoss.tousz()].push(all);
                }
                sbas.v[VarType::FirUnbSave.tousz()].v =
                    sbas.vy[VarType::FirUnbSave.tousz()].iter().sum();
                sbas.v[VarType::FirTrSatSave.tousz()].v =
                    sbas.vy[VarType::FirTrSatSave.tousz()].iter().sum();
                sbas.v[VarType::FirTrPhsSatSave.tousz()].v =
                    sbas.vy[VarType::FirTrPhsSatSave.tousz()].iter().sum();
                sbas.v[VarType::FirNonTechLoss.tousz()].v =
                    sbas.vy[VarType::FirNonTechLoss.tousz()].iter().sum();

                let mut fir_cpx_opx: Vec<f32> = vec![0f32; 15];
                let mut eir_cpx_opx: Vec<f32> = vec![0f32; 15];

                // CAPOPEX
                let mut capop: Vec<f32> = vec![0f32; 15];

                // CAPEX
                sbas.v[VarType::CstCapEx.tousz()].v =
                    CAPEX_FLDS.iter().map(|vt| sbas.v[vt.tousz()].v).sum();
                let mut vy0: Vec<f32> = vec![0f32; 15];
                for vt in &CAPEX_FLDS {
                    for (i, vy) in sbas.vy[vt.tousz()].iter().enumerate() {
                        vy0[i] += vy;
                        capop[i] += vy;
                        fir_cpx_opx[i] -= vy;
                        eir_cpx_opx[i] -= vy;
                    }
                }
                sbas.vy[VarType::CstCapEx.tousz()] = vy0;

                let reinv = sbas.v[VarType::CstCapEx.tousz()].v * REINVEST_RATE;

                sbas.vy[VarType::CstReinvest.tousz()].append(&mut cst_reinvest(reinv));
                sbas.v[VarType::CstReinvest.tousz()].v =
                    sbas.vy[VarType::CstReinvest.tousz()].iter().sum();

                // OPEX
                sbas.v[VarType::CstOpEx.tousz()].v =
                    OPEX_FLDS.iter().map(|vt| sbas.v[vt.tousz()].v).sum();
                let mut vy0: Vec<f32> = vec![0f32; 15];
                for vt in &OPEX_FLDS {
                    for (i, vy) in sbas.vy[vt.tousz()].iter().enumerate() {
                        vy0[i] += vy;
                        capop[i] += vy;
                        fir_cpx_opx[i] -= vy;
                        eir_cpx_opx[i] -= vy;
                    }
                }
                sbas.vy[VarType::CstOpEx.tousz()] = vy0;

                sbas.v[VarType::CstCapOpEx.tousz()].v = sbas.v[VarType::CstOpEx.tousz()].v
                    + sbas.v[VarType::CstCapEx.tousz()].v
                    + sbas.v[VarType::CstReinvest.tousz()].v;
                sbas.vy[VarType::CstCapOpEx.tousz()] = capop;

                // FIR
                sbas.v[VarType::FirSum.tousz()].v =
                    FIR_FLDS.iter().map(|vt| sbas.v[vt.tousz()].v).sum();
                let mut vy0: Vec<f32> = vec![0f32; 15];
                for vt in &FIR_FLDS {
                    for (i, vy) in sbas.vy[vt.tousz()].iter().enumerate() {
                        vy0[i] += vy;
                        fir_cpx_opx[i] += vy;
                    }
                }
                sbas.vy[VarType::FirSum.tousz()] = vy0;

                // EIR
                sbas.v[VarType::EirSum.tousz()].v =
                    EIR_FLDS.iter().map(|vt| sbas.v[vt.tousz()].v).sum();
                let mut vy0: Vec<f32> = vec![0f32; 15];
                for vt in &EIR_FLDS {
                    for (i, vy) in sbas.vy[vt.tousz()].iter().enumerate() {
                        vy0[i] += vy;
                        eir_cpx_opx[i] -= vy;
                    }
                }
                sbas.vy[VarType::EirSum.tousz()] = vy0;

                let guess = Some(0.);
                let fir: Vec<f64> = fir_cpx_opx.iter().map(|n| *n as f64).collect();
                let firr = financial::irr(&fir, guess).unwrap_or(0f64);
                let eir: Vec<f64> = eir_cpx_opx.iter().map(|n| *n as f64).collect();
                let eirr = financial::irr(&eir, guess).unwrap_or(0f64);
                //println!("FIRR: {}", firr);

                sbas.vy[VarType::FirCstRate.tousz()] = fir_cpx_opx;
                sbas.vy[VarType::EirCstRate.tousz()] = eir_cpx_opx;
                sbas.v[VarType::FirCstRate.tousz()].v = firr as f32;
                sbas.v[VarType::EirCstRate.tousz()].v = eirr as f32;
                //}
                //for tr in v_tras_raw.iter_mut() {
                //sbas.copy(tras, VarType::SolarEnergy);

                // calculation
                //}
                //if sbas.v[VarType::TakeNote as usize].v == 1f32 {
                pvas.add(&sbas);
                //}
                pvas.copy(tras, VarType::NewCarReg);
                pvas.copy(tras, VarType::Gpp);

                v_sbas.push(sbas);
                //println!("   {sid} - {}", v_tras.len());
            } // end sub loop

            // check if already exists
            let pv = pvas.pvid.clone();
            let mut tmp = Vec::<PeaAssVar>::new();
            let mut add = Vec::<PeaAssVar>::new();
            tmp.append(&mut v_pvas);
            for a in tmp {
                if a.pvid == pv {
                    add.push(a);
                } else {
                    v_pvas.push(a);
                }
            }
            while let Some(a) = add.pop() {
                pvas.add(&a);
            }

            // re-calculation of value
            pvas.v[VarType::LvPowSatTr as usize].v =
                pvas.v[VarType::PkPowTr as usize].v / z2o(pvas.v[VarType::PwCapTr as usize].v);
            pvas.v[VarType::CntLvPowSatTr as usize].v =
                if pvas.v[VarType::LvPowSatTr as usize].v > 0.8f32 {
                    1f32
                } else {
                    0f32
                };
            pvas.v[VarType::ChgStnCap as usize].v = pvas.v[VarType::ChgStnCapTr as usize].v;
            pvas.v[VarType::ChgStnSell as usize].v = pvas.v[VarType::ChgStnSellTr as usize].v;
            pvas.v[VarType::MvPowSatTr as usize].v = pvas.v[VarType::MaxPosPowSub as usize].v
                / z2o(pvas.v[VarType::SubPowCap as usize].v);
            pvas.v[VarType::MvVspp as usize].v = pvas.v[VarType::VsppMv as usize].v;
            pvas.v[VarType::HvSpp as usize].v = pvas.v[VarType::SppHv as usize].v;
            pvas.v[VarType::SmallSell as usize].v = pvas.v[VarType::SmallSellTr as usize].v;
            pvas.v[VarType::LargeSell as usize].v = pvas.v[VarType::LargeSellTr as usize].v;
            pvas.v[VarType::UnbalPow as usize].v = pvas.v[VarType::UnbalPowTr as usize].v;
            let v =
                pvas.v[VarType::UnbalPowTr as usize].v / z2o(pvas.v[VarType::PwCapTr as usize].v);
            pvas.v[VarType::CntUnbalPow as usize].v = if v > 0.5f32 { 1f32 } else { 0f32 };
            // end of recalculation

            v_pvas.push(pvas);
        } // end provi loop
    } // end area
    let mut uc1_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc1Val as usize].v, i))
        .collect();
    uc1_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc1_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc1Rank as usize].v = r as f32;
    }

    let mut uc2_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc2Val as usize].v, i))
        .collect();
    uc2_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc2_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc2Rank as usize].v = r as f32;
    }

    let mut uc3_v: Vec<_> = v_sbas
        .iter()
        .enumerate()
        .map(|(i, s)| (s.v[VarType::Uc3Val as usize].v, i))
        .collect();
    uc3_v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (r, (_, i)) in uc3_v.iter().enumerate() {
        v_sbas[*i].v[VarType::Uc3Rank as usize].v = r as f32;
    }

    // save substation data
    let bin: Vec<u8> = bincode::encode_to_vec(&v_sbas, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-sbrw.bin"), bin).unwrap();
    write_trn_ass_02(&v_sbas, &format!("{DNM}/000-sbrw0.txt"))?;
    //write_ass_csv_02(&v_sbas, &format!("{DNM}/000-sbrw0.csv"))?;

    //println!("SBAS MAX:{:?}", sbas_mx.v);
    let mut v_sbas_no = v_sbas.clone();
    for sub in v_sbas_no.iter_mut() {
        sub.nor(&sbas_mx);
    }
    let bin: Vec<u8> = bincode::encode_to_vec(&v_sbas_no, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-sbno.bin"), bin).unwrap();
    write_trn_ass_02(&v_sbas_no, &format!("{DNM}/000-sbno0.txt"))?;
    //write_ass_csv_02(&v_sbas_no, &format!("{DNM}/000-sbno0.csv"))?;

    /*
    let mut ben80 = 0.0;
    for pvas in &v_pvas {
        let ben8n = pvas.vy[VarType::FirBilAccu.tousz()].len();
        let mut ben8a = 0.0;
        for b8 in &pvas.vy[VarType::FirBilAccu.tousz()] {
            ben8a += b8;
        }
        ben80 += ben8a;
        println!("{} - {ben8n} = {ben8a}", pvas.pvid);
    }
    println!("{ben80}");
    */

    for pvas in v_pvas.iter_mut() {
        let fir_cpx_opx = pvas.vy[VarType::FirCstRate.tousz()].clone();
        let guess = Some(0.);
        let fir: Vec<f64> = fir_cpx_opx.iter().map(|n| *n as f64).collect();
        let firr = financial::irr(&fir, guess).unwrap_or(0f64);
        pvas.v[VarType::FirCstRate.tousz()].v = firr as f32;
    }
    let bin: Vec<u8> = bincode::encode_to_vec(&v_pvas, bincode::config::standard()).unwrap();
    std::fs::write(format!("{DNM}/000-pvrw.bin"), bin).unwrap();
    //write_trn_ass_02(&v_pvas, &format!("{DNM}/000-pvrw.txt"))?;
    //write_ass_csv_02(&v_sbas_no, &format!("{DNM}/000-pvrw.csv"))?;
    Ok(())
}
