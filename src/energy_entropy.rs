use num::Complex;

pub fn fft_magnitude(signal: &[f64]) -> Vec<f64> {
    let fft = rustfft::FftPlanner::new().plan_fft(signal.len(), rustfft::FftDirection::Forward);
    let mut spectrum: Vec<Complex<f64>> = signal.iter().map(|&x| Complex::new(x, 0.0)).collect();
    fft.process(&mut spectrum);

    spectrum.iter().map(|&c| c.norm_sqr()).collect::<Vec<_>>()
}

// TODO: add boundaries for allowing to calculate the energy of only certain frequencies
pub fn spectral_energy(signal: &[f64]) -> f64 {
    let magnitude_spectrum = fft_magnitude(signal);
    magnitude_spectrum.iter().map(|&x| x.powf(2.0)).sum::<f64>()
}

pub fn power_spectral_density(signal: &[f64]) -> Vec<f64> {
    let fft = fft_magnitude(signal);

    let count = fft.len() as f64;

    return fft.iter().map(|x| x.powf(2.0) / count).collect();
}

pub fn normalised_power_spectral_density(signal: &[f64]) -> Vec<f64> {
    let psd = power_spectral_density(signal);

    let total: f64 = psd.iter().sum();

    return psd.iter().map(|x| x / total).collect();
}

// https://stackoverflow.com/a/30465336
pub fn entropy(signal: &[f64]) -> f64 {
    let norm = normalised_power_spectral_density(signal);

    return -norm.iter().map(|x| x * x.ln()).sum::<f64>();
}
