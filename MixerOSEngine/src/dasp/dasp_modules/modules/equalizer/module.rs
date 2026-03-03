use super::eq::{EqBand, BandType};

pub struct ParametricEq {
    bands: Vec<EqBand>,
    pub enabled: bool,
}

