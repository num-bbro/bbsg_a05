use crate::dcl::ProcEngine;
use crate::dcl::EV_PRV_ADJ_2;
use crate::dcl::*;
use crate::p01::ev_distr;
use crate::p03::p03_load_lp;
use crate::p04::SubFeedTrans;
use crate::p08::ld_sub_info;
use crate::utl::*;
use sglib04::ld1::p13_am_po_de;
use sglib04::ld1::p13_aoj;
use sglib04::ld1::p13_cnl_mt;
use sglib04::ld1::p13_cnl_trs;
use sglib04::ld1::p13_ev_distr;
use sglib04::ld1::p13_fd_rep_lp;
use sglib04::ld1::p13_lv_solar;
use sglib04::ld1::p13_mt2bil;
use sglib04::ld1::p13_mt_bil;
use sglib04::ld1::p13_mu_po_de;
use sglib04::ld1::p13_re_plan;
use sglib04::ld1::p13_sb_in_re;
use sglib04::ld1::p13_sb_in_spp;
use sglib04::ld1::p13_sb_in_vspp;
use sglib04::ld1::p13_sb_rep_lp;
use sglib04::ld1::p13_spp;
use sglib04::ld1::p13_tr_in_amp;
use sglib04::ld1::p13_tr_in_aoj;
use sglib04::ld1::p13_tr_in_mun;
use sglib04::ld1::p13_tr_in_sol;
use sglib04::ld1::p13_tr_in_vol;
use sglib04::ld1::p13_tr_in_zn;
use sglib04::ld1::p13_volta;
use sglib04::ld1::p13_vspp;
use sglib04::ld1::p13_zone;
use sglib04::ld1::EV_PRV_ADJ_1;
use strum::IntoEnumIterator;

impl VarType {
    pub fn tousz(&self) -> usize {
        self.clone() as usize
    }
}

impl PeaAssVar {
    pub fn from(n1d: u64) -> Self {
        let mut v = Vec::<AssVar>::new();
        let mut vy = Vec::<Vec<f32>>::new();
        for vt in VarType::iter() {
            let st = match vt {
                VarType::MaxPosPowSub => SumType::Max,
                VarType::MaxNegPowSub => SumType::Max,
                VarType::MaxPosPowFeeder => SumType::Max,
                VarType::MaxNegPowFeeder => SumType::Max,
                VarType::MaxPosDiffFeeder => SumType::Max,
                VarType::MaxNegDiffFeeder => SumType::Max,
                VarType::UnbalPowRate => SumType::Max,
                _ => SumType::Sum,
            };
            v.push(AssVar::new(vt, st));
            let vv = Vec::<f32>::new();
            vy.push(vv);
        }
        PeaAssVar {
            n1d,
            v,
            vy,
            ..Default::default()
        }
    }
    pub fn div(&mut self, o: f32) {
        if o == 0f32 {
            return;
        }
        for v in self.v.iter_mut() {
            v.v /= o;
        }
    }
    pub fn nor(&mut self, o: &PeaAssVar) {
        for (v, o) in self.v.iter_mut().zip(o.v.iter()) {
            v.v /= z2o(o.v);
        }
    }
    pub fn copy(&mut self, o: &PeaAssVar, t: VarType) {
        self.v[t.clone() as usize].v = o.v[t.clone() as usize].v;
    }
    pub fn add(&mut self, o: &PeaAssVar) {
        for (v, o) in self.v.iter_mut().zip(o.v.iter()) {
            match v.s {
                SumType::Sum => v.v += o.v,
                SumType::Max => v.v = v.v.max(o.v),
                SumType::Min => v.v = v.v.min(o.v),
            }
        }
        for (vy, oy) in self.vy.iter_mut().zip(o.vy.iter()) {
            if vy.is_empty() && oy.len() > vy.len() {
                let mut oya = oy.clone();
                vy.append(&mut oya);
            } else if vy.len() == oy.len() {
                for (vv, ov) in vy.iter_mut().zip(oy.iter()) {
                    *vv += *ov;
                }
            }
        }
    }
    pub fn add1(&mut self, o: &PeaAssVar) -> String {
        for (v, o) in self.v.iter_mut().zip(o.v.iter()) {
            match v.s {
                SumType::Sum => v.v += o.v,
                SumType::Max => v.v = v.v.max(o.v),
                SumType::Min => v.v = v.v.min(o.v),
            }
        }
        for (vy, oy) in self.vy.iter_mut().zip(o.vy.iter()) {
            if vy.is_empty() && oy.len() > vy.len() {
                let mut oya = oy.clone();
                vy.append(&mut oya);
            } else if vy.len() == oy.len() {
                for (vv, ov) in vy.iter_mut().zip(oy.iter()) {
                    *vv += *ov;
                }
            }
        }
        String::new()
    }
    pub fn max(&mut self, o: &PeaAssVar) {
        if self.set == 0 {
            self.set += 1;
            for (v, o) in self.v.iter_mut().zip(o.v.iter()) {
                v.v = o.v;
            }
            return;
        }
        self.set += 1;
        for (v, o) in self.v.iter_mut().zip(o.v.iter()) {
            v.v = v.v.max(o.v);
        }
    }

