#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum PbcType {
    AllEdges,
    None
}

#[derive(Debug)]
pub struct SimContext {
    pub(crate) system_size: usize,
    pub(crate) pbc: PbcType,
    pub(crate) temperature: f64,
    pub(crate) n_sweeps: usize,
}

impl SimContext {
    pub fn new(system_size: usize, pbc: PbcType, temperature: f64, n_sweeps: usize) -> Self {

        SimContext { system_size, pbc, temperature, n_sweeps }
    } 
}
