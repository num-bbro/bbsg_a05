use crate::dcl::*;
use std::error::Error;

#[allow(dead_code)]
pub fn write_ass_csv_01(tr_as: &Vec<PeaAssVar>, fnm: &str) -> Result<String, Box<dyn Error>> {
    let mut x = String::new();
    use std::fmt::Write;
    let flds = [
        VarType::NewCarReg,
        VarType::Gpp,
        VarType::MaxPosPowSub,
        VarType::MaxNegPowSub,
        VarType::VsppMv,
        VarType::SppHv,
        VarType::BigLotMv,
        VarType::BigLotHv,
        VarType::SubPowCap,
        VarType::MaxPosPowFeeder,
        VarType::MaxNegPowFeeder,
        VarType::MaxPosDiffFeeder,
        VarType::MaxNegDiffFeeder,
        VarType::NoMeterTrans,
        VarType::SmallSellTr,
        VarType::ChgStnCapTr,
        VarType::ChgStnSellTr,
        VarType::PwCapTr,
        VarType::ZoneTr,
        VarType::PopTr,
        VarType::UnbalPowTr,
        VarType::PkPowTr,
        VarType::LargeSellTr,
        VarType::AllNoMeterTr,
        VarType::NoTr,
        VarType::HmChgEvTr,
        VarType::LvPowSatTr,
        VarType::CntLvPowSatTr,
        VarType::ChgStnCap,
        VarType::ChgStnSell,
        VarType::MvPowSatTr,
        VarType::SolarRoof,
        VarType::MvVspp,
        VarType::HvSpp,
        VarType::SmallSell,
        VarType::LargeSell,
        VarType::UnbalPow,
        VarType::CntUnbalPow,
        VarType::Uc1Val,
        VarType::Uc2Val,
        VarType::Uc3Val,
        VarType::Uc1Rank,
        VarType::Uc2Rank,
        VarType::Uc3Rank,
        VarType::NoHmChgEvTr,
        VarType::PowHmChgEvTr,
    ];
    write!(x, "\"SUB\"")?;
    write!(x, ",\"FEEDER\"")?;
    write!(x, ",\"AOJ\"")?;
    write!(x, ",\"OWN\"")?;
    write!(x, ",\"PEANO\"")?;
    for f in flds.iter() {
        let l = format!("{f:?}");
        write!(x, ",\"{l}\"")?;
    }
    writeln!(x)?;
    for t in tr_as {
        //t._a("y:");
        write!(x, "\"{}\"", t.sbid)?;
        write!(x, ",\"{}\"", t.fdid)?;
        write!(x, ",\"{}\"", t.aoj)?;
        write!(x, ",\"{}\"", t.own)?;
        write!(x, ",\"{}\"", t.peano)?;
        write!(x, ",\"{}\"", t.v[VarType::NewCarReg as usize].v)?;
        for f in flds.iter() {
            let d = t.v[f.clone() as usize].v;
            write!(x, ",{d}")?;
        }
        writeln!(x)?;
    }
    println!("        ===== write to {fnm}");
    let b = x.as_bytes();
    let h = sha256::digest(b);
    std::fs::write(fnm, b)?;

    Ok(h)
}

