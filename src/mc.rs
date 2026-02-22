#![allow(dead_code)]

use crate::{lattice::Lattice, system_context::SimContext};

pub(crate) fn run(context: SimContext, lattice: Lattice) {
    let mut frames:Vec<Lattice> = Vec::with_capacity(context.n_sweeps);

    for _ in 0..context.n_sweeps {
        let idx = lattice.get_rand_point();
    }
}

pub(crate) fn propagate(idx:(usize, usize), lattice: &mut Lattice) {

}
