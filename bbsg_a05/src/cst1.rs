use sglib04::prc41::SubCalc;
use sglib04::web1::COMM_COST;
use sglib04::web1::ESS_COST;
use sglib04::web1::ESS_OP_COST;
use sglib04::web1::M1P_COST;
use sglib04::web1::M1P_IMP_COST;
use sglib04::web1::M1P_OP_COST;
use sglib04::web1::M3P_COST;
use sglib04::web1::M3P_IMP_COST;
use sglib04::web1::M3P_OP_COST;
use sglib04::web1::PLATFORM_COST;
use sglib04::web1::PLATFORM_OP_COST;
use sglib04::web1::TRX_COST;
use sglib04::web1::TRX_IMP_COST;
use sglib04::web1::TRX_OP_COST;

pub fn cst_m1p_ins(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let cst = M1P_COST * sbtr.mt_1_ph as f32 / 3.0;
    let cst = M1P_COST * no / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_m3p_ins(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let cst = M3P_COST * sbtr.mt_3_ph as f32 / 3.0;
    let cst = M3P_COST * no / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_tr_ins(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    //let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    //let cst = TRX_COST * (trp + trc) / 3.0;
    let cst = TRX_COST * no / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_bes_ins(_sbtr: &SubCalc, bescap: f32) -> Vec<f32> {
    let cst = ESS_COST * bescap / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_plfm_ins(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    //let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    //let cnt = sbtr.mt_1_ph as f32 + sbtr.mt_3_ph as f32 + trp + trc;
    //let cnt = if bescap > 0f32 { cnt + 1.0 } else { cnt };
    //let cst = PLATFORM_COST * cnt / 3.0;
    let cst = PLATFORM_COST * no / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}
pub fn cst_comm_ins(_sbtr: &SubCalc, _bescap: f32) -> Vec<f32> {
    vec![0.0]
}

pub fn cst_m1p_imp(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let cst = M1P_IMP_COST * sbtr.mt_1_ph as f32 / 3.0;
    let cst = M1P_IMP_COST * no as f32 / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_m3p_imp(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let cst = M3P_IMP_COST * sbtr.mt_3_ph as f32 / 3.0;
    let cst = M3P_IMP_COST * no as f32 / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_tr_imp(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    //let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    //let cst = TRX_IMP_COST * (trp + trc) / 3.0;
    let cst = TRX_IMP_COST * no / 3.0;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(cst);
    }
    csts
}

pub fn cst_bes_imp(_sbtr: &SubCalc, _bescap: f32) -> Vec<f32> {
    vec![0.0]
}
pub fn cst_plfm_imp(_sbtr: &SubCalc, _bescap: f32) -> Vec<f32> {
    vec![0.0]
}
pub fn cst_comm_imp(_sbtr: &SubCalc, _bescap: f32) -> Vec<f32> {
    vec![0.0]
}

pub const OP_INC_RATE: f32 = 0.03;

pub fn cst_reinvest(reinv: f32) -> Vec<f32> {
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        csts.push(reinv);
    }
    csts
}

pub fn cst_m1p_op(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let cst = M1P_OP_COST * sbtr.mt_1_ph as f32;
    let mut cst = M1P_OP_COST * no;
    for _i in 0..3 {
        cst *= 1.0 + OP_INC_RATE;
    }
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        cst *= 1.0 + OP_INC_RATE;
        csts.push(cst);
    }
    csts
}

pub fn cst_m3p_op(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let cst = M3P_OP_COST * sbtr.mt_3_ph as f32;
    let mut cst = M3P_OP_COST * no;
    for _i in 0..3 {
        cst *= 1.0 + OP_INC_RATE;
    }
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        cst *= 1.0 + OP_INC_RATE;
        csts.push(cst);
    }
    csts
}

pub fn cst_tr_op(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    //let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    //let cst = TRX_OP_COST * (trp + trc);
    let mut cst = TRX_OP_COST * no;
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..3 {
        cst *= 1.0 + OP_INC_RATE;
    }
    for _i in 0..12 {
        cst *= 1.0 + OP_INC_RATE;
        csts.push(cst);
    }
    csts
}

pub fn cst_bes_op(_sbtr: &SubCalc, bescap: f32) -> Vec<f32> {
    let mut cst = bescap * ESS_OP_COST / 3.0;
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..3 {
        cst *= 1.0 + OP_INC_RATE;
    }
    for _i in 0..12 {
        cst *= 1.0 + OP_INC_RATE;
        csts.push(cst);
    }
    csts
}

pub fn cst_plfm_op(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    //let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    //let cnt = sbtr.mt_1_ph as f32 + sbtr.mt_3_ph as f32 + trp + trc;
    //let cnt = if bescap > 0f32 { cnt + 1.0 } else { cnt };
    //let mut cst = PLATFORM_OP_COST * cnt;
    let mut cst = PLATFORM_OP_COST * no;
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..3 {
        cst *= 1.0 + OP_INC_RATE;
    }
    for _i in 0..12 {
        cst *= 1.0 + OP_INC_RATE;
        csts.push(cst);
    }
    csts
}

pub fn cst_comm_op(_sbtr: &SubCalc, no: f32) -> Vec<f32> {
    //let trp: f32 = sbtr.p_tx_cn_m.values().map(|v| *v as f32).sum();
    //let trc: f32 = sbtr.c_tx_cn_m.values().map(|v| *v as f32).sum();
    //let cnt = sbtr.mt_1_ph as f32 + sbtr.mt_3_ph as f32 + trp + trc;
    //let cnt = if bescap > 0f32 { cnt + 1.0 } else { cnt };
    // 12 montsh
    //let mut cst = COMM_COST * cnt * 12.0;
    let mut cst = COMM_COST * no * 12.0;
    for _i in 0..3 {
        cst *= 1.0 + OP_INC_RATE;
    }
    let mut csts = vec![0.0, 0.0, 0.0];
    for _i in 0..12 {
        cst *= 1.0 + OP_INC_RATE;
        csts.push(cst);
    }
    csts
}

pub fn eir_cust_loss_save(no: f32) -> Vec<f32> {
    let cst = no * 0.01;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(0f32);
    }
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
pub fn eir_cust_save(no: f32) -> Vec<f32> {
    let cst = no * 0.05;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(0f32);
    }
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
pub fn eir_ghg_save(no: f32) -> Vec<f32> {
    let cst = no * 0.10;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(0f32);
    }
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
pub fn eir_cust_mv_rev(no: f32) -> Vec<f32> {
    let cst = no * 0.13;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(0f32);
    }
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
pub fn eir_cust_ev_save(no: f32) -> Vec<f32> {
    let cst = no * 0.09;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(0f32);
    }
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
pub fn eir_cust_etruck_save(no: f32) -> Vec<f32> {
    let cst = no * 0.08;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(0f32);
    }
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
pub fn eir_cust_solar_roof(no: f32) -> Vec<f32> {
    let cst = no * 0.06;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(0f32);
    }
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
pub fn eir_en_rev_save(no: f32) -> Vec<f32> {
    let cst = no * 0.10;
    let mut csts = Vec::<f32>::new();
    for _i in 0..3 {
        csts.push(0f32);
    }
    for _i in 0..12 {
        csts.push(cst);
    }
    csts
}
