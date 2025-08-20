use std::ops::{Add, Mul, Sub};

use crate::id::DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single};
use crate::id::{DimensionRange, SpaceTimeId};

/// Represents the containment relationship between two `SpaceTimeId` instances.
///
/// This enum describes how two space-time regions relate to each other in terms of
/// spatial and temporal overlap.
#[derive(Debug, PartialEq)]
pub enum Containment {
    /// Indicates that `self` fully contains the `other` ID.
    /// The entire region represented by `other` lies within `self`.
    Full,
    /// Indicates that `self` and `other` partially overlap.
    /// The associated `SpaceTimeId` represents the intersection area.
    Partial(SpaceTimeId),
    /// Indicates that there is no overlap between `self` and `other`.
    /// The two regions are completely disjoint.
    None,
}

impl SpaceTimeId {
    /// Determines the containment relationship between `self` and another `SpaceTimeId`.
    ///
    /// This method checks if `self` fully contains, partially overlaps with, or is disjoint from `other`.
    /// To compare two IDs with potentially different resolutions, it first scales both to a common,
    /// finer resolution before calculating the intersection.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the other `SpaceTimeId` to compare against.
    ///
    /// # Returns
    ///
    /// A `Containment` enum:
    /// - `Containment::Full` if `self`'s area completely covers `other`'s area.
    /// - `Containment::Partial(intersection_id)` if they overlap, where `intersection_id` is the common area.
    /// - `Containment::None` if they do not overlap at all.
    ///
    /// # Example
    ///
    /// ```rust
    /// use logic::id::{DimensionRange::Single, SpaceTimeId};
    ///
    /// let stid1 = SpaceTimeId::new(4, Single(5), Single(3), Single(10), 60, Single(100)).unwrap();
    /// let stid2 = SpaceTimeId::new(4, Single(5), Single(3), Single(10), 60, Single(100)).unwrap();
    /// let containment = stid1.containment_relation(&stid2);
    /// ```
    pub fn relation(&self, other: &SpaceTimeId) -> Containment {
        let target_z;
        let target_i;

        if self.i == 0 && other.i == 0 {
            //空間IDと空間ID
            target_z = self.z.max(other.z);
            target_i = self.i.min(other.i);
        } else if self.i != 0 && other.i != 0 {
            //時空間IDと時空間ID
            //現状のアルゴリズムで問題なく動作
            target_z = self.z.max(other.z);
            target_i = self.i.min(other.i);
        } else {
            //時空間IDと空間ID
            //空間ID側を時空間IDに変換
            target_z = self.z.max(other.z);
            target_i = self.i.max(other.i);
        }

        let self_scaled = self
            .change_scale(Some(target_z), Some(target_i))
            .expect("Failed to scale self");
        let other_scaled = other
            .change_scale(Some(target_z), Some(target_i))
            .expect("Failed to scale other");

        let x = match Self::same_level_dimension_intersection(self_scaled.x, other_scaled.x) {
            Some(v) => v,
            None => {
                return Containment::None;
            }
        };

        let y = match Self::same_level_dimension_intersection(self_scaled.y, other_scaled.y) {
            Some(v) => v,
            None => {
                return Containment::None;
            }
        };

        let f = match Self::same_level_dimension_intersection(self_scaled.f, other_scaled.f) {
            Some(v) => v,
            None => {
                return Containment::None;
            }
        };

        let t = match Self::same_level_dimension_intersection(self_scaled.t, other_scaled.t) {
            Some(v) => v,
            None => {
                return Containment::None;
            }
        };

        if other_scaled.x == x && other_scaled.y == y && other_scaled.f == f && other_scaled.t == t
        {
            return Containment::Full;
        } else {
            return Containment::Partial(SpaceTimeId::new(target_z, f, x, y, target_i, t).unwrap());
        }
    }

    fn same_level_dimension_intersection<T>(
        a: DimensionRange<T>,
        b: DimensionRange<T>,
    ) -> Option<DimensionRange<T>>
    where
        T: Copy
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + From<u8>
            + PartialEq
            + std::cmp::PartialOrd
            + Ord,
    {
        match a {
            Single(a_v) => match b {
                Single(b_v) => {
                    if a_v == b_v {
                        Some(a)
                    } else {
                        None
                    }
                }
                LimitRange(b_v_s, b_v_e) => {
                    if b_v_s <= a_v && a_v <= b_v_e {
                        Some(a)
                    } else {
                        None
                    }
                }
                AfterUnLimitRange(b_v_s) => {
                    if b_v_s <= a_v {
                        Some(a)
                    } else {
                        None
                    }
                }

                BeforeUnLimitRange(b_v_e) => {
                    if a_v <= b_v_e {
                        Some(a)
                    } else {
                        None
                    }
                }
                Any => Some(a),
            },
            LimitRange(a_v_s, a_v_e) => match b {
                LimitRange(b_v_s, b_v_e) => {
                    let start = std::cmp::max(a_v_s, b_v_s);
                    let end = std::cmp::min(a_v_e, b_v_e);

                    if start <= end {
                        Some(LimitRange(start, end))
                    } else {
                        None
                    }
                }
                AfterUnLimitRange(b_v_s) => {
                    if b_v_s <= a_v_e {
                        let start = std::cmp::max(a_v_s, b_v_s);
                        let end = a_v_e;
                        Some(LimitRange(start, end))
                    } else {
                        None
                    }
                }
                BeforeUnLimitRange(b_v_e) => {
                    if b_v_e >= a_v_s {
                        let start = a_v_s;
                        let end = std::cmp::min(a_v_e, b_v_e);
                        if start <= end {
                            Some(LimitRange(start, end))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Any => Some(a),
                _ => Self::same_level_dimension_intersection(b, a),
            },
            AfterUnLimitRange(a_v_s) => match b {
                AfterUnLimitRange(b_v_s) => Some(AfterUnLimitRange(std::cmp::max(a_v_s, b_v_s))),
                BeforeUnLimitRange(b_v_e) => {
                    if a_v_s <= b_v_e {
                        Some(LimitRange(a_v_s, b_v_e))
                    } else {
                        None
                    }
                }
                Any => Some(a),
                _ => Self::same_level_dimension_intersection(b, a),
            },

            BeforeUnLimitRange(a_v_e) => match b {
                BeforeUnLimitRange(b_v_e) => {
                    let end = std::cmp::min(a_v_e, b_v_e);
                    Some(BeforeUnLimitRange(end))
                }
                Any => Some(a),
                _ => Self::same_level_dimension_intersection(b, a),
            },
            Any => match b {
                Any => Some(a),
                _ => Self::same_level_dimension_intersection(b, a),
            },
        }
    }
}
