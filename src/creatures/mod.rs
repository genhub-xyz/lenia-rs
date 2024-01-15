pub mod orbium;

pub trait Creature<const S: usize> {
    const R: u32;
    const T: f64;
    const M: f64;
    const S: f64;
    const CELLS: [[f64; S]; S];

    fn growth(y: f64) -> f64 {
        let m = Self::M;
        let s = Self::S;
        Self::bell(y, m, s) * 2. - 1.
    }

    fn bell(x: f64, m: f64, s: f64) -> f64 {
        f64::exp(-((x - m) / s).powf(2.) / 2.)
    }
}
