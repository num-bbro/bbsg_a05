use crate::stg3::ASSET_WORTH_RATIO;
use crate::stg3::MODEL_ENTRY_COST;
use crate::stg3::MODEL_ENTRY_RATIO;
use num::pow::Pow;
use sglib04::prc41::SubCalc;
use sglib04::web1::ECO_GRW_RATE;
use sglib04::web1::ENERGY_GRW_RATE;
use sglib04::web1::M1P_COST;
use sglib04::web1::M3P_COST;
use sglib04::web1::TRX_COST;
use sglib04::web1::UNIT_PRICE;

pub const SMETER_ACCU_IMPRV: f32 = 0.01f32;
pub const SMETER_BILL_IMPRV: f32 = 0.4f32;

pub fn ben_bill_accu(sbtr: &SubCalc) -> Vec<f32> {
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = sbtr.eg_sm as f64 * SMETER_ACCU_IMPRV as f64 * SMETER_BILL_IMPRV as f64;
        let be = be * 30f64;
        let be = be * UNIT_PRICE as f64;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        let be = be * 0.1;
        //print!(" {}-{be:.2}", y + 2028);
        proj.push(be as f32);
    }
    proj
}

pub const CASH_FLOW_COST: f32 = 0.0569; // cash flow cost per day
pub const CASH_DAY_DELAY_80: f32 = 2.5; // days delays for 80% of meter
pub const CASH_DAY_DELAY_20: f32 = 12.5; // days delays for 80% of meter
pub const CASH_DAY_DELAY_SMART: f32 = 2.0;

pub fn ben_cash_flow(sbtr: &SubCalc) -> Vec<f32> {
    let al0 = sbtr.eg_sm;
    let dl_80 = CASH_DAY_DELAY_80;
    let dl_20 = CASH_DAY_DELAY_20;
    let dl_0 = dl_80 * 0.8 + dl_20 * 0.2; // average days delay of cash flow
    let dl_d = dl_0 - CASH_DAY_DELAY_SMART; // diff days improved
    let dl_m1 = al0 as f32 * UNIT_PRICE / 365.0 * dl_d * CASH_FLOW_COST;
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = dl_m1 as f64;
        // adjust
        let be = be * 40f64;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push(be as f32);
    }
    //println!();
    //BenProj { proj }
    proj
}

pub const DR_DEV_PLAN_RATE: f32 = 0.02f32;

pub fn ben_dr_save(sbtr: &SubCalc) -> Vec<f32> {
    //let cap1 = 80_000_000f64 / 22_000_000f64;
    //let cap2 = 20_000_000f64 / 22_000_000f64;
    //print!("====  Demand Response ");
    let mt_1_ph = sbtr.mt_1_ph as f64 * DR_DEV_PLAN_RATE as f64;
    let mt_3_ph = sbtr.mt_3_ph as f64 * DR_DEV_PLAN_RATE as f64;
    let cap3 = mt_1_ph * 2_500f64;
    let cap4 = mt_3_ph * 4_650f64;
    let opx1 = cap3 * 0.005;
    let opx2 = cap4 * 0.005;
    let opx3 = (mt_1_ph + mt_3_ph) * 55f64 * 12f64;
    let opx4 = cap3 * 0.05f64;
    let opx5 = cap4 * 0.05f64;
    //let mut proj = Vec::<(u32, f32)>::new();
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = if y == 0 { cap3 + cap4 } else { 0f64 };
        let be = be + opx1 + opx2 + opx3 + opx4 + opx5;
        // adjust
        let be = be * 1.1f64;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        proj.push(be as f32);
    }
    proj
}

pub const BOX_LINE_NEED_RATE: f32 = 0.05f32;
pub const BOX_LINE_UNIT_COST: f32 = 172.41f32;

pub fn ben_boxline_save(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  BOX : ");
    let boxcnt = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 * BOX_LINE_NEED_RATE as f64;
    let boxex = boxcnt * BOX_LINE_UNIT_COST as f64;
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = boxex;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push(be as f32);
    }
    //println!();
    proj
}

pub const METER_PER_WORKER: f32 = 5825f32;
pub const WORKER_MONTH_SALARY: f32 = 35_000f32;
pub const WORKER_BONUS_MONTH: f32 = 1f32;
pub const WORKER_SAVING_RATE: f32 = 0.03f32;
pub const WORKER_SOC_SEC_RATE: f32 = 0.05f32;
pub const WORKER_REDUCE_RATE: f32 = 0.25f32;
pub const SALARY_INCR_RATE: f32 = 0.04f32;

