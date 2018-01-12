use std::time::{Duration, Instant};


mod analysis;

pub use self::analysis::AnalysedMeasurements;



#[derive(Debug, Clone)]
pub struct Measurements {
    raw_data: Vec<u64>,
}

impl Measurements {
    pub fn analyse(self) -> AnalysedMeasurements {
        AnalysedMeasurements::analyse(self)
    }
}

pub fn run<S, T, I>(
    mut sorter: S,
    mut generate_instance: I,
    time_budget: Duration,
    max_iters: u64,
) -> Result<Measurements, ()>
where
    S: FnMut(&mut [T]),
    T: Ord,
    I: FnMut() -> Vec<T>,
{
    let mut time_budget_left = time_budget;
    let mut durations = Vec::new();

    for _ in 0..max_iters {
        let mut arr = generate_instance();

        // The actual measurement
        let before = Instant::now();
        sorter(&mut arr);
        let duration = before.elapsed();

        // Convert `Duration` to u64 (nanoseconds). We don't need the full
        // precision of `Duration`, as we don't expect anyone to run a
        // benchmark for longer than 584 years.
        let ns = duration.as_secs() * 1_000_000_000 + duration.subsec_nanos() as u64;
        durations.push(ns);

        if duration > time_budget_left {
            break;
        }
        time_budget_left -= duration;
    }

    Ok(Measurements { raw_data: durations })
}
