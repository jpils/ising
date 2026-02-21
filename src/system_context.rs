#![allow(dead_code)] // todo remove
#[derive(Debug, PartialEq)]
pub enum PbcType {
    AllEdges,
    None
}

#[derive(Debug, PartialEq)]
pub struct SimContext {
    system_size: usize,
    pbc: PbcType,
    temperature: f64,
    n_sweeps: u32,
}

impl SimContext {
    pub fn new(system_size: usize, pbc: PbcType, temperature: f64, n_sweeps: u32) -> Self {

        SimContext { system_size, pbc, temperature, n_sweeps }
    } 
}
