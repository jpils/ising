#![allow(dead_code)] // todo remave later

use ndarray::Array2;
use rand::RngExt;

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Spin {
    Up = 1,
    Down = -1
}

impl Spin {
    pub fn flip(&mut self) {
        *self = match self {
            Spin::Up(_) => Self::Down,
            Self::Down(_) => Spin::Up,
        }
    }

    fn get_rand_spin() -> Spin {
        let mut rng = rand::rng();

        if rng.random_bool(0.5) { 
            Spin::Up()
        } else { 
            Spin::Down 
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Lattice {
    shape: (usize, usize),
    lattice: Array2<Spin>
}

impl Lattice {
    pub(crate) fn new(system_size: usize) -> Self {
        let lattice_rand = (0..system_size.pow(2))
            .map(|_| Spin::get_rand_spin())
            .collect();

        let shape = (system_size, system_size);
        let lattice = Array2::from_shape_vec(shape, lattice_rand).unwrap();

        Lattice { shape, lattice }
    }

    pub(crate) fn randomize(&mut self) {
        self.lattice
            .iter_mut()
            .for_each(|spin| *spin = Spin::get_rand_spin() );
    }

    pub(crate) fn get_rand_point(&self) -> (usize, usize) {
        let mut rng = rand::rng();
        (rng.random_range(self.shape.0..self.shape.1), rng.random_range(self.shape.0..self.shape.1))
    }

    pub(crate) fn to_u
}
