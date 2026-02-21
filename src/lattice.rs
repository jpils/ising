#![allow(dead_code)] // todo remave later

use ndarray::Array2;
use rand::RngExt;

#[derive(Debug, Clone, PartialEq)]
pub enum Spin {
    Up,
    Down
}

impl Spin {
    pub fn flip(&mut self) {
        *self = match self {
            Spin::Up => Self::Down,
            Self::Down => Spin::Up,
        }
    }

    fn get_rand_spin() -> Spin {
        let mut rng = rand::rng();

        if rng.random_bool(0.5) { 
            Spin::Up 
        } else { 
            Spin::Down 
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Lattice {
    lattice: Array2<Spin>
}

impl Lattice {
    pub fn new(system_size: usize) -> Self {
        let lattice_rand = (0..system_size.pow(2))
            .map(|_| Spin::get_rand_spin())
            .collect();

        Lattice { lattice: Array2::from_shape_vec((system_size, system_size), lattice_rand).unwrap() }
    }

    pub fn randomize(&mut self) {
        self.lattice
            .iter_mut()
            .for_each(|spin| *spin = Spin::get_rand_spin() );
    }
}
