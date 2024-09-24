//!

use statrs::distribution::ContinuousCDF;
use statrs::distribution::StudentsT;

///
pub fn studentized_range(n_groups: usize, freedom: f64, alpha: f64) -> f64 {
    let t_dist = StudentsT::new(0.0, 1.0, freedom).unwrap();
    let q = t_dist.sf(1.0 - alpha / (2.0 * n_groups as f64));
    q * (2.0_f64).sqrt()
}

///
pub fn studentized_range_p_value(n_groups: usize, freedom: f64, q: f64) -> f64 {
    let t_dist = StudentsT::new(0.0, 1.0, freedom).unwrap();

    let t = q / (2.0_f64).sqrt();
    let p = 2.0 * n_groups as f64 * t_dist.sf(t);

    p.min(1.0) // P値は1を超えないようにする
}