    pub fn min(&mut self, o: &PeaAssVar) {
        if self.set == 0 {
            self.set += 1;
            for (v, o) in self.v.iter_mut().zip(o.v.iter()) {
                v.v = o.v;
            }
        }
        self.set += 1;
        for (v, o) in self.v.iter_mut().zip(o.v.iter()) {
            v.v = v.v.max(o.v);
        }
    }
    pub fn weigh(&mut self, o: &PeaAssVar) {
        for (v, o) in self.v.iter_mut().zip(o.v.iter()) {
            v.v *= o.v;
        }
    }
    pub fn sum(&mut self) {
        self.res = self.v.iter().map(|v| v.v).sum();
    }
}

impl PeaTrans {
    pub fn from_cmt(&mut self, cmt: &sglib04::geo1::CnlData) {
        self.tr_tag = cmt.tr_tag.clone();
        self.tr_fid = cmt.tr_fid.clone();
        self.tr_lt = cmt.tr_lt;
        self.tr_ln = cmt.tr_ln;
        self.tr_cd = cmt.tr_cd;
        self.tr_aoj = cmt.tr_aoj.clone();
        self.tr_pea = cmt.tr_pea.clone();
        self.tr_kva = cmt.tr_kva;
        self.tr_own = cmt.tr_own.clone();
        self.tr_loc = cmt.tr_loc.clone();
        self.tr_n1d = cmt.tr_n1d;
    }
}

impl PeaMeter {
    pub fn from_cmt(&mut self, cmt: &sglib04::geo1::CnlData) {
        self.mt_ins = cmt.mt_ins.clone();
        self.mt_pea = cmt.mt_pea.clone();
        self.mt_tag = cmt.mt_tag.clone();
        self.mt_phs = cmt.mt_phs.clone();
        self.mt_x = cmt.mt_x;
        self.mt_y = cmt.mt_y;
        self.mt_lt = cmt.mt_lt;
        self.mt_ln = cmt.mt_ln;
        self.mt_aoj = cmt.mt_aoj.clone();
        self.tr_tag = cmt.tr_tag.clone();
        self.tr_fid = cmt.tr_fid.clone();
        self.tr_lt = cmt.tr_lt;
        self.tr_ln = cmt.tr_ln;
        self.tr_cd = cmt.tr_cd;
        self.tr_aoj = cmt.tr_aoj.clone();
        self.tr_pea = cmt.tr_pea.clone();
        self.tr_kva = cmt.tr_kva;
        self.tr_own = cmt.tr_own.clone();
        self.tr_loc = cmt.tr_loc.clone();
        self.tr_n1d = cmt.tr_n1d;
        self.mt_n1d = cmt.mt_n1d;
        self.ar = cmt.ar.clone();
        self.ly = cmt.ly.clone();
        self.ix = cmt.ix;
    }
    pub fn from_bil(&mut self, bil: &sglib04::geo1::MeterBill) {
        self.trsg = bil.trsg.clone();
        self.pea = bil.pea.clone();
        self.ca = bil.ca.clone();
        self.inst = bil.inst.clone();
        self.rate = bil.rate.clone();
        self.volt = bil.volt.clone();
        self.mru = bil.mru.clone();
        self.mat = bil.mat.clone();
        self.main = bil.main.clone();
        self.kwh15 = bil.kwh15;
        self.kwh18 = bil.kwh18;
        self.amt19 = bil.amt19;
        self.ar = bil.ar.clone();
        self.idx = bil.idx;
        self.meth = bil.meth;
    }
}

impl AssVar {
    pub fn val(v: f32) -> AssVar {
        AssVar {
            t: VarType::None,
            s: SumType::Sum,
            v,
            ..Default::default()
        }
    }
    pub fn new(t: VarType, s: SumType) -> AssVar {
        AssVar {
            t,
            s,
            ..Default::default()
        }
    }
}

