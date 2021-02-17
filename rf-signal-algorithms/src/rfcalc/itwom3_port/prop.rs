#[derive(Default)]
pub(crate) struct PropType {
    pub(crate) aref: f64,
    pub(crate) dist: f64,
    pub(crate) hg: [f64; 2],
    pub(crate) rch: [f64; 2],
    pub(crate) wn: f64,
    pub(crate) dh: f64,
    pub(crate) dhd: f64,
    pub(crate) ens: f64,
    pub(crate) encc: f64,
    pub(crate) cch: f64,
    pub(crate) cd: f64,
    pub(crate) gme: f64,
    pub(crate) zgndreal: f64,
    pub(crate) zgndimag: f64,
    pub(crate) he: [f64; 2],
    pub(crate) dl: [f64; 2],
    pub(crate) the: [f64; 2],
    pub(crate) tiw: f64,
    pub(crate) ght: f64,
    pub(crate) ghr: f64,
    pub(crate) rph: f64,
    pub(crate) hht: f64,
    pub(crate) hhr: f64,
    pub(crate) tgh: f64,
    pub(crate) tsgh: f64,
    pub(crate) thera: f64,
    pub(crate) thenr: f64,
    pub(crate) rpl: i32,
    pub(crate) kwx: i32,
    pub(crate) mdp: i32,
    pub(crate) ptx: i32,
    pub(crate) los: i32,
}

#[derive(Default)]
pub(crate) struct PropVType {
    pub(crate) sgc: f64,
    pub(crate) lvar: i32,
    pub(crate) mdvar: i32,
    pub(crate) klim: i32,
}

#[derive(Default)]
pub(crate) struct PropAType {
    pub(crate) dlsa: f64,
    pub(crate) dx: f64,
    pub(crate) ael: f64,
    pub(crate) ak1: f64,
    pub(crate) ak2: f64,
    pub(crate) aed: f64,
    pub(crate) emd: f64,
    pub(crate) aes: f64,
    pub(crate) ems: f64,
    pub(crate) dls: [f64; 2],
    pub(crate) dla: f64,
    pub(crate) tha: f64,
}
