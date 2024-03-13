fn signal_vector_magnitude(measurements: &[f64]) -> Option<f64> {
    let count = measurements.len();

    if count == 0 {
        return None;
    }

    return Some(
        (1.0 / count as f64) * measurements.iter().map(|x| x.powf(2.0).sqrt()).sum::<f64>(),
    );
}
