trait Filters<T> {
    fn low_pass_filter(&self, cutoff_frequency: T) -> Vec<T>;
    fn high_pass_filter(&self, cutoff_frequency: T) -> Vec<T>;
    fn band_pass_filters(&self, low_cutoff_frequency: T, high_cutoff_frequency: T) -> Vec<T>;
}

macro_rules! impl_filters {
    ($float:ty) => {
        impl Filters<$float> for Vec<$float> {
            fn low_pass_filter(&self, cutoff_frequency: $float) -> Vec<$float> {
                let mut filtered_signal = Vec::with_capacity(signal.len());

                let filter_coefficient = 2.0 * std::f64::consts::PI * cutoff_frequency;
                let mut prev_value = 0.0;

                for &value in signal {
                    let filtered_value = prev_value + filter_coefficient * (value - prev_value);
                    filtered_signal.push(filtered_value);
                    prev_value = filtered_value;
                }

                filtered_signal
            }
        }
    };
}
