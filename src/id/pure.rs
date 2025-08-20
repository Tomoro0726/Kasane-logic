use crate::id::DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single};
use crate::id::{DimensionRange, SpaceTimeId};

impl SpaceTimeId {
    /// Expands all dimension ranges into individual `SpaceTimeId` instances with single values only.
    /// 
    /// This method converts extended notation (Range, Before, After, Any) into a collection of
    /// pure space-time IDs where each dimension uses only `Single` variants. This is useful
    /// for processing operations that require discrete, individual space-time cells.
    /// 
    /// # Returns
    /// 
    /// A `Vec<SpaceTimeId>` containing all individual IDs that represent the same space-time
    /// region as the original ID, but with each dimension expanded to single values.
    /// 
    /// # Note
    /// 
    /// The T dimension is preserved as-is and not expanded, maintaining the original temporal range.
    /// 
    /// # Japanese Note
    /// 
    /// 拡張記法 (Range, Before, After, Any) をすべて展開して
    /// 各次元が Single だけの純粋な ID 群を返す
    pub fn pure(&self) -> Vec<SpaceTimeId> {
        let z = self.z();
        let i = self.i();

        let max_xy = (1u64 << z) - 1;

        let max_f = (1i64 << z) - 1;
        let min_f = -(1i64 << z);

        let expand_u64 = |range: &DimensionRange<u64>, max: u64| -> Vec<u64> {
            match range {
                Single(v) => vec![*v],
                LimitRange(s, e) => (*s..=*e).collect(),
                BeforeUnLimitRange(e) => (0..=*e).collect(),
                AfterUnLimitRange(s) => (*s..=max).collect(),
                Any => (0..=max).collect(),
            }
        };

        let expand_i64 = |range: &DimensionRange<i64>, min: i64, max: i64| -> Vec<i64> {
            match range {
                Single(v) => vec![*v],
                LimitRange(s, e) => (*s..=*e).collect(),
                BeforeUnLimitRange(e) => (min..=*e).collect(),
                AfterUnLimitRange(s) => (*s..=max).collect(),
                Any => (min..=max).collect(),
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
                            Single(f),
                            Single(x),
                            Single(y),
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
