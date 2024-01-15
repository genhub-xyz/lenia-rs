use nalgebra::{Complex, DMatrix};

use crate::{creatures::Creature, kernels::Kernel};

pub mod lenia;

pub trait Game<const K: usize, const C: usize> {
    type Kernel: Kernel<K>;
    type Creature: Creature<C>;
    const SIZE: usize;

    fn new() -> Self;
    fn initial_state() -> DMatrix<Complex<f64>>;
    fn update(&self, cells: DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>>;
    fn k(x: f64) -> f64 {
        f64::from(x < 1.) * Self::Creature::bell(x, 0.5, 0.15)
    }
}