pub fn ben_work_save(sbtr: &SubCalc) -> Vec<f32> {
    let wk_cnt = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 / METER_PER_WORKER as f64;
    let mn_exp =
        WORKER_MONTH_SALARY as f64 * (1.0 + WORKER_SAVING_RATE + WORKER_SOC_SEC_RATE) as f64;
    let yr_exp = mn_exp * 12f64 + WORKER_MONTH_SALARY as f64 * WORKER_BONUS_MONTH as f64;
    let yr_exp = yr_exp * wk_cnt;
    //print!(" mn:{mn_exp} yr:{yr_exp}");
    let wk_redu = yr_exp * WORKER_REDUCE_RATE as f64;
    //print!(" rd:{wk_redu}");
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = wk_redu;
        let be = be / Pow::pow(1f64 + ENERGY_GRW_RATE as f64, y as f64);
        let be = be * Pow::pow(1f64 + SALARY_INCR_RATE as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push(be as f32);
    }
    proj
}

pub const METER_SELLABLE_RATE: f32 = 0.33f32;
pub const M3P_SELL_PRICE: f32 = 100f32;
pub const M1P_SELL_PRICE: f32 = 50f32;

pub fn ben_sell_meter(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  SELL METER");
    let m1p = sbtr.mt_1_ph as f64 * METER_SELLABLE_RATE as f64;
    let m3p = sbtr.mt_3_ph as f64 * METER_SELLABLE_RATE as f64;
    let m1p_s = m1p * M1P_SELL_PRICE as f64;
    let m3p_s = m3p * M3P_SELL_PRICE as f64;
    let m1p_y = m1p_s / 12f64;
    let m3p_y = m3p_s / 12f64;
    //let mut proj = Vec::<(u32, f32)>::new();
    let mut proj = vec![0.0, 0.0, 0.0];
    for _y in 0..12 {
        let be = m1p_y + m3p_y;
        //print!(" {}-{be:.2}", y + 2028);
        //proj.push((y + 2028, be as f32));
        proj.push(be as f32);
    }
    //println!();
    //BenProj { proj }
    proj
}

pub const EMTR_CNT_RATIO: f32 = 0.05f32;
pub const EMTR_SWAP_RATE: f32 = 0.1f32;
pub const EMTR_REPL_RATE: f32 = 0.02f32;
pub const EMTR_1P_COST: f32 = 525f32;
pub const EMTR_3P_COST: f32 = 1_285f32;
pub const EMTR_1P_SWAP: f32 = 100f32;
pub const EMTR_3P_SWAP: f32 = 200f32;
pub const EMTR_1P_REPL: f32 = 250f32;
pub const EMTR_3P_REPL: f32 = 400f32;
pub const EMTR_COST_UP: f32 = 0.02f32;
pub fn ben_emeter(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  EMETER");
    let m1_cnt = sbtr.mt_1_ph as f64 * EMTR_CNT_RATIO as f64;
    let m3_cnt = sbtr.mt_3_ph as f64 * EMTR_CNT_RATIO as f64;
    let m1_sw_c = m1_cnt * EMTR_SWAP_RATE as f64;
    let m3_sw_c = m3_cnt * EMTR_SWAP_RATE as f64;
    let m1_sw_e = m1_sw_c * (EMTR_1P_COST + EMTR_1P_SWAP) as f64;
    let m3_sw_e = m3_sw_c * (EMTR_3P_COST + EMTR_3P_SWAP) as f64;
    let m1_rp_c = m1_cnt * EMTR_REPL_RATE as f64;
    let m3_rp_c = m3_cnt * EMTR_REPL_RATE as f64;
    let m1_rp_e = m1_rp_c * (EMTR_1P_COST + EMTR_1P_REPL) as f64;
    let m3_rp_e = m3_rp_c * (EMTR_3P_COST + EMTR_3P_REPL) as f64;
    let ex = m1_sw_e + m3_sw_e + m1_rp_e + m3_rp_e;
    //let mut proj = Vec::<(u32, f32)>::new();
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + EMTR_COST_UP as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        //proj.push((y + 2028, be as f32));
        proj.push(be as f32);
    }
    //println!();
    //BenProj { proj }
    proj
}

