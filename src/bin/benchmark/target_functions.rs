use logic::{set::SpaceTimeIdSet};

/// Target functions for benchmarking set operations
/// 'benchmark' measures the performance of each function below
pub fn and(set1: &SpaceTimeIdSet, set2: &SpaceTimeIdSet) -> SpaceTimeIdSet {
  set1 & set2
}
pub fn or(set1: &SpaceTimeIdSet, set2: &SpaceTimeIdSet) -> SpaceTimeIdSet {
  set1 | set2
}
pub fn xor(set1: &SpaceTimeIdSet, set2: &SpaceTimeIdSet) -> SpaceTimeIdSet {
  set1 ^ set2
}
pub fn eq(set1: &SpaceTimeIdSet, set2: &SpaceTimeIdSet) -> bool {
  set1 == set2
}


