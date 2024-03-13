use vec_utilities::maths::stats::Statistics;

pub fn differential_signal_vector_magnitude(
    measurements: &[f64],
    times: &[f64],
) -> Result<f64, ()> {
    if measurements.len() != times.len() {
        return Err(());
    }

    let mut dsvm = 0.0;

    for i in 1..measurements.len() - 1 {
        dsvm = ((measurements[i] - measurements[i - 1]).abs()
            + (measurements[i + 1] - measurements[i]).abs())
            * (times[i] - times[i - 1]);
    }

    let max_time = times.iter().float_max();
    let min_time = times.iter().float_min();

    let time = max_time - min_time;

    return Ok(dsvm / (2.0 * time));
}