#[allow(dead_code)]
pub fn write_ass_csv_02(tr_as: &Vec<PeaAssVar>, fnm: &str) -> Result<String, Box<dyn Error>> {
    let mut x = String::new();
    use std::fmt::Write;
    let flds = [
        VarType::NewCarReg,
        VarType::Gpp,
        VarType::MaxPosPowSub,
        VarType::MaxNegPowSub,
        VarType::VsppMv,
        VarType::SppHv,
        VarType::BigLotMv,
        VarType::BigLotHv,
        VarType::SubPowCap,
        VarType::MaxPosPowFeeder,
        VarType::MaxNegPowFeeder,
        VarType::MaxPosDiffFeeder,
        VarType::MaxNegDiffFeeder,
        VarType::NoMeterTrans,
        VarType::SmallSellTr,
        VarType::ChgStnCapTr,
        VarType::ChgStnSellTr,
        VarType::PwCapTr,
        VarType::ZoneTr,
        VarType::PopTr,
        VarType::UnbalPowTr,
        VarType::PkPowTr,
        VarType::LargeSellTr,
        VarType::AllNoMeterTr,
        VarType::NoTr,
        VarType::HmChgEvTr,
        VarType::LvPowSatTr,
        VarType::CntLvPowSatTr,
        VarType::ChgStnCap,
        VarType::ChgStnSell,
        VarType::MvPowSatTr,
        VarType::SolarRoof,
        VarType::MvVspp,
        VarType::HvSpp,
        VarType::SmallSell,
        VarType::LargeSell,
        VarType::UnbalPow,
        VarType::CntUnbalPow,
        VarType::Uc1Val,
        VarType::Uc2Val,
        VarType::Uc3Val,
        VarType::Uc1Rank,
        VarType::Uc2Rank,
        VarType::Uc3Rank,
        VarType::NoHmChgEvTr,
        VarType::PowHmChgEvTr,
        VarType::SolarEnergy,
    ];
    write!(x, "\"SUB\"")?;
    write!(x, ",\"PROV\"")?;
    write!(x, ",\"ARID\"")?;
    for f in flds.iter() {
        let l = format!("{f:?}");
        write!(x, ",\"{l}\"")?;
    }
    writeln!(x)?;
    for t in tr_as {
        //t._a("y:");
        write!(x, "\"{}\"", t.sbid)?;
        write!(x, ",\"{}\"", t.pvid)?;
        write!(x, ",\"{}\"", t.arid)?;
        for f in flds.iter() {
            let d = t.v[f.clone() as usize].v;
            write!(x, ",{d}")?;
        }
        writeln!(x)?;
    }
    println!("        ===== write to {fnm}");
    let b = x.as_bytes();
    let h = sha256::digest(b);
    std::fs::write(fnm, b)?;

    Ok(h)
}

#[allow(dead_code)]
pub fn write_trn_ass_01(tr_as: &Vec<PeaAssVar>, fnm: &str) -> Result<String, Box<dyn Error>> {
    let flds = [
        VarType::NewCarReg,
        VarType::Gpp,
        VarType::MaxPosPowSub,
        VarType::MaxNegPowSub,
        VarType::VsppMv,
        VarType::SppHv,
        VarType::BigLotMv,
        VarType::BigLotHv,
        VarType::SubPowCap,
        VarType::MaxPosPowFeeder,
        VarType::MaxNegPowFeeder,
        VarType::MaxPosDiffFeeder,
        VarType::MaxNegDiffFeeder,
        VarType::NoMeterTrans,
        VarType::SmallSellTr,
        VarType::ChgStnCapTr,
        VarType::ChgStnSellTr,
        VarType::PwCapTr,
        VarType::ZoneTr,
        VarType::PopTr,
        VarType::UnbalPowTr,
        VarType::PkPowTr,
        VarType::LargeSellTr,
        VarType::AllNoMeterTr,
        VarType::NoTr,
        VarType::HmChgEvTr,
        VarType::LvPowSatTr,
        VarType::CntLvPowSatTr,
        VarType::ChgStnCap,
        VarType::ChgStnSell,
        VarType::MvPowSatTr,
        VarType::SolarRoof,
        VarType::MvVspp,
        VarType::HvSpp,
        VarType::SmallSell,
        VarType::LargeSell,
        VarType::UnbalPow,
        VarType::CntUnbalPow,
        VarType::Uc1Val,
        VarType::Uc2Val,
        VarType::Uc3Val,
        VarType::Uc1Rank,
        VarType::Uc2Rank,
        VarType::Uc3Rank,
        VarType::NoHmChgEvTr,
        VarType::PowHmChgEvTr,
        VarType::PkSelPowPhsAKw,
        VarType::PkSelPowPhsBKw,
        VarType::PkSelPowPhsCKw,
        VarType::PkSelPowPhsAvg,
        VarType::PkSelPowPhsMax,
        VarType::UnbalPowRate,
        VarType::TransLossKw,
        VarType::UnbalPowLossKw,
    ];
    write_text_01(tr_as, &flds, fnm)
}