pub const MT_READ_COST: f32 = 6.2f32;
pub const READ_COST_UP: f32 = 0.04f32;

pub fn ben_mt_read(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  READING");
    let m1_rd = sbtr.mt_1_ph as f64 * MT_READ_COST as f64 * 12f64;
    let m3_rd = sbtr.mt_3_ph as f64 * MT_READ_COST as f64 * 12f64;
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = m1_rd + m3_rd;
        let be = be * Pow::pow(1f64 + READ_COST_UP as f64, y as f64);
        proj.push(be as f32);
    }
    proj
}

pub const M1_DISCON_COST: f32 = 130f32;
pub const M3_DISCON_COST: f32 = 190f32;
pub const M1_DISCON_RATE: f32 = 0.004f32;
pub const M3_DISCON_RATE: f32 = 0.001f32;
pub const DISCON_COST_UP: f32 = 0.04f32;

pub fn ben_mt_disconn(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  DISCON");
    let m1_cn = sbtr.mt_1_ph as f64 * M1_DISCON_RATE as f64;
    let m3_cn = sbtr.mt_3_ph as f64 * M3_DISCON_RATE as f64;
    let m1_ex = m1_cn * M1_DISCON_COST as f64;
    let m3_ex = m3_cn * M3_DISCON_COST as f64;

    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = m1_ex + m3_ex;
        let be = be * 200f64;
        let be = be * Pow::pow(1f64 + DISCON_COST_UP as f64, y as f64);
        proj.push(be as f32);
    }
    proj
}

pub const TOU_METER_RATIO: f32 = 0.15;
pub const TOU_SELLABLE_RATE: f32 = 0.20;
//const TOU_1P_RATIO: f32 = 0.74f32;
//const TOU_3P_RATIO: f32 = 0.26f32;
pub const TOU_1P_SELL_PRICE: f32 = 350f32;
pub const TOU_3P_SELL_PRICE: f32 = 857f32;

pub fn ben_tou_sell(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  SELL METER");
    let m1p = sbtr.mt_1_ph as f64 * TOU_METER_RATIO as f64 * TOU_SELLABLE_RATE as f64;
    let m3p = sbtr.mt_3_ph as f64 * TOU_METER_RATIO as f64 * TOU_SELLABLE_RATE as f64;
    let m1p_s = m1p * TOU_1P_SELL_PRICE as f64;
    let m3p_s = m3p * TOU_3P_SELL_PRICE as f64;
    let m1p_y = m1p_s / 12f64;
    let m3p_y = m3p_s / 12f64;
    let mut proj = vec![0.0, 0.0, 0.0];
    for _y in 0..12 {
        let be = m1p_y + m3p_y;
        //print!(" {}-{be:.2}", y + 2028);
        proj.push(be as f32);
    }
    proj
}

//pub const TOU_READ_COST: f32 = 18f32;
pub const TOU_READ_COST: f32 = 15f32;
pub const TOU_COST_UP: f32 = 0.04f32;

pub fn ben_tou_read(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  TOU READ");
    let m1p = sbtr.mt_1_ph as f64 * TOU_METER_RATIO as f64 * 12f64;
    let m3p = sbtr.mt_3_ph as f64 * TOU_METER_RATIO as f64 * 12f64;
    let m1_rd = m1p * TOU_READ_COST as f64;
    let m3_rd = m3p * TOU_READ_COST as f64;
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = m1_rd + m3_rd;
        let be = be * Pow::pow(1f64 + TOU_COST_UP as f64, y as f64);
        proj.push(be as f32);
    }
    proj
}

pub const TOU_UPDATE_COST: f32 = 200f32;

pub fn ben_tou_update(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  TOU UPDATE");
    let m1p = sbtr.mt_1_ph as f64 * TOU_METER_RATIO as f64;
    let m3p = sbtr.mt_3_ph as f64 * TOU_METER_RATIO as f64;
    let m1_rd = m1p * TOU_UPDATE_COST as f64;
    let m3_rd = m3p * TOU_UPDATE_COST as f64;
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = m1_rd + m3_rd;
        let be = be * Pow::pow(1f64 + TOU_COST_UP as f64, y as f64);
        //print!(" {}-{be:.2}", y + 2028);
        proj.push(be as f32);
    }
    proj
}

