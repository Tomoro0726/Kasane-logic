pub mod center;
pub mod change_scale;
pub mod complement;
pub mod coordinates;
pub mod relation;
pub mod to_pure;
pub mod value;
pub mod vertex;

/// Represents a value for a single dimension (F, X, Y, or T) in a SpaceTimeId.
///
/// This enum can represent a single value, a range, or an Any value,
/// corresponding to the extended notation of the spatial ID.
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DimensionRange<T> {
    /// A closed range with a start and end value (e.g., 5:10).
    LimitRange(T, T),
    /// An unlimited range up to an end value (e.g., -:10).
    BeforeUnLimitRange(T),
    /// An unlimited range from a start value onwards (e.g., 5:-).
    AfterUnLimitRange(T),
    /// A single, discrete value.
    Single(T),
    /// An Any value, indicating it applies to all possible values in this dimension (e.g., -).
    Any,
}
use std::fmt;

impl<T> fmt::Display for DimensionRange<T>
where
    T: fmt::Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DimensionRange::LimitRange(start, end) => write!(f, "{}:{}", start, end),
            DimensionRange::BeforeUnLimitRange(v) => write!(f, "-:{}", v),
            DimensionRange::AfterUnLimitRange(v) => write!(f, "{}:-", v),
            DimensionRange::Single(v) => write!(f, "{}", v),
            DimensionRange::Any => write!(f, "-"),
        }
    }
}
#[derive(PartialEq, Debug, Clone, Copy)]
/// * z: The zoom level, a u16 value. It defines the coordinate space boundaries.
/// * f: The value for the F (vertical) dimension as a DimensionRange<i64>.
/// * x: The value for the X dimension as a DimensionRange<u64>.
/// * y: The value for the Y dimension as a DimensionRange<u64>.
/// * i: The time interval in seconds, a u32 value.
/// * t: The time index value as a DimensionRange<u32>.
///
/// # Normalization Rules
///
/// The input `DimensionRange` values are normalized during validation inside `SpaceTimeId::new`
/// according to the following rules:
///
/// ## `DimensionRange::Single`
/// - No normalization applied (used as-is).
///
/// ## `DimensionRange::LimitRange(start, end)`
/// - If `start == end`: converted to `Single(start)`.
/// - If the range spans the entire valid domain:
///   - For x/y: `0..=2^z - 1`
///   - For f: `-2^z..=2^z - 1`
///   → converted to `Any`.
/// - If `start == 0`: converted to `BeforeUnLimitRange(end)`.
/// - If `end == max`: converted to `AfterUnLimitRange(start)`.
///
/// ## `DimensionRange::AfterUnLimitRange(start)`
/// - If `start == 0` (for x/y) or `start == min_f` (for f): converted to `Any`.
/// - If `start == max` (for x/y only): converted to `Single(max)`.
///
/// ## `DimensionRange::BeforeUnLimitRange(end)`
/// - If `end == max` (for x/y) or `end == max_f` (for f): converted to `Any`.
/// - If `end == 0` (for x/y) or `end == min_f` (for f): converted to `Single(end)`.
///
/// ## `DimensionRange::Any`
/// - No normalization applied (used as-is).
///
/// ## Special case for time dimension `t`
/// - If `i == 0`, then `t` **must** be `Any`. Otherwise, an error is returned.

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct SpaceTimeId {
    z: u16,
    f: DimensionRange<i64>,
    x: DimensionRange<u64>,
    y: DimensionRange<u64>,
    i: u32,
    t: DimensionRange<u32>,
}

impl fmt::Display for SpaceTimeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}_{}/{}",
            self.z, self.f, self.x, self.y, self.i, self.t
        )
    }
}

