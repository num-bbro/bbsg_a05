use bincode::{Decode, Encode};
use phf_macros::phf_map;
use std::collections::HashMap;
use strum_macros::EnumIter;

pub const TRF_LOSS_RATIO: f32 = 0.03;
pub const TRF_UNBAL_K: f32 = 1.0f32;
pub const TRF_UNBAL_CNT_RATE: f32 = 0.8f32;
pub const SCURV_YEAR_BEG: usize = 2021;
pub const SSHOW_YEAR_BEG: usize = 2025;
pub const SSHOW_YEAR_END: usize = 2039;
//pub const EV_AT_2050: f32 = 6_000_000f32;
// personal car, personal truck, taxi
pub const EV_AT_2050: f32 = 11_734_944.0 * 0.5 + 7_109_198.0 * 0.5 + 81_296.0 * 0.5;
//pub const ET_AT_2050: f32 = 1_371_464.0 * 0.5;
pub const ET_AT_2050: f32 = 1_371_464.0 * 0.01;
// personal bike + taxi bike
pub const EB_AT_2050: f32 = 22_490_445.0 * 0.5 + 133_298.0 * 0.5;

pub const EV_HR_DAY: f32 = 3.0;
pub const EV_MWH_BAHT: f32 = 1000f32;
pub const RE_SCURV_BEG: usize = 2018;
pub const DNM: &str = "/mnt/e/CHMBACK/pea-data/sgdt-a04";
pub const EN_AVG_GRW_RATE: f32 = 3.6f32;
pub const EN_MAX_GRW_RATE: f32 = 5.0f32;
pub const RE_MV2HV_RATIO: f32 = 0.0986;

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct Pea {
    pub aream: HashMap<String, PeaArea>,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct PeaArea {
    pub arid: String,
    pub provm: HashMap<String, PeaProv>,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct PeaProv {
    pub pvnm: String,
    pub gppv: f32,
    pub evpc: f32,
    pub subm: HashMap<String, PeaSub>,
}
use crate::p03::SubLoadProfRepr;

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct PeaSub {
    pub sbid: String,
    pub feedm: HashMap<String, PeaFeed>,
    pub name: String,
    pub enam: String,
    pub area: String,
    pub arid: String,
    pub volt: String,
    pub cate: String,
    pub egat: String,
    pub state: String,
    pub conf: String,
    pub trax: String,
    pub mvax: String,
    pub feed: String,
    pub feno: usize,
    pub feeders: Vec<String>,
    pub trxn: usize,
    pub mvxn: i32,
    pub prov: String,
    pub sbtp: String,
    pub n1d_s: u64,
    pub n1d_f: u64,
    pub lp_rep_23: SubLoadProfRepr,
    pub lp_rep_24: SubLoadProfRepr,
    pub vspps: Vec<usize>,
    pub spps: Vec<usize>,
    pub repls: Vec<usize>,
    pub aojv: Vec<AojObj>,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct EvDistCalc {
    pub id: String,
    pub ev_no: f32,
    pub ev_pc: f32,
    pub ev_ds: f32,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct PeaFeed {
    pub fdid: String,
    pub tranm: HashMap<u64, PeaTrans>,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct PeaTrans {
    pub mets: Vec<PeaMeter>,
    pub trid: String,
    pub pea: String,
    pub n1d: u64,
    pub n1d_f: u64,
    pub ix: usize,
    pub lix: usize,
    pub own: String,
    pub mts: Vec<usize>,
    pub aojs: Vec<usize>,
    pub amps: Vec<usize>,
    pub muns: Vec<usize>,
    pub zons: Vec<usize>,
    pub sols: Vec<usize>,
    pub vols: Vec<usize>,
    pub vopw: f32,
    pub vose: f32,
    pub kw: f32,

    pub tr_tag: Option<String>,
    pub tr_fid: Option<String>,
    pub tr_lt: Option<f32>,
    pub tr_ln: Option<f32>,
    pub tr_cd: Option<f32>,
    pub tr_aoj: Option<String>,
    pub tr_pea: Option<String>,
    pub tr_kva: Option<f32>,
    pub tr_own: Option<String>,
    pub tr_loc: Option<String>,
    pub tr_n1d: Option<u64>,
    //pub ar: String,
    //pub ly: String,
    //pub ix: usize,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct PeaMeter {
    pub mt_ins: Option<String>,
    pub mt_pea: Option<String>,
    pub mt_tag: Option<String>,
    pub mt_phs: Option<String>,
    pub mt_x: Option<f32>,
    pub mt_y: Option<f32>,
    pub mt_lt: Option<f32>,
    pub mt_ln: Option<f32>,
    pub mt_aoj: Option<String>,
    pub tr_tag: Option<String>,
    pub tr_fid: Option<String>,
    pub tr_lt: Option<f32>,
    pub tr_ln: Option<f32>,
    pub tr_cd: Option<f32>,
    pub tr_aoj: Option<String>,
    pub tr_pea: Option<String>,
    pub tr_kva: Option<f32>,
    pub tr_own: Option<String>,
    pub tr_loc: Option<String>,
    pub tr_n1d: Option<u64>,
    pub mt_n1d: Option<u64>,
    pub ar: String,
    pub ly: String,
    pub ix: usize,
    //pub bills: Vec<PeaBill>,
    pub trsg: String,
    pub pea: String,
    pub ca: String,
    pub inst: String,
    pub rate: String,
    pub volt: String,
    pub mru: String,
    pub mat: String,
    pub main: String,
    pub kwh15: f32,
    pub kwh18: f32,
    pub amt19: f32,
    pub idx: usize,
    pub meth: i32,
    pub met_type: MeterAccType,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub enum MeterAccType {
    #[default]
    Small,
    Large,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub enum GridLevel {
    #[default]
    DisTrans,
    Feeder,
    Sub,
    Area,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub enum SumType {
    #[default]
    Sum,
    Max,
    Min,
}

#[derive(Encode, Decode, Debug, Clone, Default, EnumIter)]
pub enum VarType {
    #[default]
    None,
    NewCarReg,
    Gpp,
    MaxPosPowSub,
    MaxNegPowSub,
    VsppMv,
    SppHv,
    BigLotMv,
    BigLotHv,
    SubPowCap,
    PowTrSat,

    MaxPosPowFeeder,
    MaxNegPowFeeder,
    MaxPosDiffFeeder,
    MaxNegDiffFeeder,
    EnGrowth,
    NoMeterTrans,
    SmallSellTr,
    AllSellTr,
    ChgStnCapTr,
    ChgStnSellTr,
    PwCapTr,
    ZoneTr,
    PopTr,
    UnbalPowTr,
    PkPowTr,
    LargeSellTr,
    AllNoMeterTr,

    NoMet1Ph,
    NoMet3Ph,
    NoTr,
    NoPeaTr,
    NoCusTr,
    BessMWh,
    NoBess,
    NoDevice,

    LvPowSatTr,
    CntLvPowSatTr,
    ChgStnCap,
    ChgStnSell,
    MvPowSatTr,
    SolarRoof,
    MvVspp,
    HvSpp,
    SmallSell,
    LargeSell,
    UnbalPow,
    CntUnbalPow,
    Uc1Val,
    Uc2Val,
    Uc3Val,
    Uc1Rank,
    Uc2Rank,
    Uc3Rank,

    HmChgEvTr,
    NoHmChgEvTr,
    PowHmChgEvTr,

    ChgEtTr,
    NoEtTr,
    //PowEtChgTr,
    ChgEbTr,
    NoEbTr,

    PkSelPowPhsAKw,
    PkSelPowPhsBKw,
    PkSelPowPhsCKw,
    PkSelPowPhsAvg,
    PkSelPowPhsMax,
    UnbalPowRate,
    TransLossKw,
    UnbalPowLossKw,
    CntTrUnbalLoss,
    CntTrSatLoss,
    TakeNote,
    /// How likely the province to have EV car
    EvCarLikely,
    /// How likely the province to be select
    SelectLikely,
    SubSolarPeekMw,
    SubSolarEnergy,
    SolarEnergy,

    FirBilAccu,
    FirCashFlow,
    FirDRSave,
    FirBatSubSave,
    FirBatSvgSave,
    FirBatEnerSave,
    FirBatPriceDiff,
    FirMetBoxSave,
    FirLaborSave,
    FirMetSell,
    FirEMetSave,
    FirMetReadSave,
    FirMetDisSave,
    FirTouSell,
    FirTouReadSave,
    FirTouUpdateSave,
    FirOutLabSave,
    FirComplainSave,
    FirAssetValue,
    FirDataEntrySave,

    FirEvChgThb,
    FirMvReThb,
    FirUnbSave,
    FirTrSatSave,
    FirTrPhsSatSave,
    FirNonTechLoss,
    FirEtChgThb,
    FirEbChgThb,

    EirCustLossSave,
    EirConsumSave,
    EirGrnHsEmsSave,
    EirCustMvRev,
    EirCustEvSave,
    EirCustEtrkSave,
    EirSolaRfTopSave,
    EirEnerResvSave,

    CstMet1pIns,
    CstMet3pIns,
    CstTrIns,
    CstBessIns,
    CstPlfmIns,
    CstCommIns,

    CstMet1pImp,
    CstMet3pImp,
    CstTrImp,
    CstBessImp,
    CstPlfmImp,
    CstCommImp,

    CstMet1pOp,
    CstMet3pOp,
    CstTrOp,
    CstBessOp,
    CstPlfmOp,
    CstCommOp,
    CstReinvest,

    CstCapEx,
    CstOpEx,
    CstCapOpEx,
    FirSum,
    EirSum,

    FirCstRate,
    EirCstRate,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct AssVar {
    pub v: f32,
    pub l: GridLevel,
    pub t: VarType,
    pub s: SumType,
}

#[derive(Encode, Decode, Debug, Clone, Default)]
pub struct PeaAssVar {
    pub arid: String,
    pub pvid: String,
    pub sbid: String,
    pub fdid: String,
    pub n1d: u64,
    pub own: String,
    pub peano: String,
    pub aoj: String,
    pub aojv: Vec<AojObj>,
    pub set: u32,
    pub v: Vec<AssVar>,
    pub res: f32,
    pub vy: Vec<Vec<f32>>,
}

pub trait Pan {
    fn san(v: &str) -> String;
    fn pan0(&self) -> String;
    fn pan2(&self) -> String;
    fn pan3(&self) -> String;
    fn pan(&self, i: i32) -> String;
}

pub const WE_EV: [(VarType, f32); 8] = [
    (VarType::NewCarReg, 0.15),
    (VarType::Gpp, 0.15),
    (VarType::NoMeterTrans, 0.05),
    (VarType::SmallSellTr, 0.20),
    (VarType::ChgStnCapTr, 0.15),
    (VarType::ChgStnSellTr, 0.10),
    (VarType::ZoneTr, 0.10),
    (VarType::PopTr, 0.10),
];

pub const WE_RE: [(VarType, f32); 5] = [
    (VarType::Gpp, 0.20),
    (VarType::NoMeterTrans, 0.10),
    (VarType::SmallSellTr, 0.30),
    (VarType::ZoneTr, 0.20),
    (VarType::PopTr, 0.20),
];

pub const WE_ET: [(VarType, f32); 5] = [
    (VarType::Gpp, 0.20),
    (VarType::NoMeterTrans, 0.20),
    (VarType::AllSellTr, 0.20),
    (VarType::ZoneTr, 0.20),
    (VarType::PopTr, 0.20),
];

pub const WE_EB: [(VarType, f32); 5] = [
    (VarType::Gpp, 0.20),
    (VarType::NoMeterTrans, 0.20),
    (VarType::AllSellTr, 0.20),
    (VarType::ZoneTr, 0.20),
    (VarType::PopTr, 0.20),
];

pub const WE_UC1: [(VarType, f32); 11] = [
    (VarType::SmallSellTr, 0.05),
    (VarType::HmChgEvTr, 0.28),
    (VarType::CntLvPowSatTr, 0.15),
    (VarType::ChgStnCap, 0.05),
    //(VarType::MvPowSatTr, 0.05),
    (VarType::PowTrSat, 0.05),
    (VarType::SolarRoof, 0.15),
    (VarType::ZoneTr, 0.05),
    (VarType::PopTr, 0.05),
    (VarType::MvVspp, 0.05),
    (VarType::HvSpp, 0.02),
    (VarType::CntUnbalPow, 0.10),
];

pub const WE_UC2: [(VarType, f32); 10] = [
    (VarType::SmallSellTr, 0.10),
    (VarType::HmChgEvTr, 0.10),
    (VarType::CntLvPowSatTr, 0.15),
    (VarType::ChgStnCap, 0.05),
    (VarType::PowTrSat, 0.15),
    //(VarType::MvPowSatTr, 0.15),
    (VarType::SolarEnergy, 0.15),
    (VarType::ZoneTr, 0.05),
    (VarType::PopTr, 0.05),
    (VarType::MvVspp, 0.15),
    (VarType::HvSpp, 0.05),
    //(VarType::CntUnbalPow, 0.10),
    //    (VarType::ChgStnSell, 0.10),
    //(VarType::UnbalPow, 0.05)2
    //(VarType::LargeSellTr, 0.10),
    //    (VarType::SelectLikely, 0.10),
];

pub const WE_UC3: [(VarType, f32); 8] = [
    (VarType::SolarRoof, 0.25),
    (VarType::HmChgEvTr, 0.25),
    (VarType::SmallSellTr, 0.10),
    (VarType::CntLvPowSatTr, 0.10),
    (VarType::CntUnbalPow, 0.10),
    (VarType::MvVspp, 0.10),
    (VarType::ZoneTr, 0.05),
    (VarType::PopTr, 0.05),
    //(VarType::UnbalPow, 0.05),
];

pub static EV_LIKELY: phf::Map<&'static str, f32> = phf_map! {
"ระยอง" => 1f32,
"ชลบุรี" => 1f32,
"ปทุมธานี" => 1f32,
"สมุทรสาคร" => 1f32,
"นครปฐม" => 1f32,
"สงขลา" => 1f32,
"พระนครศรีอยุธยา" => 1f32,
"สระบุรี" => 1f32,
"เชียงใหม่" => 1f32,
"ฉะเชิงเทรา" => 1f32,
"นครราชสีมา" => 1f32,
"ราชบุรี" => 1f32,
"ขอนแก่น" => 1f32,
"ปราจีนบุรี" => 1f32,
"พิษณุโลก" => 1f32,
"สุราษฎร์ธานี" => 1f32,
"นครสวรรค์" => 1f32,
"เพชรบุรี" => 1f32,
"ภูเก็ต" => 1f32,
};

pub static SELE_LIKELY: phf::Map<&'static str, f32> = phf_map! {
"ระยอง" => 1f32,
"ชลบุรี" => 1f32,
"ปทุมธานี" => 1f32,
"สมุทรสาคร" => 1f32,
"นครปฐม" => 1f32,
"สงขลา" => 1f32,
"พระนครศรีอยุธยา" => 1f32,
"สระบุรี" => 1f32,
"เชียงใหม่" => 1f32,
"ฉะเชิงเทรา" => 1f32,
"นครราชสีมา" => 1f32,
"ราชบุรี" => 1f32,
"ขอนแก่น" => 1f32,
"ปราจีนบุรี" => 1f32,
"พิษณุโลก" => 1f32,
"สุราษฎร์ธานี" => 1f32,
"นครสวรรค์" => 1f32,
"เพชรบุรี" => 1f32,
"ภูเก็ต" => 1f32,
};

#[derive(Encode, Decode, PartialEq, Debug, Clone, Default)]
pub struct SubAssObj {
    pub sbid: String,
    pub sbth: String,
    pub sben: String,
    pub arid: String,
    pub prov: String,
    pub cpmw: f32,
    pub ld21: Vec<f32>,
    pub ld22: Vec<f32>,
    pub ld23: Vec<f32>,
    pub ld24: Vec<f32>,
    pub mx21: f32,
    pub mx22: f32,
    pub mx23: f32,
    pub mx24: f32,
    pub av21: f32,
    pub av22: f32,
    pub av23: f32,
    pub av24: f32,
    pub trpe: usize,
    pub trcu: usize,
    pub mtpe: usize,
    pub mtcu: usize,
    pub mt13: usize,
    pub mt45: usize,
    pub se_s: f32,
    pub se_l: f32,
    pub se_2: f32,
    pub sell: f32,
    pub evca: f32,
    pub gpp: f32,
    pub psat: f32,
    pub vopw: f32,
    pub vose: f32,
    pub dens: f32,
    pub zone: f32,
    pub sorf: f32,
    pub vspkw: f32,
    pub sppmw: f32,
    pub unbal: f32,
    pub repln: f32,
    pub note: i32,
    pub aojv: Vec<AojObj>,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone, Default)]
pub struct AojObj {
    pub code: String,
    pub sht_name: String,
    pub office: String,
    pub pea: String,
    pub aoj_sz: String,
    pub reg: String,
    pub name: String,
    pub level: f32,
    pub trcn: usize,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone, Default)]
pub struct SubAssObj2 {
    pub sbid: String,
    pub prov: String,
    pub arid: String,
    pub ev1: f32,
    pub ev2: f32,
    pub ev3: f32,
    pub ev4: f32,
    pub ev5: f32,
    pub re1: f32,
    pub re2: f32,
    pub re3: f32,
    pub en1: f32,
    pub en2: f32,
    pub en3: f32,
    pub en4: f32,
    pub sum: f32,
    pub rank: usize,
}

//use crate::p03::SubLoadProfRepr;
use crate::p04::SubFeedTrans;
//use crate::p08::ld_sub_info;
use crate::p08::SubInfo;
use sglab02_lib::sg::wk5::EvDistCalc as OldEvDistCalc;
use sglib04::geo1::CnlData;
use sglib04::geo1::MeterBill;
use sglib04::geo2::CnlTrans;
use sglib04::geo2::SppData;
use sglib04::geo2::VoltaStation;
use sglib04::geo2::VsppData;
use sglib04::geo3::GisAoj;
use sglib04::geo3::GisZone;
use sglib04::geo3::PopuDenseSave;
use sglib04::geo4::LowVoltSolar;
use sglib04::geo4::REPlan;
use sglib04::ld1::RepLoadProf;

//#[derive(Encode, Decode, Debug, Clone, Default)]
#[derive(Debug, Clone, Default)]
pub struct ProcEngine {
    pub subs: Vec<SubFeedTrans>,
    pub ctrs: Vec<CnlTrans>,
    pub cmts: Vec<CnlData>,
    pub bils: Vec<MeterBill>,
    pub m2bs: Vec<Vec<usize>>,
    pub vols: Vec<VoltaStation>,
    pub votr: Vec<Vec<usize>>,
    pub vsps: Vec<VsppData>,
    pub vssb: Vec<Vec<usize>>,
    pub spps: Vec<SppData>,
    pub spsb: Vec<Vec<usize>>,
    pub zons: Vec<GisZone>,
    pub zntr: Vec<Vec<usize>>,
    pub aojs: Vec<GisAoj>,
    pub aotr: Vec<Vec<usize>>,
    pub amps: Vec<PopuDenseSave>,
    pub amtr: Vec<Vec<usize>>,
    pub muni: Vec<PopuDenseSave>,
    pub mutr: Vec<Vec<usize>>,
    pub repl: Vec<REPlan>,
    pub resb: Vec<Vec<usize>>,
    pub sola: Vec<LowVoltSolar>,
    pub sotr: Vec<Vec<usize>>,
    pub sblp: Vec<RepLoadProf>,
    pub fdlp: Vec<RepLoadProf>,
    pub carg: HashMap<String, f64>,
    pub evpv: HashMap<String, OldEvDistCalc>,
    //pub sbif: HashMap<String, SubstInfo>,
    pub sbif: HashMap<String, SubInfo>,
    pub lp23: HashMap<String, SubLoadProfRepr>,
    pub lp24: HashMap<String, SubLoadProfRepr>,
}

//#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone, Default)]
//#[derive(Encode, Decode, Debug, Clone, Default)]
#[derive(Debug, Clone, Default)]
pub struct AojInfo {
    pub code: String,
    pub sht_name: String,
    pub office: String,
    pub pea: String,
    pub aoj_sz: String,
    pub reg: String,
    pub name: String,
    pub level: f32,
    pub trcn: usize,
}

pub const EV_PRV_ADJ_2: [(&str, f64, f64); 26] = [
    ("สมุทรสาคร", 5.0, 0.0),
    ("พระนครศรีอยุธยา", 6.0, 0.0),
    ("ปทุมธานี", 12.0, 0.0),
    ("ชลบุรี", 6.0, 0.0),
    ("ระยอง", 6.0, 0.0),
    ("ฉะเชิงเทรา", 6.0, 0.0),
    ("นครปฐม", 6.0, 0.0),  // 6.0
    ("ปราจีนบุรี", 6.0, 0.0), // 7.0
    ("สงขลา", 5.0, 0.0),
    ("ราชบุรี", 5.0, 0.0),
    ("ภูเก็ต", 0.0, 3.0),
    ("นครสวรรค์", 3.0, 0.0),
    ("ระนอง", 2.0, 0.0),
    ("สมุทรสงคราม", 2.0, 0.0),
    ("กระบี่", 2.0, 0.0),
    ("เพชรบุรี", 2.0, 0.0),
    ("สุราษฎร์ธานี", 4.0, 0.0),
    ("สระบุรี", 3.0, 0.0),
    ("นครราชสีมา", 4.0, 0.0),
    ("เชียงใหม่", 4.0, 0.0),
    ("พิษณุโลก", 2.0, 0.0),
    ("ขอนแก่น", 5.0, 0.0),
    ("ลพบุรี", 2.0, 0.0),
    ("กรุงเทพมหานคร", 0.0, 30.0),
    ("นนทบุรี", 0.0, 25.0),
    ("สมุทรปราการ", 0.0, 15.0),
];

pub const FIR_FLDS: [VarType; 28] = [
    VarType::FirEvChgThb,
    VarType::FirEtChgThb,
    VarType::FirEbChgThb,
    VarType::FirMvReThb,
    VarType::FirUnbSave,
    VarType::FirTrSatSave,
    VarType::FirTrPhsSatSave,
    VarType::FirNonTechLoss,
    VarType::FirBilAccu,
    VarType::FirCashFlow,
    VarType::FirDRSave,
    VarType::FirBatSubSave,
    VarType::FirBatSvgSave,
    VarType::FirBatEnerSave,
    VarType::FirBatPriceDiff,
    VarType::FirMetBoxSave,
    VarType::FirLaborSave,
    VarType::FirMetSell,
    VarType::FirEMetSave,
    VarType::FirMetReadSave,
    VarType::FirMetDisSave,
    VarType::FirTouSell,
    VarType::FirTouReadSave,
    VarType::FirTouUpdateSave,
    VarType::FirOutLabSave,
    VarType::FirComplainSave,
    VarType::FirAssetValue,
    VarType::FirDataEntrySave,
];

pub const EIR_FLDS: [VarType; 8] = [
    VarType::EirCustLossSave,
    VarType::EirConsumSave,
    VarType::EirGrnHsEmsSave,
    VarType::EirCustMvRev,
    VarType::EirCustEvSave,
    VarType::EirCustEtrkSave,
    VarType::EirSolaRfTopSave,
    VarType::EirEnerResvSave,
];

pub const CAPEX_FLDS: [VarType; 13] = [
    VarType::CstMet1pIns,
    VarType::CstMet3pIns,
    VarType::CstTrIns,
    VarType::CstBessIns,
    VarType::CstPlfmIns,
    VarType::CstCommIns,
    VarType::CstMet1pImp,
    VarType::CstMet3pImp,
    VarType::CstTrImp,
    VarType::CstBessImp,
    VarType::CstPlfmImp,
    VarType::CstCommImp,
    VarType::CstReinvest,
];

pub const OPEX_FLDS: [VarType; 6] = [
    VarType::CstMet1pOp,
    VarType::CstMet3pOp,
    VarType::CstTrOp,
    VarType::CstBessOp,
    VarType::CstPlfmOp,
    VarType::CstCommOp,
];

pub const SHOW_FLDS: [VarType; 62] = [
    VarType::FirEvChgThb,
    VarType::FirEtChgThb,
    VarType::FirEbChgThb,
    VarType::FirMvReThb,
    VarType::FirUnbSave,
    VarType::FirTrSatSave,
    VarType::FirTrPhsSatSave,
    VarType::FirNonTechLoss,
    VarType::FirDataEntrySave,
    VarType::FirOutLabSave,
    VarType::FirComplainSave,
    VarType::FirBilAccu,
    VarType::FirCashFlow,
    VarType::FirDRSave,
    VarType::FirMetBoxSave,
    VarType::FirLaborSave,
    VarType::FirMetSell,
    VarType::FirEMetSave,
    VarType::FirMetReadSave,
    VarType::FirMetDisSave,
    VarType::FirTouSell,
    VarType::FirTouReadSave,
    VarType::FirTouUpdateSave,
    VarType::FirAssetValue,
    VarType::FirBatSubSave,
    VarType::FirBatSvgSave,
    VarType::FirBatEnerSave,
    VarType::FirBatPriceDiff,
    // ===== Cost
    VarType::CstMet1pIns,
    VarType::CstMet3pIns,
    VarType::CstTrIns,
    VarType::CstBessIns,
    VarType::CstPlfmIns,
    VarType::CstCommIns,
    VarType::CstMet1pImp,
    VarType::CstMet3pImp,
    VarType::CstTrImp,
    VarType::CstBessImp,
    VarType::CstPlfmImp,
    VarType::CstCommImp,
    VarType::CstMet1pOp,
    VarType::CstMet3pOp,
    VarType::CstTrOp,
    VarType::CstBessOp,
    VarType::CstPlfmOp,
    VarType::CstCommOp,
    VarType::CstReinvest,
    // ===== SUM
    VarType::CstCapEx,
    VarType::CstOpEx,
    VarType::CstCapOpEx,
    VarType::FirSum,
    VarType::FirCstRate,
    // ===== SUM
    VarType::EirCustLossSave,
    VarType::EirConsumSave,
    VarType::EirGrnHsEmsSave,
    VarType::EirCustMvRev,
    VarType::EirCustEvSave,
    VarType::EirCustEtrkSave,
    VarType::EirSolaRfTopSave,
    VarType::EirEnerResvSave,
    VarType::EirSum,
    VarType::EirCstRate,
];

pub const SHOW_FLDS2: [VarType; 29] = [
    VarType::FirSum,
    VarType::FirEvChgThb,
    VarType::FirEtChgThb,
    VarType::FirEbChgThb,
    VarType::FirMvReThb,
    VarType::FirUnbSave,
    VarType::FirTrSatSave,
    VarType::FirTrPhsSatSave,
    VarType::FirNonTechLoss,
    VarType::FirDataEntrySave,
    VarType::FirOutLabSave,
    VarType::FirComplainSave,
    VarType::FirBilAccu,
    VarType::FirCashFlow,
    VarType::FirDRSave,
    VarType::FirMetBoxSave,
    VarType::FirLaborSave,
    VarType::FirMetSell,
    VarType::FirEMetSave,
    VarType::FirMetReadSave,
    VarType::FirMetDisSave,
    VarType::FirTouSell,
    VarType::FirTouReadSave,
    VarType::FirTouUpdateSave,
    VarType::FirAssetValue,
    VarType::FirBatSubSave,
    VarType::FirBatSvgSave,
    VarType::FirBatEnerSave,
    VarType::FirBatPriceDiff,
];

pub const SHOW_FLDS3: [VarType; 33] = [
    VarType::CstCapEx,
    VarType::CstOpEx,
    VarType::CstCapOpEx,
    VarType::FirSum,
    VarType::EirSum,
    // ===== Scope
    VarType::NoTr,
    VarType::NoPeaTr,
    VarType::NoCusTr,
    VarType::NoMet1Ph,
    VarType::NoMet3Ph,
    VarType::BessMWh,
    VarType::NoBess,
    VarType::NoDevice,
    VarType::NoHmChgEvTr,
    // ===== Cost
    VarType::CstMet1pIns,
    VarType::CstMet3pIns,
    VarType::CstTrIns,
    VarType::CstBessIns,
    VarType::CstPlfmIns,
    VarType::CstCommIns,
    VarType::CstMet1pImp,
    VarType::CstMet3pImp,
    VarType::CstTrImp,
    VarType::CstBessImp,
    VarType::CstPlfmImp,
    VarType::CstCommImp,
    VarType::CstMet1pOp,
    VarType::CstMet3pOp,
    VarType::CstTrOp,
    VarType::CstBessOp,
    VarType::CstPlfmOp,
    VarType::CstCommOp,
    VarType::CstReinvest,
    // ===== SUM
];