pub const OUT_MT_HOUR_YEAR: f32 = 0.0011f32; // 125/116000
pub const LABOR_COST_HOUR: f32 = 2_000f32;

pub fn ben_outage_labor(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  OUTAGE LABOR");
    let hr = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 * OUT_MT_HOUR_YEAR as f64;
    let ex = hr * LABOR_COST_HOUR as f64 * 5f64;
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + ECO_GRW_RATE as f64, y as f64);
        proj.push(be as f32);
    }
    proj
}

// FirComplainSave
pub const CALL_CENTER_COST_MT: f32 = 3.33f32;
pub const CALL_CENTER_COST_UP: f32 = 0.04f32;

pub fn ben_reduce_complain(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  COMPLAIN");
    let ex = (sbtr.mt_1_ph + sbtr.mt_3_ph) as f64 * CALL_CENTER_COST_MT as f64;
    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = ex;
        let be = be * Pow::pow(1f64 + CALL_CENTER_COST_UP as f64, y as f64);
        proj.push(be as f32);
    }
    proj
}

//FirAssetValue
pub fn ben_asset_value(sbtr: &SubCalc) -> Vec<f32> {
    let m1i = sbtr.mt_1_ph as f64 * M1P_COST as f64;
    let m3i = sbtr.mt_3_ph as f64 * M3P_COST as f64;
    let txp = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txc = sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txi = (txp + txc) as f64 * TRX_COST as f64;
    let esi = 0f64;
    /*
    let mut esi = 0f64;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32 {
        esi = ben.bat_cost as f64 * 1_000_000_f64;
    }
    */
    let ass = (m1i + m3i + txi + esi) * ASSET_WORTH_RATIO as f64;
    //print!("  m1:{m1i} m3:{m3i} t:{txi} b:{esi} = as:{ass}\n");
    let mut proj = vec![0.0, 0.0, 0.0];
    for _y in 0..11 {
        proj.push(0f32);
    }
    proj.push(ass as f32);
    proj
}

pub fn ben_model_entry(sbtr: &SubCalc) -> Vec<f32> {
    //print!("====  MODEL ENTRY");
    let txp = sbtr.p_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let txc = sbtr.c_tx_cn_m.iter().map(|(_, v)| v).sum::<u32>();
    let cnt = (txp + txc + sbtr.mt_1_ph as u32 + sbtr.mt_3_ph as u32) as f64;
    /*
    let mut cnt = (txp + txc + sbtr.mt_1_ph as u32 + sbtr.mt_3_ph as u32) as f64;
    if ben.mx_pw > 0f32 && ben.grw < 7f32 && ben.be_start <= 3 && ben.trlm > 40f32 {
        cnt += 1.0;
    }
    */
    let ent_cn = cnt * MODEL_ENTRY_RATIO as f64;
    let ent_ex = ent_cn * MODEL_ENTRY_COST as f64;

    let mut proj = vec![0.0, 0.0, 0.0];
    for y in 0..12 {
        let be = ent_ex;
        let be = be * Pow::pow(1f64 + CALL_CENTER_COST_UP as f64, y as f64);
        //print!(" {} - {be}", y + 2028);
        proj.push(be as f32);
    }
    proj
}

use crate::dcl::PeaSub;
use sglib03::prc4::SubYearBenInfo;
use sglib03::prc4::BC_BESS_YLEN;
use sglib03::prc4::BC_DISCN_RATE;
use sglib03::prc4::BC_SUBST_COST;
use sglib03::prc4::BC_SUBST_YLEN;

