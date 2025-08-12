use crate::{
    id::{DimensionRange, SpaceTimeId},
    set::SpaceTimeIdSet,
};

impl SpaceTimeId {
    /// Calculates the complement of the `SpaceTimeId`.
    ///
    /// The complement represents all space and time *not* covered by this ID.
    /// This is achieved by inverting each dimension's range and then computing the
    /// Cartesian product of all resulting inverted ranges.
    ///
    /// # Returns
    ///
    /// A `SpaceTimeIdSet` containing the set of IDs that represent the complement space.
    /// Returns an empty SpaceTimeIdSet if the complement is an empty set (e.g., the complement of "everything").
    pub fn complement(&self) -> SpaceTimeIdSet {
        //物理的な全ての空間が指定されている場合にはその補集合は空集合である
        if self.x == DimensionRange::Any
            && self.y == DimensionRange::Any
            && self.f == DimensionRange::Any
        {
            return SpaceTimeIdSet::new();
        }

        //物理的な全ての時間が指定されている場合にはその補集合は空集合である
        //時空間IDかつ、tがAnyを持つ時にはその補集合は空集合となる
        if self.i != 0 && self.t == DimensionRange::Any {
            return SpaceTimeIdSet::new();
        }

        let x_inversions = Self::invert_xy_dimension(&self.x, self.z);
        let y_inversions = Self::invert_xy_dimension(&self.y, self.z);
        let f_inversions = Self::invert_f_dimension(&self.f, self.z);
        let t_inversions = Self::invert_t_dimension(&self.t);

        // 空が含まれる場合（全体補集合が空）のショートカット
        if x_inversions.is_empty()
            || y_inversions.is_empty()
            || f_inversions.is_empty()
            || t_inversions.is_empty()
        {
            return SpaceTimeIdSet::new();
        }

        let mut result = SpaceTimeIdSet::new();
        for (x_num, xi) in x_inversions.iter().enumerate() {
            for (y_num, yi) in y_inversions.iter().enumerate() {
                for (f_num, fi) in f_inversions.iter().enumerate() {
                    for (t_num, ti) in t_inversions.iter().enumerate() {
                        if x_num == 0 && y_num == 0 && f_num == 0 && t_num == 0 {
                            continue;
                        } else {
                            let stid = SpaceTimeId::new(
                                self.z,
                                fi.clone(),
                                xi.clone(),
                                yi.clone(),
                                self.i,
                                ti.clone(),
                            )
                            .expect("Invalid SpaceTimeId generated during complement");

                            result.insert(stid);
                        }
                    }
                }
            }
        }
        result
    }

    /// Inverts a spatial dimension range (x or y).
    fn invert_xy_dimension(dim_range: &DimensionRange<u64>, z: u16) -> Vec<DimensionRange<u64>> {
        let max = (1u64 << z) - 1;
        let mut result = Vec::new();

        match *dim_range {
            DimensionRange::Single(v) => {
                result.push(*dim_range);
                if v > 0 {
                    result.push(DimensionRange::BeforeUnLimitRange(v - 1));
                }
                if v < max {
                    result.push(DimensionRange::AfterUnLimitRange(v + 1));
                }
            }
            DimensionRange::LimitRange(s, e) => {
                result.push(*dim_range);
                if s > 0 {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                }
                if e < max {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                }
            }
            DimensionRange::AfterUnLimitRange(s) => {
                result.push(*dim_range);
                if s > 0 {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                }
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                result.push(*dim_range);
                if e < max {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                }
            }
            DimensionRange::Any => {
                result.push(DimensionRange::Any);
            }
        }
        result
    }

    /// Inverts a vertical dimension range (f).
    fn invert_f_dimension(dim_range: &DimensionRange<i64>, z: u16) -> Vec<DimensionRange<i64>> {
        let limit = 1i64 << z;
        let max = limit - 1;
        let min = -limit;
        let mut result = Vec::new();

        match *dim_range {
            DimensionRange::Single(v) => {
                result.push(*dim_range);
                if v > min {
                    result.push(DimensionRange::BeforeUnLimitRange(v - 1));
                }
                if v < max {
                    result.push(DimensionRange::AfterUnLimitRange(v + 1));
                }
            }
            DimensionRange::LimitRange(s, e) => {
                result.push(*dim_range);
                if s > min {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                }
                if e < max {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                }
            }
            DimensionRange::AfterUnLimitRange(s) => {
                result.push(*dim_range);
                if s > min {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                }
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                result.push(*dim_range);

                if e < max {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                }
            }
            DimensionRange::Any => {
                result.push(DimensionRange::Any);
            }
        }
        result
    }

    /// Inverts a temporal dimension range (t). Assumes an unbounded positive range.
    fn invert_t_dimension(dim_range: &DimensionRange<u32>) -> Vec<DimensionRange<u32>> {
        let mut result = Vec::new();
        match *dim_range {
            DimensionRange::Single(v) => {
                result.push(*dim_range);
                if v > 0 {
                    result.push(DimensionRange::BeforeUnLimitRange(v - 1));
                }
                result.push(DimensionRange::AfterUnLimitRange(v + 1));
            }
            DimensionRange::LimitRange(s, e) => {
                result.push(*dim_range);

                if s > 0 {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                }
                result.push(DimensionRange::AfterUnLimitRange(e + 1));
            }
            DimensionRange::AfterUnLimitRange(s) => {
                result.push(*dim_range);

                if s > 0 {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                }
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                result.push(*dim_range);

                result.push(DimensionRange::AfterUnLimitRange(e + 1));
            }
            DimensionRange::Any => {
                result.push(DimensionRange::Any);
            }
        }
        result
    }
}
