//! Union (`|`) logic for `SpaceTimeIdSet`.
//!
//! Performs a logical union (OR) between two space-time sets.

use std::ops::BitOr;

use crate::set::SpaceTimeIdSet;

/// Implements the `|` (union) operator for various combinations of `SpaceTimeIdSet` and references.
///
/// This returns a new set containing all space-time regions from both operands,
/// ensuring that overlapping regions are merged canonically.
///
/// # Example
/// ```
/// use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};
///
/// let stid1 = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3),
///                              DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
/// let stid2 = SpaceTimeId::new(4, DimensionRange::Single(6), DimensionRange::Single(4),
///                              DimensionRange::Single(11), 60, DimensionRange::Single(101)).unwrap();
/// let set1 = SpaceTimeIdSet::from(stid1);
/// let set2 = SpaceTimeIdSet::from(stid2);
/// let union = &set1 | &set2;
/// let union_ref = &set1 | &set2;
/// let mut set1_clone = set1.clone();
/// let union_mut = &mut set1_clone | &set2;
/// ```
macro_rules! impl_bitor_for_spacetimeidset {
    ($lhs:ty, $rhs:ty) => {
        impl BitOr<$rhs> for $lhs {
            type Output = SpaceTimeIdSet;

            fn bitor(self, rhs: $rhs) -> Self::Output {
                let mut new_set = self.clone();
                for stid in rhs.iter() {
                    new_set.insert(*stid);
                }
                new_set
            }
        }
    };
}

// Owned and reference combinations
impl_bitor_for_spacetimeidset!(SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitor_for_spacetimeidset!(SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitor_for_spacetimeidset!(SpaceTimeIdSet, &mut SpaceTimeIdSet);
impl_bitor_for_spacetimeidset!(&SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitor_for_spacetimeidset!(&SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitor_for_spacetimeidset!(&SpaceTimeIdSet, &mut SpaceTimeIdSet);
impl_bitor_for_spacetimeidset!(&mut SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitor_for_spacetimeidset!(&mut SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitor_for_spacetimeidset!(&mut SpaceTimeIdSet, &mut SpaceTimeIdSet);