pub fn ben_bess_calc(
    _sbtr: &SubCalc,
    sb: &PeaSub,
    gr: f32,
    pwx: f32,
) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, f32) {
    use sglib03::prc4::BC_NO_DAY_IN_YEAR;
    use sglib03::prc4::BC_OFFPEAK_COST;
    use sglib03::prc4::BC_ON_PEAK_COST;
    use sglib03::prc4::BC_POWER_FACT;
    use sglib03::prc4::BC_SELL_PRICE;
    use sglib03::prc4::BC_TR_CRIT_LIM;
    use sglib03::prc4::BC_TR_LOAD_LIM;

    let mut sub_save = vec![0.0, 0.0, 0.0];
    let mut svg_save = vec![0.0, 0.0, 0.0];
    let mut dif_save = vec![0.0, 0.0, 0.0];
    let mut eng_save = vec![0.0, 0.0, 0.0];
    //let ben = ld_ben_bess1(&sbas.sbid);

    // ==============================================
    // ==============================================
    // ======= BEGIN =======
    // BENEFIT
    /*
    println!(
        "cate:{} state:{} conf:{} sbtp:{}",
        sb.cate, sb.state, sb.conf, sb.sbtp
    );
    */

    let grw = gr;
    let pwmx = pwx;

    /*
    let mut grw = 100f32 * ENERGY_GRW_RATE;
    let mut pwmx = 0f32;
    if let Some(reps) = &sb.lp_rep_24.pos_rep.val {
        for vv in reps.iter().flatten() {
            pwmx = pwmx.max(*vv);
        }
    };
    let mut pwmx0 = 0f32;
    if let Some(reps) = &sb.lp_rep_23.pos_rep.val {
        for vv in reps.iter().flatten() {
            pwmx0 = pwmx0.max(*vv);
        }
    };

    let grw2 = if pwmx0 > 0f32 {
        (pwmx - pwmx0) / pwmx * 100f32
    } else {
        0f32
    };
    if grw2 > grw && grw2 < 6f32 {
        grw = grw2;
    }
    */

    let trlm = sb.mvxn as f32 * BC_POWER_FACT * BC_TR_LOAD_LIM;
    let trcr = sb.mvxn as f32 * BC_POWER_FACT * BC_TR_CRIT_LIM;
    let dppy = trlm * grw / 100f32; // MW/yr increase
    let yrno = (trlm - pwmx) / dppy;
    let yrno = yrno as usize;
    let mut ls_ex_en = 0f32;

    if sb.sbtp == "AIS" && yrno < 6 {
        // day load profile
        let daylp = if let Some(reps) = &sb.lp_rep_24.pos_rep.val {
            reps.iter().flatten().cloned().collect::<Vec<_>>()
        } else {
            vec![0f32; 96]
        };

        use sglib03::prc4::BC_PROJ_YLEN;
        let mut yr_daypf = Vec::<Vec<f32>>::new();
        for i in 0..=BC_PROJ_YLEN {
            let mut day_prof = daylp.clone();
            for vapf in day_prof.iter_mut() {
                *vapf *= Pow::pow(1f32 + grw / 100f32, i as f32);
            }
            yr_daypf.push(day_prof);
        }

        let yr_start = yrno;
        let mxrt = pwmx / trlm * 100f32;

        let r = BC_DISCN_RATE / 100f32;
        let n = BC_SUBST_YLEN as f32;
        let anrt = (1f32 - Pow::pow(1f32 + r, -n)) / r;
        let ancs = BC_SUBST_COST / anrt;
        let cst: Vec<f64> = vec![ancs.into(); 25];
        let mut subcst = Vec::<SubYearBenInfo>::new();
        for (i, v) in cst.iter().enumerate() {
            let fa = v / Pow::pow(1f64 + r as f64, i as f64);
            let be = if i < 12 {
                v * Pow::pow(1.03f64, i as f64)
            } else {
                0f64
            };
            subcst.push(SubYearBenInfo {
                year: i,
                sub_cost: *v as f32,
                sub_npv: fa as f32,
                sub_save: be as f32,
                ..Default::default()
            });
        }

        let mut sbsav = 0f32;
        let be_start = if yr_start < 4 { 1 } else { yr_start - 3 };
        for _i in 1..be_start {
            sub_save.push(0f32);
        }
        for cst in subcst.iter().take(BC_BESS_YLEN).skip(be_start - 1) {
            sbsav += cst.sub_save;
            sub_save.push(cst.sub_save * 1_000_000f32);
        }

        // power and energy of the last year
        let mut ls_ex_sm = 0f32;
        let mut ls_ex_pw = 0f32;
        for tm_pf in yr_daypf[BC_PROJ_YLEN].iter() {
            let dv = tm_pf - trcr;
            if dv >= 0f32 {
                ls_ex_pw = dv.max(ls_ex_pw);
                ls_ex_sm += dv;
            }
        }
        ls_ex_en = ls_ex_sm * 0.25f32;
        if ls_ex_en > 20f32 {
            ls_ex_en = 20f32;
        }

        // load profile of year 2024
        let tm_pf: Vec<_> = if let Some(reps) = &sb.lp_rep_24.pos_rep.val {
            reps.iter().flatten().cloned().collect()
        } else {
            vec![0f32; 96]
        };
        let (p1, p2) = pow_calc_peak(&tm_pf);
        // energy, onpeak, offpeak
        let p_en: f32 = tm_pf.iter().sum();
        let en_onp = p1.p_en;
        let en_ofp = p2.p_en;

        let grw = 2f32;
        let en0 = p_en * Pow::pow(1f32 + grw / 100f32, yr_start as f32);
        let enn = en_onp * Pow::pow(1f32 + grw / 100f32, yr_start as f32);
        let enf = en_ofp * Pow::pow(1f32 + grw / 100f32, yr_start as f32);

        let mut be_en_added = Vec::<f32>::new();
        for _n in 3..=yr_start {
            //print!(" {_n}");
            be_en_added.push(0f32);
            eng_save.push(0f32);
        }
        let l1 = eng_save.len();

        let (mut _aen0, mut _aenn, mut _aenf) = (0f32, 0f32, 0f32);
        let uc_onp = BC_SELL_PRICE - BC_ON_PEAK_COST;
        let uc_ofp = BC_SELL_PRICE - BC_OFFPEAK_COST;

        //print!("   ");
        let yr0 = if yr_start == 0 { 2 } else { yr_start };
        let yr0 = if yr_start == 1 { 2 } else { yr0 };
        for n in yr0 + 1..BC_PROJ_YLEN {
            //print!(" {n}");
            let aennx = en_onp * Pow::pow(1f32 + grw / 100f32, n as f32) - enn;
            let aenfx = en_ofp * Pow::pow(1f32 + grw / 100f32, n as f32) - enf;

            let aenny = aennx * uc_onp * 1000f32 * BC_NO_DAY_IN_YEAR as f32;
            let aenfy = aenfx * uc_ofp * 1000f32 * BC_NO_DAY_IN_YEAR as f32;

            let aen = (aenny + aenfy) * 0.94f32;
            be_en_added.push(aen);
            eng_save.push(aen);
        }
        //println!();
        let l2 = eng_save.len();
        if l2 > 15 {
            println!(
                "=== sub: {}  yr_start: {yr_start}, yr0:{yr0} LEN:{BC_PROJ_YLEN} len:{l1}->{l2}",
                sb.sbid,
            );
        }

        let _en0 = en0 * BC_NO_DAY_IN_YEAR as f32;
        let _enn = enn * BC_NO_DAY_IN_YEAR as f32;
        let _enf = enf * BC_NO_DAY_IN_YEAR as f32;
        let ex_ben_onp = _aenn * uc_onp * 1000f32 * BC_NO_DAY_IN_YEAR as f32;
        let ex_ben_ofp = _aenf * uc_ofp * 1000f32 * BC_NO_DAY_IN_YEAR as f32;
        let _ex_ben = (ex_ben_onp + ex_ben_ofp) * 0.94f32;

        //let mut be_re_diff = Vec::<(u32, f32)>::new();
        let mut yr_diff = ls_ex_en * (BC_ON_PEAK_COST - BC_OFFPEAK_COST) * 1000f32;
        for _yi in 0..BC_BESS_YLEN {
            yr_diff *= 1.04;
            //be_re_diff.push((yi as u32 + 2028, yr_diff));
            dif_save.push(yr_diff * BC_NO_DAY_IN_YEAR as f32);
            //dif_save.push(55f32);
        }

        let _dec_ben =
            ls_ex_en * (BC_ON_PEAK_COST - BC_OFFPEAK_COST) * 1000f32 * BC_NO_DAY_IN_YEAR as f32;

        //let mut pkt = trcr;
        let pkt = pwmx;
        let qbes = (pkt - trlm) * 0.4663; // tan 25
        let qbes = if qbes < 0f32 { 0f32 } else { qbes };
        let qcst = qbes * 4f32; // 4 million bht
                                //println!("{qbes}- {qcst}");
        let _r = BC_DISCN_RATE / 100f32;
        //let fa = qcsy / Pow::pow(1f32 + r, 10f32);
        //
        let mut be_svg_save = Vec::<f32>::new();
        for _y in 3..=yr_start {
            be_svg_save.push(0f32);
            svg_save.push(0f32);
        }
        let l1 = svg_save.len();
        let mut _ben3 = 0f32;
        let yr1 = if l1 == 3 { 2 } else { yr_start };
        for n in yr1 + 1..BC_PROJ_YLEN {
            //for y in yr_start..=BC_PROJ_YLEN - 3 {
            //for y in yr_start..=BC_PROJ_YLEN - 3 {
            let be = qcst / 10f32 * Pow::pow(1.03f32, n as f32);
            be_svg_save.push(be * 1_000_000f32);
            _ben3 += be;
            svg_save.push(be * 1_000_000f32);
        }
        let _l2 = svg_save.len();
        //print!("SVG qbes:{qbes} pkt:{pkt} trlm:{trlm} qcst:{qcst} l1:{l1} l2:{l2}");

        use sglib03::prc4::BC_PEA_PROFIT;

        let mut _ac_ex_sm = 0f32;
        let mut _ac_ex_be = 0f32;
        for yr_daypf in yr_daypf.iter() {
            for (i, tm_pf) in yr_daypf.iter().enumerate() {
                let dv = tm_pf - trlm;
                if dv >= 0f32 {
                    let up = if (BC_ON_PEAK_BEGIN..BC_ON_PEAK_END).contains(&i) {
                        let df = BC_ON_PEAK_COST - BC_OFFPEAK_COST;
                        df + BC_PEA_PROFIT
                    } else {
                        BC_PEA_PROFIT
                    };
                    _ac_ex_sm += dv;
                    _ac_ex_be += dv * up * 0.5f32;
                }
            }
        }
        _ac_ex_sm *= BC_NO_DAY_IN_YEAR as f32;
        _ac_ex_be *= BC_NO_DAY_IN_YEAR as f32;
        println!( "====  trlm:{trlm} trcr:{trcr} pwmx:{pwmx} yrno:{yrno} grw:{grw} mxrt:{mxrt} sbsav:{sbsav}");
    }
    // ======= END =======
    // ==============================================
    // ==============================================
    (sub_save, svg_save, dif_save, eng_save, ls_ex_en)
}

