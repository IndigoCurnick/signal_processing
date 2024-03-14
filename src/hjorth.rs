pub trait Hjorth<T> {
    fn hjorth_activity(&self) -> T;
    fn hjorth_mobility(&self) -> T;
    fn hjorth_complexity(&self) -> T;
}

macro_rules! impl_hjorth {
    ($float:ty) => {
        impl Hjorth<$float> for Vec<$float> {
            fn hjorth_activity(&self) -> $float {
                let n = self.len() as $float;
                let sum_squared = self.iter().map(|x| x.powf(2.0)).sum::<$float>();
                sum_squared / n
            }

            fn hjorth_mobility(&self) -> $float {
                let n = self.len() as $float;
                let derivative = self.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
                let sum_squared_derivative = derivative.iter().map(|x| x.powi(2)).sum::<$float>();
                let activity = self.hjorth_activity();
                (sum_squared_derivative / n).sqrt() / activity.sqrt()
            }

            fn hjorth_complexity(&self) -> $float {
                let n = self.len() as $float;
                let second_derivative = self
                    .windows(3)
                    .map(|w| w[2] - 2.0 * w[1] + w[0])
                    .collect::<Vec<_>>();
                let sum_squared_second_derivative = second_derivative
                    .iter()
                    .map(|x| x.powf(2.0))
                    .sum::<$float>();
                let mobility = self.hjorth_mobility();
                (sum_squared_second_derivative / n).sqrt() / mobility.sqrt()
            }
        }
    };
}

impl_hjorth!(f64);
impl_hjorth!(f32);
