use crate::id::{DimensionRange, SpaceTimeId};

impl SpaceTimeId {
    /// 拡張記法 (Range, Before, After, Any) をすべて展開して
    /// 各次元が Single だけの純粋な ID 群を返す
    pub fn to_pure(&self) -> Vec<SpaceTimeId> {
        let z = self.z();
        let i = self.i();

        let max_xy = (1u64 << z) - 1;

        let max_f = (1i64 << z) - 1;
        let min_f = -(1i64 << z);

        let expand_u64 = |range: &DimensionRange<u64>, max: u64| -> Vec<u64> {
            match range {
                DimensionRange::Single(v) => vec![*v],
                DimensionRange::LimitRange(s, e) => (*s..=*e).collect(),
                DimensionRange::BeforeUnLimitRange(e) => (0..=*e).collect(),
                DimensionRange::AfterUnLimitRange(s) => (*s..=max).collect(),
                DimensionRange::Any => (0..=max).collect(),
            }
        };

        let expand_i64 = |range: &DimensionRange<i64>, min: i64, max: i64| -> Vec<i64> {
            match range {
                DimensionRange::Single(v) => vec![*v],
                DimensionRange::LimitRange(s, e) => (*s..=*e).collect(),
                DimensionRange::BeforeUnLimitRange(e) => (min..=*e).collect(),
                DimensionRange::AfterUnLimitRange(s) => (*s..=max).collect(),
                DimensionRange::Any => (min..=max).collect(),
            }
        };

        let x_vals = expand_u64(&self.x(), max_xy);
        let y_vals = expand_u64(&self.y(), max_xy);
        let f_vals = expand_i64(&self.f(), min_f, max_f);

        let mut result = Vec::new();

        for &x in &x_vals {
            for &y in &y_vals {
                for &f in &f_vals {
                    result.push(
                        SpaceTimeId::new(
                            z,
                            DimensionRange::Single(f),
                            DimensionRange::Single(x),
                            DimensionRange::Single(y),
                            i,
                            self.t().clone(), // t はそのまま
                        )
                        .unwrap(),
                    );
                }
            }
        }

        result
    }
}
