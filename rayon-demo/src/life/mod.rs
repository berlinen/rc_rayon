use crate::cpu_time::{self, CpuMesure};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::sync::Arc;

#[cfg(test)]
mod bench;

#[derive(serde::Deserialize)]
pub struct Args {
    cmd_bench: bool,
    cmd_play: bool,
    flag_size: Option<usize>,
    flag_gens: Option<usize>,
    flag_fps: Option<usize>,
    flag_skip_bridge: bool,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Board {
    board: Vec<bool>,
    survive: Arc<Vec<usize>>,
    born: Arc<Vec<usize>>,
    rows: usize,
    cols: usize,
}
