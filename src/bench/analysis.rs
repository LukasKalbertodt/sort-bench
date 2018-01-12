use std::fmt;
use super::Measurements;

pub struct AnalysedMeasurements {
    sorted_measurements: Vec<u64>,
    avg: f64,
}

impl AnalysedMeasurements {
    pub fn analyse(meas: Measurements) -> Self {
        let mut sorted_measurements = meas.raw_data;
        sorted_measurements.sort();

        let avg = sorted_measurements.iter().cloned().sum::<u64>() as f64
            / sorted_measurements.len() as f64;

        Self { avg, sorted_measurements }
    }

    pub fn num_measurements(&self) -> usize {
        self.sorted_measurements.len()
    }

    pub fn avg(&self) -> f64 {
        self.avg
    }

    pub fn median(&self) -> u64 {
        self.sorted_measurements[self.num_measurements() / 2]
    }
}


impl fmt::Debug for AnalysedMeasurements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AnalysedMeasurements")
            .field("avg", &self.avg())
            .field("median", &self.median())
            .finish()
    }
}
