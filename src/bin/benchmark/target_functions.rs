use logic::{set::SpaceTimeIdSet};


  pub fn and(set1: &SpaceTimeIdSet, set2: &SpaceTimeIdSet) -> SpaceTimeIdSet {
  set1 & set2
}
pub fn or(set1: &SpaceTimeIdSet, set2: &SpaceTimeIdSet) -> SpaceTimeIdSet {
  set1 | set2
}
pub fn not(set1: &SpaceTimeIdSet, _set2: &SpaceTimeIdSet) -> SpaceTimeIdSet {
  !set1
}

