use itertools::Itertools;
use nalgebra::{Complex, DMatrix};

pub mod lenia;

pub trait Kernel<const S: usize> {
    const CELLS: [[f64; S]; S];

    fn to_dmatrix() -> DMatrix<Complex<f64>> {
        let cells = Self::CELLS
            .flatten()
            .into_iter()
            .map(|x| Complex::new(*x, 0.))
            .collect_vec();
        DMatrix::from_vec(S, S, cells)
    }
}
