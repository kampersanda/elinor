//!

use statrs::distribution::ContinuousCDF;
use statrs::distribution::StudentsT;

///
pub fn studentized_range(n_groups: usize, freedom: f64, alpha: f64) -> f64 {
    let t_dist = StudentsT::new(0.0, 1.0, freedom).unwrap();
    let q = t_dist.sf(1.0 - alpha / (2.0 * n_groups as f64));
    q * (2.0_f64).sqrt()
}
