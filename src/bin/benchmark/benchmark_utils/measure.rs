use std::time::Instant;
use logic::set::SpaceTimeIdSet;

pub fn measure_benchmark<F, R>(
    calculate: F,
    subset_set_a: &SpaceTimeIdSet,
    subset_set_b: &SpaceTimeIdSet
) -> u128
where
    F: Fn(&SpaceTimeIdSet, &SpaceTimeIdSet) -> R,
{
    let start = Instant::now();
    let _result = calculate(subset_set_a, subset_set_b);
    start.elapsed().as_nanos()
}
