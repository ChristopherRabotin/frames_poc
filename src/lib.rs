extern crate nalgebra;

/// Returns the provided angle bounded between 0.0 and 360.0
pub fn between_0_360(angle: f64) -> f64 {
    let mut bounded = angle;
    while bounded > 360.0 {
        bounded -= 360.0;
    }
    while bounded < 0.0 {
        bounded += 360.0;
    }
    bounded
}

/// Returns the provided angle bounded between -180.0 and +180.0
pub fn between_pm_180(angle: f64) -> f64 {
    let mut bounded = angle;
    while bounded > 180.0 {
        bounded -= 360.0;
    }
    while bounded < -180.0 {
        bounded += 360.0;
    }
    bounded
}

pub mod frames;
pub mod state;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
