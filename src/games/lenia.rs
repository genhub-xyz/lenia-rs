use fft2d::nalgebra::{fft_2d, fftshift, ifft_2d};
use nalgebra::{Complex, DMatrix};
use rand::{thread_rng, Rng};

use crate::{
    creatures::{orbium::Orbium, Creature},
    kernels::{lenia::Ring, Kernel},
};

use super::Game;

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Lenia {
    k: DMatrix<Complex<f64>>,
}

impl Game<64, 20> for Lenia {
    type Kernel = Ring;
    type Creature = Orbium;
    const SIZE: usize = 64;

    fn new() -> Self {
        let d = Self::Kernel::to_dmatrix();
        let sum = d.fold(0., |acc, x| x.re + acc);
        let k_norm = d.map(|x| Self::k(x.re)).map(|x| Complex::new(x / sum, 0.));
        let k_shifted = fftshift(&k_norm);
        let fk = fft_2d(k_shifted);
        Self { k: fk }
    }

    fn initial_state() -> DMatrix<Complex<f64>> {
        let mut rng = thread_rng();
        let mut initial_state = DMatrix::zeros(Self::SIZE, Self::SIZE);
        initial_state = initial_state.map(|_| {
            let v: f64 = rng.gen();
            Complex::new(v, 0.)
        });
        initial_state
    }

    fn update(&self, cells: DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
        let res = fft_2d(cells.clone());
        let a = self.k.clone() * res;
        let res2 = ifft_2d(a);
        cells.zip_map(&res2, |x, y| {
            let real_part = x.re + 1. / Self::Creature::T * Self::Creature::growth(y.re);
            Complex::new(real_part.clamp(0., 1.), 0.)
        })
    }
}
