//! Intersection (`&`) logic for `SpaceTimeIdSet`.

use std::ops::BitAnd;

use crate::{id::relation::Containment, set::SpaceTimeIdSet};

/// Implements the `&` (intersection) operator for `SpaceTimeIdSet` and its references.
///
/// This returns a new set containing only the regions present in both operands.
///
/// # Example
/// ```
/// use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};
///
/// let stid_a = SpaceTimeId::new(4, Single(5), Single(3),
///                               Single(10), 60, Single(100)).unwrap();
/// let stid_b = SpaceTimeId::new(4, Single(5), Single(3),
///                               Single(10), 60, Single(100)).unwrap();
/// let set_a = SpaceTimeIdSet::from(stid_a);
/// let set_b = SpaceTimeIdSet::from(stid_b);
/// let a_and_b = &set_a & &set_b;
/// // a_and_b contains only the overlapping space-time areas
/// ```
macro_rules! impl_bitand_for_spacetimeidset {
    ($lhs:ty, $rhs:ty) => {
        impl BitAnd<$rhs> for $lhs {
            type Output = SpaceTimeIdSet;

            fn bitand(self, rhs: $rhs) -> Self::Output {
                let mut result = Vec::new();

                for self_stid in &self.inner {
                    for rhs_stid in &rhs.inner {
                        match self_stid.containment_relation(rhs_stid) {
                            Containment::Full => {
                                result.push(rhs_stid.clone());
                            }
                            Containment::Partial(overlap_stid) => {
                                result.push(overlap_stid.clone());
                            }
                            Containment::None => {}
                        }
                    }
                }

                SpaceTimeIdSet { inner: result }
            }
        }
    };
}

// Apply BitAnd to combinations of owned and reference types
impl_bitand_for_spacetimeidset!(SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(SpaceTimeIdSet, &mut SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&SpaceTimeIdSet, &mut SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&mut SpaceTimeIdSet, SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&mut SpaceTimeIdSet, &SpaceTimeIdSet);
impl_bitand_for_spacetimeidset!(&mut SpaceTimeIdSet, &mut SpaceTimeIdSet);