impl SpaceTimeId {
    /// Creates a new `SpaceTimeId` and normalizes all dimension ranges.
    ///
    /// # Normalization Rules
    ///
    /// Each dimension (`x`, `y`, `f`, `t`) is normalized so that
    /// redundant or equivalent representations are reduced to a canonical form.
    ///
    /// ## XY dimensions
    /// - `AfterUnLimitRange(0)` → `Any`
    /// - `AfterUnLimitRange(max)` → `Single(max)`
    /// - `BeforeUnLimitRange(max)` → `Any`
    /// - `BeforeUnLimitRange(0)` → `Single(0)`
    ///
    /// ## F dimension
    /// - `AfterUnLimitRange(min_f)` → `Any`
    /// - `AfterUnLimitRange(max_f)` → `Single(max_f)`
    /// - `BeforeUnLimitRange(max_f)` → `Any`
    /// - `BeforeUnLimitRange(min_f)` → `Single(min_f)`
    ///
    /// (`min_f = -2^z`, `max_f = 2^z - 1`)
    ///
    /// ## T dimension
    /// - If `i == 0` (purely spatial ID), then `t` must be `Any`  
    ///   otherwise an error is returned.
    ///
    /// # Errors
    /// - If any range value is outside its valid bounds for the given zoom `z`
    /// - If `t` is not `Any` when `i == 0`
    pub fn new(
        z: u16,
        f: DimensionRange<i64>,
        x: DimensionRange<u64>,
        y: DimensionRange<u64>,
        i: u32,
        t: DimensionRange<u32>,
    ) -> Result<Self, String> {
        if z >= 32 {
            return Err(format!(
                "Zoom level z must be less than 32 to prevent overflow. Received: {}.",
                z
            ));
        }

        fn validate_xy_dim(
            dim_val: &DimensionRange<u64>,
            z: u16,
        ) -> Result<DimensionRange<u64>, String> {
            let max = (1u64 << z) - 1;

            match *dim_val {
                DimensionRange::Single(v) => {
                    if v <= max {
                        Ok(DimensionRange::Single(v))
                    } else {
                        Err(format!(
                            "value {} is out of bounds for zoom level {}. Must be less than {}.",
                            v, z, max
                        ))
                    }
                }
                DimensionRange::LimitRange(start, end) => {
                    if start > end {
                        return validate_xy_dim(&DimensionRange::LimitRange(end, start), z);
                    }
                    if end == start {
                        return Ok(DimensionRange::Single(start));
                    }
                    if end <= max {
                        if start == 0 && end == max {
                            Ok(DimensionRange::Any)
                        } else if start == 0 {
                            Ok(DimensionRange::BeforeUnLimitRange(end))
                        } else if end == max {
                            Ok(DimensionRange::AfterUnLimitRange(start))
                        } else {
                            Ok(DimensionRange::LimitRange(start, end))
                        }
                    } else {
                        Err(format!(
                            "end value {} is out of bounds for zoom level {}. Must be less than {}.",
                            end, z, max
                        ))
                    }
                }
                DimensionRange::AfterUnLimitRange(start) => {
                    if start <= max {
                        if start == 0 {
                            Ok(DimensionRange::Any)
                        } else if start == max {
                            Ok(DimensionRange::Single(max))
                        } else {
                            Ok(DimensionRange::AfterUnLimitRange(start))
                        }
                    } else {
                        Err(format!(
                            "start value {} is out of bounds for zoom level {}. Must be less than {}.",
                            start, z, max
                        ))
                    }
                }
                DimensionRange::BeforeUnLimitRange(end) => {
                    if end <= max {
                        if end == max {
                            Ok(DimensionRange::Any)
                        } else if end == 0 {
                            Ok(DimensionRange::Single(0))
                        } else {
                            Ok(DimensionRange::BeforeUnLimitRange(end))
                        }
                    } else {
                        Err(format!(
                            "end value {} is out of bounds for zoom level {}. Must be less than {}.",
                            end, z, max
                        ))
                    }
                }
                DimensionRange::Any => Ok(DimensionRange::Any),
            }
        }

        fn validate_f_dim(
            dim_val: &DimensionRange<i64>,
            z: u16,
        ) -> Result<DimensionRange<i64>, String> {
            let limit = 1i64 << z;
            let max_f = limit - 1;
            let min_f = -limit;

            match *dim_val {
                DimensionRange::Single(v) => {
                    if v >= min_f && v <= max_f {
                        Ok(DimensionRange::Single(v))
                    } else {
                        Err(format!(
                            "value {} is out of bounds for zoom level {}. Must be between {} and {}.",
                            v, z, min_f, max_f
                        ))
                    }
                }
                DimensionRange::LimitRange(start, end) => {
                    if start > end {
                        return validate_f_dim(&DimensionRange::LimitRange(end, start), z);
                    }
                    if end == start {
                        return Ok(DimensionRange::Single(start));
                    }
                    if end <= max_f {
                        if start == min_f && end == max_f {
                            Ok(DimensionRange::Any)
                        } else if start == min_f {
                            Ok(DimensionRange::BeforeUnLimitRange(end))
                        } else if end == max_f {
                            Ok(DimensionRange::AfterUnLimitRange(start))
                        } else {
                            Ok(DimensionRange::LimitRange(start, end))
                        }
                    } else {
                        Err(format!(
                            "range {}:{} is out of bounds for zoom level {}. Must be within [{}, {}].",
                            start, end, z, min_f, max_f
                        ))
                    }
                }
                DimensionRange::AfterUnLimitRange(start) => {
                    if start >= min_f && start <= max_f {
                        if start == min_f {
                            Ok(DimensionRange::Any)
                        } else if start == max_f {
                            Ok(DimensionRange::Single(max_f))
                        } else {
                            Ok(DimensionRange::AfterUnLimitRange(start))
                        }
                    } else {
                        Err(format!(
                            "start value {} is out of bounds for zoom level {}. Must be between {} and {}.",
                            start, z, min_f, max_f
                        ))
                    }
                }
                DimensionRange::BeforeUnLimitRange(end) => {
                    if end >= min_f && end <= max_f {
                        if end == max_f {
                            Ok(DimensionRange::Any)
                        } else if end == min_f {
                            Ok(DimensionRange::Single(min_f))
                        } else {
                            Ok(DimensionRange::BeforeUnLimitRange(end))
                        }
                    } else {
                        Err(format!(
                            "end value {} is out of bounds for zoom level {}. Must be between {} and {}.",
                            end, z, min_f, max_f
                        ))
                    }
                }
                DimensionRange::Any => Ok(DimensionRange::Any),
            }
        }

        fn validate_t_dim(
            dim_val: &DimensionRange<u32>,
            i: u32,
        ) -> Result<DimensionRange<u32>, String> {
            if i == 0 {
                if *dim_val != DimensionRange::Any {
                    return Err("t must be Any when time interval i is 0.".to_string());
                } else {
                    return Ok(DimensionRange::Any);
                }
            }

            match *dim_val {
                DimensionRange::Single(_) => Ok(dim_val.clone()),

                DimensionRange::LimitRange(start, end) => {
                    if start > end {
                        return Err(format!(
                            "t dimension range is invalid: start {} > end {}.",
                            start, end
                        ));
                    }
                    if start == end {
                        return Ok(DimensionRange::Single(start));
                    };
                    if start == 0 {
                        return Ok(DimensionRange::BeforeUnLimitRange(end));
                    }
                    Ok(DimensionRange::LimitRange(start, end))
                }

                DimensionRange::BeforeUnLimitRange(end) => {
                    if end == 0 {
                        return Ok(DimensionRange::Single(0));
                    } else {
                        Ok(DimensionRange::BeforeUnLimitRange(end))
                    }
                }

                DimensionRange::AfterUnLimitRange(start) => {
                    if start == 0 {
                        Ok(DimensionRange::Any)
                    } else {
                        Ok(DimensionRange::AfterUnLimitRange(start))
                    }
                }

                DimensionRange::Any => Ok(DimensionRange::Any),
            }
        }

        Ok(Self {
            z,
            f: validate_f_dim(&f, z).map_err(|e| format!("Invalid f dimension: {}", e))?,
            x: validate_xy_dim(&x, z).map_err(|e| format!("Invalid x dimension: {}", e))?,
            y: validate_xy_dim(&y, z).map_err(|e| format!("Invalid y dimension: {}", e))?,
            i,
            t: validate_t_dim(&t, i).map_err(|e| format!("Invalid t dimension: {}", e))?,
        })
    }
}
