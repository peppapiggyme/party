use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/**
 * Get the name of the type of a var
 */
pub fn get_name<F>(_: &F) -> &'static str {
    std::any::type_name::<F>()
}

/**
 * Return forward-backward asymmetry
 */
pub fn asym(f: f64, b: f64) -> f64 {
    (f - b) / (f + b)
}

/**
 * Calculate eta given zenith angle `theta`
 */
pub fn eta(t: f64) -> f64 {
    -(t / 2.0).tan().ln()
}

/**
 * write a 1xN vector to a file, given the path
 */
pub fn write_to_file<P: AsRef<Path>>(data: Vec<f64>, path: P) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    for e in data {
        f.write_all(format!("{}\n", e).as_bytes())?;
    }
    Ok(())
}

/**
 * A fake parton distribution function set.
 * @note it does not depend on Q^2
 */
pub fn pdf(flavour: &str, x: f64, _q2: f64) -> f64 {
    match flavour {
        "u" => 2. * x.powf(1. / 9.) * (1. - x), // some function that has maximum at 0.1
        "d" => x.powf(1. / 9.) * (1. - x),      // some function that has maximum at 0.1
        "u~" => 1. - x * (1. - x).exp(),        // some monotonic decaying function..
        "d~" => 1. - x * (1. - x).exp(),        // same as u~
        _ => panic!("Flavour {} not supported", flavour),
    }
}
