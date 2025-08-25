use std::time::{Duration, Instant};
use logic::set::SpaceTimeIdSet;

pub fn measure_benchmark<F, R>(
    calculate: F,
    iterations: usize,
    subset_set_a: &SpaceTimeIdSet,
    subset_set_b: &SpaceTimeIdSet
) -> u128
where
    F: Fn(&SpaceTimeIdSet, &SpaceTimeIdSet) -> R,
{
    let mut duration_sum = Duration::ZERO;
    for _ in 0..iterations {
        let start = Instant::now();
        let _result = calculate(subset_set_a, subset_set_b);
        duration_sum += start.elapsed();
    }
    duration_sum.as_nanos() / iterations as u128
}