#[allow(dead_code)]
pub fn write_trn_ass_02(tr_as: &Vec<PeaAssVar>, fnm: &str) -> Result<String, Box<dyn Error>> {
    let flds = [
        VarType::NewCarReg,
        VarType::Gpp,
        VarType::MaxPosPowSub,
        VarType::MaxNegPowSub,
        VarType::VsppMv,
        VarType::SppHv,
        VarType::BigLotMv,
        VarType::BigLotHv,
        VarType::SubPowCap,
        VarType::MaxPosPowFeeder,
        VarType::MaxNegPowFeeder,
        VarType::MaxPosDiffFeeder,
        VarType::MaxNegDiffFeeder,
        VarType::NoMeterTrans,
        VarType::SmallSellTr,
        VarType::ChgStnCapTr,
        VarType::ChgStnSellTr,
        VarType::PwCapTr,
        VarType::ZoneTr,
        VarType::PopTr,
        VarType::UnbalPowTr,
        VarType::PkPowTr,
        VarType::LargeSellTr,
        VarType::AllNoMeterTr,
        VarType::NoTr,
        VarType::HmChgEvTr,
        VarType::LvPowSatTr,
        VarType::CntLvPowSatTr,
        VarType::ChgStnCap,
        VarType::ChgStnSell,
        VarType::MvPowSatTr,
        VarType::SolarRoof,
        VarType::MvVspp,
        VarType::HvSpp,
        VarType::SmallSell,
        VarType::LargeSell,
        VarType::UnbalPow,
        VarType::CntUnbalPow,
        VarType::Uc1Val,
        VarType::Uc2Val,
        VarType::Uc3Val,
        VarType::Uc1Rank,
        VarType::Uc2Rank,
        VarType::Uc3Rank,
        VarType::NoHmChgEvTr,
        VarType::PowHmChgEvTr,
        VarType::PkSelPowPhsAKw,
        VarType::PkSelPowPhsBKw,
        VarType::PkSelPowPhsCKw,
        VarType::PkSelPowPhsAvg,
        VarType::PkSelPowPhsMax,
        VarType::UnbalPowRate,
        VarType::TransLossKw,
        VarType::UnbalPowLossKw,
        VarType::TakeNote,
        VarType::CntTrUnbalLoss,
        VarType::CntTrSatLoss,
        VarType::SolarEnergy,
    ];
    write_text_02(tr_as, &flds, fnm)
}

pub fn write_text_02(
    tr_as: &Vec<PeaAssVar>,
    flds: &[VarType],
    fnm: &str,
) -> Result<String, Box<dyn Error>> {
    let mut x = String::new();
    use std::fmt::Write;
    for t in tr_as {
        //t._a("y:");
        write!(x, "{}", t.sbid)?;
        write!(x, "\t{}", t.pvid)?;
        write!(x, "\t{}", t.arid)?;
        for f in flds.iter() {
            let d = t.v[f.clone() as usize].v;
            write!(x, "\t{d}")?;
        }
        writeln!(x)?;
    }
    //println!("        ??? ===== write to {fnm}");
    let b = x.as_bytes();
    let h = sha256::digest(b);
    std::fs::write(fnm, b)?;
    Ok(h)
}

#[allow(dead_code)]
pub fn write_text_01(
    tr_as: &Vec<PeaAssVar>,
    flds: &[VarType],
    fnm: &str,
) -> Result<String, Box<dyn Error>> {
    let mut x = String::new();
    use std::fmt::Write;
    for t in tr_as {
        //t._a("y:");
        write!(x, "{}", t.sbid)?;
        write!(x, "\t{}", t.fdid)?;
        write!(x, "\t{}", t.aoj)?;
        write!(x, "\t{}", t.own)?;
        write!(x, "\t{}", t.peano)?;
        for f in flds.iter() {
            let d = t.v[f.clone() as usize].v;
            write!(x, "\t{d}")?;
        }
        writeln!(x)?;
    }
    println!("        ===== write to {fnm}");
    let b = x.as_bytes();
    let h = sha256::digest(b);
    std::fs::write(fnm, b)?;
    Ok(h)
}
