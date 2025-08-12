use crate::id::{DimensionRange, SpaceTimeId};
pub mod and;
pub mod equal;
pub mod insert;
pub mod not;
pub mod or;
pub mod xor;

#[derive(Clone)]
/// This type represents a set of `SpaceTimeId` elements.
///
/// When a new `SpaceTimeId` is added, any overlapping ranges with existing elements
/// will be subtracted from the new one to ensure that the ranges represented by IDs
/// in the set remain disjoint (i.e., no overlap between any two entries).
///
/// As a result, each distinct region in the physical space-time domain is guaranteed
/// to be represented by *only one* `SpaceTimeId` within the set.
/// This ensures unambiguous mapping between a spatial-temporal region and its identifier.

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct SpaceTimeIdSet {
    inner: Vec<SpaceTimeId>,
}

impl SpaceTimeIdSet {
    /// Creates a new, empty `SpaceTimeIdSet`.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
    /// Returns an iterator over the `SpaceTimeId` elements contained in the set.
    /// This allows read-only access to each element in the set.
    pub fn iter(&self) -> impl Iterator<Item = &SpaceTimeId> {
        self.inner.iter()
    }
    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

use std::{convert::From, fmt};

impl From<SpaceTimeId> for SpaceTimeIdSet {
    fn from(id: SpaceTimeId) -> Self {
        let mut set = SpaceTimeIdSet::new();
        set.insert(id);
        set
    }
}

impl fmt::Display for SpaceTimeIdSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = self.inner.iter().map(|stid| stid.to_string()).collect();
        write!(f, "{}", elements.join(", "))
    }
}

impl fmt::Debug for SpaceTimeIdSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();

        for stid in &self.inner {
            let z = stid.z();
            let i = stid.i();
            let max_xy = (1u64 << z) - 1;
            let max_f = (1i64 << z) - 1;
            let min_f = -(1i64 << z) - 1;
            let max_t = u32::MAX;

            let x_vals = match stid.x() {
                DimensionRange::Single(v) => vec![v],
                DimensionRange::LimitRange(s, e) => (s..=e).collect(),
                DimensionRange::BeforeUnLimitRange(e) => (0..=e).collect(),
                DimensionRange::AfterUnLimitRange(s) => (s..=max_xy).collect(),
                DimensionRange::Any => (0..=max_xy).collect(),
            };

            let y_vals = match stid.y() {
                DimensionRange::Single(v) => vec![v],
                DimensionRange::LimitRange(s, e) => (s..=e).collect(),
                DimensionRange::BeforeUnLimitRange(e) => (0..=e).collect(),
                DimensionRange::AfterUnLimitRange(s) => (s..=max_xy).collect(),
                DimensionRange::Any => (0..=max_xy).collect(),
            };

            let f_vals = match stid.f() {
                DimensionRange::Single(v) => vec![v],
                DimensionRange::LimitRange(s, e) => (s..=e).collect(),
                DimensionRange::BeforeUnLimitRange(e) => (min_f..=e).collect(),
                DimensionRange::AfterUnLimitRange(s) => (s..=max_f).collect(),
                DimensionRange::Any => (min_f..=max_f).collect(),
            };

            let t_vals = match stid.t() {
                DimensionRange::Single(v) => vec![v],
                DimensionRange::LimitRange(s, e) => (s..=e).collect(),
                DimensionRange::BeforeUnLimitRange(e) => (0..=e).collect(),
                DimensionRange::AfterUnLimitRange(s) => (s..=max_t).collect(),
                DimensionRange::Any => vec![0], // 無限ループ回避のため、必要なら max_t を含めても良い
            };

            for &x in &x_vals {
                for &y in &y_vals {
                    for &f in &f_vals {
                        for &t in &t_vals {
                            if i == 0 {
                                list.entry(&format_args!("{}/{}/{}/{}", z, f, x, y));
                            } else {
                                list.entry(&format_args!("{}/{}/{}/{}_{}/{}", z, x, y, f, i, t));
                            }
                        }
                    }
                }
            }
        }

        list.finish()
    }
}

impl IntoIterator for SpaceTimeIdSet {
    type Item = SpaceTimeId;
    type IntoIter = std::vec::IntoIter<SpaceTimeId>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a SpaceTimeIdSet {
    type Item = &'a SpaceTimeId;
    type IntoIter = std::slice::Iter<'a, SpaceTimeId>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl FromIterator<SpaceTimeId> for SpaceTimeIdSet {
    fn from_iter<I: IntoIterator<Item = SpaceTimeId>>(iter: I) -> Self {
        let mut set = SpaceTimeIdSet::new();
        for stid in iter {
            set.insert(stid);
        }
        set
    }
}