impl Pan for f32 {
    fn san(v: &str) -> String {
        v.as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    }
    fn pan0(&self) -> String {
        let v = format!("{self:.2}");
        let f = v[..v.len() - 3].to_string();
        let v = Self::san(&f);
        v.to_string()
    }
    fn pan2(&self) -> String {
        let v = format!("{self:.2}");
        let n = v[v.len() - 3..].to_string();
        let f = v[..v.len() - 3].to_string();
        let v = Self::san(&f);
        format!("{v}{n}")
    }
    fn pan3(&self) -> String {
        let v = format!("{self:.3}");
        let n = v[v.len() - 4..].to_string();
        let f = v[..v.len() - 4].to_string();
        let v = Self::san(&f);
        format!("{v}{n}")
    }
    fn pan(&self, i: i32) -> String {
        let v = match i {
            4 => format!("{self:.4}"),
            3 => format!("{self:.3}"),
            2 => format!("{self:.2}"),
            1 => format!("{self:.1}"),
            _ => format!("{self:.0}"),
        };
        if i > 0 && i <= 4 {
            let n = v[v.len() - (i as usize + 1)..].to_string();
            let f = v[..v.len() - (i as usize + 1)].to_string();
            let v = Self::san(&f);
            format!("{v}{n}")
        } else {
            Self::san(&v)
        }
    }
}

impl SubAssObj2 {
    pub fn sum(&mut self) {
        self.sum = self.ev1
            + self.ev2
            + self.ev3
            + self.ev4
            + self.ev5
            + self.re1
            + self.re2
            + self.re3
            + self.en1
            + self.en2
            + self.en3
            + self.en4;
    }
}

