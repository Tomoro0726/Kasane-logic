use crate::{
    id::{
        DimensionRange::{self, AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single},
        SpaceTimeId,
    },
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
        if self.x == DimensionRange::Any
            && self.y == DimensionRange::Any
            && self.f == DimensionRange::Any
        {
            return SpaceTimeIdSet::new();
        }

        // i=0かつt=Anyの場合は時間次元はそのまま
        let is_pure_space = self.i == 0 && self.t == DimensionRange::Any;

        // z=0ではデフォルトで全体を表すので、早期リターンする
        if self.z == 0 {
            return SpaceTimeIdSet::new();
        }

        let x_inversions = Self::split_xy_dimension(&self.x, self.z);
        let y_inversions = Self::split_xy_dimension(&self.y, self.z);
        let f_inversions = Self::split_f_dimension(&self.f, self.z);
        let t_inversions = if is_pure_space {
            vec![self.t] // 時間次元はそのまま
        } else {
            Self::split_t_dimension(&self.t)
        };

        /// None は未確定の値を意味する
        #[derive(Debug, Clone, Copy)]
        struct FXY {
            f: Option<DimensionRange<i64>>,
            x: Option<DimensionRange<u64>>,
            y: Option<DimensionRange<u64>>,
        }

        let mut tmp = Vec::new();
        let mut result = SpaceTimeIdSet::new();

        // X の値に関して処理
        match x_inversions.as_slice() {
            [] => tmp.push(FXY {
                f: None,
                x: Some(self.x),
                y: None,
            }),
            [a] => {
                tmp.push(FXY {
                    f: Some(DimensionRange::Any),
                    x: Some(*a),
                    y: Some(DimensionRange::Any),
                });
                tmp.push(FXY {
                    f: None,
                    x: Some(self.x),
                    y: None,
                });
            }
            [a, b] => {
                tmp.push(FXY {
                    f: Some(DimensionRange::Any),
                    x: Some(*a),
                    y: Some(DimensionRange::Any),
                });
                tmp.push(FXY {
                    f: None,
                    x: Some(self.x),
                    y: None,
                });
                tmp.push(FXY {
                    f: Some(DimensionRange::Any),
                    x: Some(*b),
                    y: Some(DimensionRange::Any),
                });
            }
            _ => panic!("x_inversions の配列長がおかしい！"),
        }

        println!("X_TMP{:?}", tmp);

        // Y の補集合を展開
        tmp = tmp
            .into_iter()
            .flat_map(|a| {
                if a.y.is_none() {
                    match y_inversions.as_slice() {
                        [y1, y2] => vec![
                            FXY {
                                f: Some(Any),
                                y: Some(*y1),
                                ..a
                            },
                            FXY {
                                y: Some(self.y),
                                ..a
                            },
                            FXY {
                                f: Some(Any),
                                y: Some(*y2),
                                ..a
                            },
                        ],
                        [y1] => vec![
                            FXY {
                                f: Some(Any),

                                y: Some(*y1),
                                ..a
                            },
                            FXY {
                                y: Some(self.y),
                                ..a
                            },
                        ],
                        _ => panic!("y_inversions の配列長がおかしい！"),
                    }
                } else {
                    vec![a]
                }
            })
            .collect();

        println!("XY_TMP{:?}", tmp);

        // F の補集合を展開
        tmp = tmp
            .into_iter()
            .flat_map(|a| {
                if a.f.is_none() {
                    match f_inversions.as_slice() {
                        [f1, f2] => vec![FXY { f: Some(*f1), ..a }, FXY { f: Some(*f2), ..a }],
                        [f1] => vec![FXY { f: Some(*f1), ..a }],
                        _ => panic!("f_inversions の配列長がおかしい！"),
                    }
                } else {
                    vec![a]
                }
            })
            .collect();

        println!("XYF_TMP{:?}", tmp);

        // 時間軸の補集合を展開
        for ele in tmp {
            match t_inversions.as_slice() {
                [t1, t2] => {
                    for t in [t1, t2] {
                        let id = SpaceTimeId::new(
                            self.z,
                            ele.f.expect("f 未設定"),
                            ele.x.expect("x 未設定"),
                            ele.y.expect("y 未設定"),
                            self.i,
                            *t,
                        )
                        .expect("補集合 SpaceTimeId の生成に失敗");
                        result.insert(id);
                    }
                }
                [t1] => {
                    let id = SpaceTimeId::new(
                        self.z,
                        ele.f.expect("f 未設定"),
                        ele.x.expect("x 未設定"),
                        ele.y.expect("y 未設定"),
                        self.i,
                        *t1,
                    )
                    .expect("補集合 SpaceTimeId の生成に失敗");
                    result.insert(id);
                }
                [] => {
                    // Any の場合は補集合なし
                }
                _ => panic!("t_inversions の配列長がおかしい！"),
            }
        }

        result
    }

    /// Inverts a spatial dimension range (x or y) for complement calculation.
    fn split_xy_dimension(dim_range: &DimensionRange<u64>, z: u16) -> Vec<DimensionRange<u64>> {
        let max = (1u64 << z) - 1;
        match dim_range {
            DimensionRange::Single(v) => {
                if *v == 0 {
                    vec![DimensionRange::LimitRange(1, max)]
                } else if *v == max {
                    vec![DimensionRange::LimitRange(0, max - 1)]
                } else {
                    vec![
                        DimensionRange::LimitRange(0, v - 1),
                        DimensionRange::LimitRange(v + 1, max),
                    ]
                }
            }
            DimensionRange::LimitRange(s, e) => {
                let mut res = Vec::new();
                if *s > 0 {
                    res.push(DimensionRange::LimitRange(0, s - 1));
                }
                if *e < max {
                    res.push(DimensionRange::LimitRange(e + 1, max));
                }
                res
            }
            DimensionRange::AfterUnLimitRange(s) => {
                if *s == 0 {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(0, s - 1)]
                }
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                if *e == max {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(e + 1, max)]
                }
            }
            DimensionRange::Any => vec![],
        }
    }

    /// Inverts a vertical dimension range (f) for complement calculation.
    fn split_f_dimension(dim_range: &DimensionRange<i64>, z: u16) -> Vec<DimensionRange<i64>> {
        let max = (1i64 << z) - 1;
        let min = -(1i64 << z);
        match dim_range {
            DimensionRange::Single(v) => {
                if *v == min {
                    vec![DimensionRange::LimitRange(min + 1, max)]
                } else if *v == max {
                    vec![DimensionRange::LimitRange(min, max - 1)]
                } else {
                    vec![
                        DimensionRange::LimitRange(min, v - 1),
                        DimensionRange::LimitRange(v + 1, max),
                    ]
                }
            }
            DimensionRange::LimitRange(s, e) => {
                let mut res = Vec::new();
                if *s > min {
                    res.push(DimensionRange::LimitRange(min, s - 1));
                }
                if *e < max {
                    res.push(DimensionRange::LimitRange(e + 1, max));
                }
                res
            }
            DimensionRange::AfterUnLimitRange(s) => {
                if *s <= min {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(min, s - 1)]
                }
            }
            DimensionRange::BeforeUnLimitRange(e) => {
                if *e >= max {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(e + 1, max)]
                }
            }
            DimensionRange::Any => vec![],
        }
    }

    /// Inverts a temporal dimension range (t) for complement calculation.
    fn split_t_dimension(dim_range: &DimensionRange<u32>) -> Vec<DimensionRange<u32>> {
        match dim_range {
            DimensionRange::Single(v) => {
                if *v == 0 {
                    vec![DimensionRange::AfterUnLimitRange(1)]
                } else {
                    vec![
                        DimensionRange::LimitRange(0, v - 1),
                        DimensionRange::AfterUnLimitRange(v + 1),
                    ]
                }
            }
            DimensionRange::LimitRange(s, e) => {
                let mut res = Vec::new();
                if *s > 0 {
                    res.push(DimensionRange::LimitRange(0, s - 1));
                }
                res.push(DimensionRange::AfterUnLimitRange(e + 1));
                res
            }
            DimensionRange::AfterUnLimitRange(s) => {
                if *s == 0 {
                    vec![]
                } else {
                    vec![DimensionRange::LimitRange(0, s - 1)]
                }
            }
            DimensionRange::BeforeUnLimitRange(e) => vec![DimensionRange::AfterUnLimitRange(e + 1)],
            DimensionRange::Any => vec![],
        }
    }
}
