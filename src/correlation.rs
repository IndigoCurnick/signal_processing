use num::Float;
use vec_utilities::maths::stats::Statistics;

pub fn correlation<T: Float>(ai: &[f64], aj: &[f64]) -> f64 {
    // TODO: Potential error handling
    let ai_mean = ai.iter().mean().unwrap();
    let aj_mean = aj.iter().mean().unwrap();

    let numerator = ai
        .iter()
        .zip(aj.iter())
        .map(|(&i, &j)| (i - ai_mean) * (j - aj_mean))
        .sum::<f64>();

    let denom: f64 = (ai.iter().map(|&x| (x - ai_mean).powf(2.0)).sum::<f64>()
        * aj.iter().map(|x| (x - aj_mean).powf(2.0)).sum::<f64>())
    .sqrt();

    return numerator / denom;
}