impl ProcEngine {
    fn subs(&mut self, ar: &str) {
        let fnm = format!("/mnt/e/CHMBACK/pea-data/data2/p11_{ar}_sb_fd_tr.bin");
        let bytes = std::fs::read(fnm).unwrap();
        let (subs, _): (Vec<SubFeedTrans>, usize) =
            bincode::decode_from_slice(&bytes[..], bincode::config::standard()).unwrap();
        self.subs = subs;
    }
    fn ctrs(&mut self, ar: &str) {
        self.ctrs = p13_cnl_trs(ar).unwrap();
    }
    fn cmts(&mut self, ar: &str) {
        self.cmts = p13_cnl_mt(ar).unwrap();
    }
    fn bils(&mut self, ar: &str) {
        self.bils = p13_mt_bil(ar).unwrap();
    }
    fn m2bs(&mut self, ar: &str) {
        self.m2bs = p13_mt2bil(ar).unwrap();
    }
    fn vols(&mut self, ar: &str) {
        self.vols = p13_volta(ar).unwrap();
    }
    fn votr(&mut self, ar: &str) {
        self.votr = p13_tr_in_vol(ar).unwrap();
    }
    fn spps(&mut self, ar: &str) {
        self.spps = p13_spp(ar).unwrap();
    }
    fn spsb(&mut self, ar: &str) {
        self.spsb = p13_sb_in_spp(ar).unwrap();
    }
    fn vsps(&mut self, ar: &str) {
        self.vsps = p13_vspp(ar).unwrap();
    }
    fn vssb(&mut self, ar: &str) {
        self.vssb = p13_sb_in_vspp(ar).unwrap();
    }
    fn zons(&mut self, ar: &str) {
        self.zons = p13_zone(ar).unwrap();
    }
    fn zntr(&mut self, ar: &str) {
        self.zntr = p13_tr_in_zn(ar).unwrap();
    }
    fn aojs(&mut self, ar: &str) {
        self.aojs = p13_aoj(ar).unwrap();
    }
    fn aotr(&mut self, ar: &str) {
        self.aotr = p13_tr_in_aoj(ar).unwrap();
    }
    fn amps(&mut self, ar: &str) {
        self.amps = p13_am_po_de(ar).unwrap();
    }
    fn amtr(&mut self, ar: &str) {
        self.amtr = p13_tr_in_amp(ar).unwrap();
    }
    fn muni(&mut self, ar: &str) {
        self.muni = p13_mu_po_de(ar).unwrap();
    }
    fn mutr(&mut self, ar: &str) {
        self.mutr = p13_tr_in_mun(ar).unwrap();
    }
    fn repl(&mut self, ar: &str) {
        self.repl = p13_re_plan(ar).unwrap();
    }
    fn resb(&mut self, ar: &str) {
        self.resb = p13_sb_in_re(ar).unwrap();
    }
    fn sola(&mut self, ar: &str) {
        if let Ok(a) = p13_lv_solar(ar) {
            self.sola = a;
        }
    }
    fn sotr(&mut self, ar: &str) {
        if let Ok(a) = p13_tr_in_sol(ar) {
            self.sotr = a;
        }
    }
    fn sblp(&mut self, ar: &str) {
        self.sblp = p13_sb_rep_lp(ar).unwrap();
    }
    fn fdlp(&mut self, ar: &str) {
        self.fdlp = p13_fd_rep_lp(ar).unwrap();
    }
    /*
    fn carg(&mut self) {
        self.carg = load_pvcamp();
    }
    */
    pub fn sb2pv(&self, sb: &String) -> String {
        if let Some(sf) = self.sbif.get(sb) {
            return sf.prov.to_string();
        }
        "".to_string()
    }
    pub fn prep0(ar: &str) -> Self {
        let mut eg = ProcEngine::default();
        eg.subs(ar);
        eg.ctrs(ar);
        eg.cmts(ar);
        eg.bils(ar);
        eg.m2bs(ar);
        eg.vols(ar);
        eg.votr(ar);
        eg.spps(ar);
        eg.spsb(ar);
        eg.vsps(ar);
        eg.vssb(ar);
        eg.zons(ar);
        eg.zntr(ar);
        eg.aojs(ar);
        eg.aotr(ar);
        eg.amps(ar);
        eg.amtr(ar);
        eg.muni(ar);
        eg.mutr(ar);
        eg.repl(ar);
        eg.resb(ar);
        eg.sola(ar);
        eg.sotr(ar);
        eg.sblp(ar);
        eg.fdlp(ar);
        eg
    }
    pub fn prep1() -> Self {
        ProcEngine {
            evpv: p13_ev_distr(&EV_PRV_ADJ_1),
            sbif: ld_sub_info().clone(),
            lp23: p03_load_lp("2023"),
            lp24: p03_load_lp("2024"),
            ..Default::default()
        }
    }
    pub fn prep2(ar: &str) -> Self {
        let mut eg = ProcEngine::default();
        eg.subs(ar);
        eg.ctrs(ar);
        eg.cmts(ar);
        eg.bils(ar);
        eg.m2bs(ar);
        eg.vols(ar);
        eg.votr(ar);
        eg.spps(ar);
        eg.spsb(ar);
        eg.vsps(ar);
        eg.vssb(ar);
        eg.zons(ar);
        eg.zntr(ar);
        eg.aojs(ar);
        eg.aotr(ar);
        eg.amps(ar);
        eg.amtr(ar);
        eg.muni(ar);
        eg.mutr(ar);
        eg.repl(ar);
        eg.resb(ar);
        eg.sola(ar);
        eg.sotr(ar);
        eg.sblp(ar);
        eg
    }
    pub fn prep_c01_0() -> Self {
        ProcEngine {
            evpv: p13_ev_distr(&EV_PRV_ADJ_1),
            sbif: ld_sub_info().clone(),
            ..Default::default()
        }
    }
    pub fn prep_c01_1(ar: &str) -> Self {
        let mut eg = ProcEngine::default();
        eg.subs(ar);
        eg.ctrs(ar);
        eg.cmts(ar);
        eg.bils(ar);
        eg.m2bs(ar);
        eg.vols(ar);
        eg.votr(ar);
        eg.spps(ar);
        eg.spsb(ar);
        eg.vsps(ar);
        eg.vssb(ar);
        eg.zons(ar);
        eg.zntr(ar);
        eg.aojs(ar);
        eg.aotr(ar);
        eg.amps(ar);
        eg.amtr(ar);
        eg.muni(ar);
        eg.mutr(ar);
        eg.repl(ar);
        eg.resb(ar);
        eg.sola(ar);
        eg.sotr(ar);
        eg.sblp(ar);
        eg
    }
    pub fn prep3(ar: &str) -> Self {
        let mut eg = ProcEngine::default();
        eg.subs(ar);
        eg.vols(ar);
        eg.spps(ar);
        eg.vsps(ar);
        eg.zons(ar);
        eg.aojs(ar);
        eg.amps(ar);
        eg.muni(ar);
        eg.repl(ar);
        eg.sola(ar);
        eg
    }
    pub fn prep5() -> Self {
        ProcEngine {
            evpv: ev_distr(&EV_PRV_ADJ_2),
            sbif: ld_sub_info().clone(),
            lp23: p03_load_lp("2023"),
            lp24: p03_load_lp("2024"),
            ..Default::default()
        }
    }
}
