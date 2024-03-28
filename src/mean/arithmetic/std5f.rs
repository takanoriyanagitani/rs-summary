pub fn mean_arithmetic5f_std_fast(f5: &[f32]) -> f32 {
    let cnt: usize = f5.len();
    let sum: f32 = f5.iter().sum();
    match cnt {
        0 => f32::NAN,
        _ => sum / (cnt as f32),
    }
}

pub fn mean_arithmetic5f_std_high_precision(f5: &[f32]) -> f64 {
    let cnt: usize = f5.len();
    let mapd = f5.iter().map(|f: &f32| f64::from(*f));
    let sum: f64 = mapd.sum();
    match cnt {
        0 => f64::NAN,
        _ => sum / (cnt as f64),
    }
}