pub const BC_ON_PEAK_BEGIN: usize = 18 * 4;
pub const BC_ON_PEAK_END: usize = 22 * 4;
use sglib03::prc2::PowerCalc;

pub fn pow_calc_peak(time_v: &[f32]) -> (PowerCalc, PowerCalc) {
    let mut pwn = PowerCalc::default();
    let mut pwf = PowerCalc::default();
    for (i, v) in time_v.iter().enumerate() {
        if (BC_ON_PEAK_BEGIN..BC_ON_PEAK_END).contains(&i) {
            if *v >= 0f32 {
                pwn.p_sum += *v;
                pwn.p_cnt += 1;
                if *v > pwn.p_pk {
                    pwn.p_pk = *v;
                }
            } else {
                pwn.n_sum += -*v;
                pwn.n_cnt += 1;
                if -*v > pwn.n_pk {
                    pwn.n_pk = -*v;
                }
            }
        } else if *v >= 0f32 {
            pwf.p_sum += *v;
            pwf.p_cnt += 1;
            if *v > pwf.p_pk {
                pwf.p_pk = *v;
            }
        } else {
            pwf.n_sum += -*v;
            pwf.n_cnt += 1;
            if -*v > pwf.n_pk {
                pwf.n_pk = -*v;
            }
        }
    }
    pwn.p_en = pwn.p_sum / 2f32;
    pwn.n_en = pwn.n_sum / 2f32;
    if pwn.p_cnt > 0 {
        pwn.p_avg = pwn.p_sum / pwn.p_cnt as f32;
    }
    if pwn.n_cnt > 0 {
        pwn.n_avg = pwn.n_sum / pwn.n_cnt as f32;
    }
    pwf.p_en = pwf.p_sum / 2f32;
    pwf.n_en = pwf.n_sum / 2f32;
    if pwf.p_cnt > 0 {
        pwf.p_avg = pwf.p_sum / pwf.p_cnt as f32;
    }
    if pwf.n_cnt > 0 {
        pwf.n_avg = pwf.n_sum / pwf.n_cnt as f32;
    }
    (pwn, pwf)
}
