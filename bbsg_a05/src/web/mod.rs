// old pages
pub mod p02;
pub mod p03;
pub mod p04;
pub mod p05;
pub mod p06;
pub mod p07;
pub mod p08;
pub mod q02;
// sbb
pub mod sbb01;
pub mod sbb02;
pub mod sbb03;
pub mod sbb04;
pub mod sbb05;
pub mod sbb06;
pub mod sbb07;
pub mod sbb08;
pub mod sbb09;
pub mod sbb10;
pub mod sbb11;
pub mod sbb12;
pub mod sbb13;
// tra
pub mod tra01;
// fda
pub mod fda01;
pub mod fdw01;
pub mod fdw02;
//pub mod fda02;

pub const SBB_MENU: [(&str, &str, &str); 13] = [
    (
        "/sbb01",
        "1.SubCstFirYr",
        "แสดงรายละเอียดผลตอบแทนแต่ละปี เป็นรายสถานีไฟฟ้า เลือกดูข้อมูลที่ต้องการได้",
    ),
    (
        "/sbb02",
        "2.ProvCstFirYr",
        "แสดงรายละเอียดผลตอบแทนแต่ละปี เป็นรายจังหวัด เลือกดูข้อมูลที่ต้องการได้",
    ),
    (
        "/sbb03",
        "3.SubFirItem",
        "แสดงรายละเอียดผลตอบแทน เป็นรายสถานีไฟฟ้า",
    ),
    (
        "/sbb04",
        "4.ProvFirItem",
        "แสดงรายละเอียดผลตอบแทน เป็นรายจังหวัด",
    ),
    (
        "/sbb05",
        "5.SubCstItem",
        "แสดงรายละเอียดต้นทุนค่าใช้จ่ายในการดำเนินการ เป็นรายสถานีไฟฟ้า",
    ),
    (
        "/sbb06",
        "6.ProvCstItem",
        "แสดงรายละเอียดต้นทุนค่าใช้จ่ายในการดำเนินการ เป็นรายจังหวัด",
    ),
    (
        "/sbb07",
        "7.ProvCstFir25#1",
        "แสดงรายละเอียดผลตอบแทนแต่ละปี\n เป็นรายจังหวัด เลือกดูข้อมูลที่ต้องการได้ (25 จังหวัด #1)",
    ),
    (
        "/sbb08",
        "8.ProvFir25#1",
        "แสดงรายละเอียดผลตอบแทน เป็นรายจังหวัด (25 จังหวัด #1)",
    ),
    (
        "/sbb09",
        "9.ProvCst25#1",
        "แสดงรายละเอียดต้นทุนค่าใช้จ่ายในการดำเนินการ เป็นรายจังหวัด (25 จังหวัด #1)",
    ),
    (
        "/sbb10",
        "10.ProvCstFir25#2",
        "แสดงรายละเอียดผลตอบแทนแต่ละปี\n เป็นรายจังหวัด เลือกดูข้อมูลที่ต้องการได้ (25 จังหวัด #2)",
    ),
    (
        "/sbb11",
        "11.ProvFir25#2",
        "แสดงรายละเอียดผลตอบแทน เป็นรายจังหวัด (25 จังหวัด #2)",
    ),
    (
        "/sbb12",
        "12.ProvCst25#2",
        "แสดงรายละเอียดต้นทุนค่าใช้จ่ายในการดำเนินการ เป็นรายจังหวัด (25 จังหวัด #2)",
    ),
    ("/sbb13", "13.Para#1", "แสดงรายละเอียดพารามิเตอร์"),
];

use crate::dcl::VarType;

pub const SHOW_FLDS1: [VarType; 13] = [
    VarType::NoTr,
    VarType::NoPeaTr,
    VarType::NoCusTr,
    VarType::NoMet1Ph,
    VarType::NoMet3Ph,
    VarType::BessMWh,
    VarType::NoBess,
    VarType::NoDevice,
    VarType::NoHmChgEvTr,
    VarType::SubPowCap,
    VarType::MaxPosPowSub,
    VarType::PowTrSat,
    VarType::EnGrowth,
];
