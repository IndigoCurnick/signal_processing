use vec_utilities::maths::stats::Statistics;

/// NOTE: Can produce unusual behaviour with empty lists
pub fn signal_magnitude_area(measurements: &[f64], times: &[f64]) -> Result<f64, ()> {
    if measurements.len() != times.len() {
        // Need to have as many measurements as times
        return Err(());
    }

    let mut sma = 0.0;
    for i in 1..measurements.len() {
        sma += (measurements[i - 1].abs() + measurements[i].abs()) * (times[i] - times[i - 1]);
    }

    let max_time = times.iter().float_max();
    let min_time = times.iter().float_min();

    let time = max_time - min_time;

    return Ok(sma / (2.0 * time));
}
