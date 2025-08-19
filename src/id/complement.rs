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

        //もしもZoomLevelが0のときは全体を指し示すため、その範囲の逆は存在しない
        if self.z == 0 {
            return SpaceTimeIdSet::new();
        }

        let x_inversions = Self::split_xy_dimension(&self.x, self.z);
        let y_inversions = Self::split_xy_dimension(&self.y, self.z);
        let f_inversions = Self::split_f_dimension(&self.f, self.z);
        let t_inversions = Self::split_t_dimension(&self.t);

        println!("X{:?}", x_inversions);
        println!("Y{:?}", y_inversions);
        println!("F{:?}", f_inversions);
        println!("T{:?}", t_inversions);

        //どこでAnyにするのか

        let mut result = SpaceTimeIdSet::new();

        result
    }

    /// Inverts a spatial dimension range (x or y) for complement calculation.
    fn split_xy_dimension(dim_range: &DimensionRange<u64>, z: u16) -> Vec<DimensionRange<u64>> {
        let max = (1u64 << z) - 1;
        let mut result = Vec::new();

        match *dim_range {
            DimensionRange::Single(v) => {
                if v == 0 {
                    result.push(DimensionRange::AfterUnLimitRange(1));
                } else if v == max {
                    result.push(DimensionRange::BeforeUnLimitRange(max - 1));
                } else {
                    result.push(DimensionRange::AfterUnLimitRange(v + 1));
                    result.push(DimensionRange::BeforeUnLimitRange(v - 1));
                }
            }
            DimensionRange::LimitRange(s, e) => {
                if s == 0 {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                } else if e == max {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                } else {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                }
            }
            DimensionRange::AfterUnLimitRange(s) => {
                result.push(DimensionRange::BeforeUnLimitRange(s - 1));
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                result.push(DimensionRange::AfterUnLimitRange(e + 1));
            }
            DimensionRange::Any => {}
        }
        result
    }

    /// Inverts a vertical dimension range (f) for complement calculation.
    fn split_f_dimension(dim_range: &DimensionRange<i64>, z: u16) -> Vec<DimensionRange<i64>> {
        let limit = 1i64 << z;
        let max = limit - 1;
        let min = -limit;
        let mut result = Vec::new();

        match *dim_range {
            DimensionRange::Single(v) => {
                if v == min {
                    result.push(DimensionRange::AfterUnLimitRange(1));
                } else if v == max {
                    result.push(DimensionRange::BeforeUnLimitRange(max - 1));
                } else {
                    result.push(DimensionRange::AfterUnLimitRange(v + 1));
                    result.push(DimensionRange::BeforeUnLimitRange(v - 1));
                }
            }
            DimensionRange::LimitRange(s, e) => {
                if s == min {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                } else if e == max {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                } else {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                }
            }
            DimensionRange::AfterUnLimitRange(s) => {
                result.push(DimensionRange::BeforeUnLimitRange(s - 1));
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                result.push(DimensionRange::AfterUnLimitRange(e + 1));
            }
            DimensionRange::Any => {}
        }
        result
    }

    /// Inverts a temporal dimension range (t) for complement calculation.
    fn split_t_dimension(dim_range: &DimensionRange<u32>) -> Vec<DimensionRange<u32>> {
        let mut result = Vec::new();

        match *dim_range {
            DimensionRange::Single(v) => {
                if v == 0 {
                    result.push(DimensionRange::AfterUnLimitRange(1));
                } else {
                    result.push(DimensionRange::BeforeUnLimitRange(v - 1));
                    result.push(DimensionRange::AfterUnLimitRange(v + 1));
                }
            }
            DimensionRange::LimitRange(s, e) => {
                if s == 0 {
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                } else {
                    result.push(DimensionRange::BeforeUnLimitRange(s - 1));
                    result.push(DimensionRange::AfterUnLimitRange(e + 1));
                }
            }
            DimensionRange::AfterUnLimitRange(s) => {
                result.push(DimensionRange::BeforeUnLimitRange(s - 1));
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                result.push(DimensionRange::AfterUnLimitRange(e + 1));
            }
            DimensionRange::Any => {}
        }
        result
    }
}
