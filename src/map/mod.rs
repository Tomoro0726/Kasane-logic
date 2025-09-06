use std::process::id;

use crate::id::z_range::{F_MAX, F_MIN, XY_MAX};
use crate::id::DimensionRange::{
    self, AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single,
};
use crate::id::SpaceTimeId;

#[derive(Debug)]
struct SpaceTimeIdMap<T> {
    inner: Inner<T>,
}

#[derive(Debug)]
enum Inner<T> {
    Value(T),
    Children(Children<T>),
}

#[derive(Debug)]
pub struct Children<T> {
    pub mask: u8,                           // どの子が存在するかをビットマスク
    pub nodes: Vec<Box<SpaceTimeIdMap<T>>>, // 存在する子だけ格納
}

impl<T: Default> SpaceTimeIdMap<T> {
    fn new() -> Self {
        Self {
            inner: Inner::Children(Children {
                mask: 0,
                nodes: Vec::new(),
            }),
        }
    }

    fn insert(&mut self, id: SpaceTimeId, v: T) -> Result<(), String> {
        //IDが0の場合
        if id.z() == 0 {
            match &self.inner {
                //z=0がvalueを持っている場合
                Inner::Value(_) => return Err("既にZ=0はValueを持っています".to_string()),
                Inner::Children(children) => {
                    //Childrenがいない場合は問題なし
                    //問題はChildrenがいる場合
                }
            }
        }

        Self::insert_innner(self, id, v, 0)
    }

    fn insert_innner(&mut self, id: SpaceTimeId, v: T, top_z: u8) -> Result<(), String> {
        //まずIDの状態を判定する
        let ids = split_id(id, top_z);
        for id in ids {}
    }
}

fn split_id(id: SpaceTimeId, top_z: u8) -> Vec<(SpaceTimeId, u8)> {
    if top_z > id.z() {
        panic!("知らん");
    };

    let (f_start, f_end) = dimension_range_to_bounds_f(&id.f(), id.z());
    let (x_start, x_end) = dimension_range_to_bounds_xy(&id.x(), id.z());
    let (y_start, y_end) = dimension_range_to_bounds_xy(&id.y(), id.z());

    //Trueの場合は0
    //Falseの場合は1
    let f_intervals: Vec<((i32, i32), bool)> =
        intervals_until_boundary_f(top_z - id.z(), f_start, f_end);
    let x_intervals: Vec<((u32, u32), bool)> =
        intervals_until_boundary_xy(top_z - id.z(), x_start, x_end);
    let y_intervals: Vec<((u32, u32), bool)> =
        intervals_until_boundary_xy(top_z - id.z(), y_start, y_end);

    //u8はOcTreeの位置
    let mut result = Vec::new();

    for (f_range, f_flag) in &f_intervals {
        for (x_range, x_flag) in &x_intervals {
            for (y_range, y_flag) in &y_intervals {
                let stid = SpaceTimeId::new(
                    top_z,
                    LimitRange(f_range.0, f_range.1),
                    LimitRange(x_range.0, x_range.1),
                    LimitRange(y_range.0, y_range.1),
                    id.i(),
                    id.t(),
                )
                .expect("Failed to create SpaceTimeId");

                // OcTree mask を計算
                let mask = flags_to_mask(*f_flag, *x_flag, *y_flag);

                result.push((stid, mask));
            }
        }
    }

    result
}

fn flags_to_mask(f_flag: bool, x_flag: bool, y_flag: bool) -> u8 {
    let f_bit = if f_flag { 0 } else { 1 } << 2; // bit 2
    let x_bit = if x_flag { 0 } else { 1 } << 1; // bit 1
    let y_bit = if y_flag { 0 } else { 1 } << 0; // bit 0
    f_bit | x_bit | y_bit
}

fn dimension_range_to_bounds_xy(dim: &DimensionRange<u32>, z: u8) -> (u32, u32) {
    let max = XY_MAX[z as usize];
    match dim {
        Single(v) => (*v, *v),
        LimitRange(s, e) => (*s, *e),
        BeforeUnLimitRange(e) => (0, *e),
        AfterUnLimitRange(s) => (*s, max),
        Any => (0, max),
    }
}

fn dimension_range_to_bounds_f(dim: &DimensionRange<i32>, z: u8) -> (i32, i32) {
    let max = F_MAX[z as usize];
    let min = F_MIN[z as usize];
    match dim {
        Single(v) => (*v, *v),
        LimitRange(s, e) => (*s, *e),
        BeforeUnLimitRange(e) => (min, *e),
        AfterUnLimitRange(s) => (*s, max),
        Any => (min, max),
    }
}

//境界のデータを作成する
fn intervals_until_boundary_f(n: u8, start: i32, end: i32) -> Vec<((i32, i32), bool)> {
    let step = 1 << n;
    let mut result = Vec::new();

    let mut current = start;
    let mut b = (start >> n) << n;
    if b < start {
        b += step;
    }

    while current <= end {
        let next = b.saturating_sub(1);
        let interval_end = if next > end { end } else { next };
        let value = ((current >> n) & 1) == 0;

        result.push(((current, interval_end), value));

        current = b;
        b = b.saturating_add(step);

        // 境界は1つだけなので、見つけたらストップ
        if current <= end {
            let next_value = ((current >> n) & 1) == 0;
            if next_value != value {
                let next_interval_end = if b.saturating_sub(1) > end {
                    end
                } else {
                    b.saturating_sub(1)
                };
                result.push(((current, next_interval_end), next_value));
                break;
            }
        }
    }

    result
}

fn intervals_until_boundary_xy(n: u8, start: u32, end: u32) -> Vec<((u32, u32), bool)> {
    let step = 1 << n;
    let mut result = Vec::new();

    let mut current = start;
    let mut b = (start >> n) << n;
    if b < start {
        b += step;
    }

    while current <= end {
        let next = b.saturating_sub(1);
        let interval_end = if next > end { end } else { next };
        let value = ((current >> n) & 1) == 0;

        result.push(((current, interval_end), value));

        current = b;
        b = b.saturating_add(step);

        // 境界は1つだけなので、見つけたらストップ
        if current <= end {
            let next_value = ((current >> n) & 1) == 0;
            if next_value != value {
                let next_interval_end = if b.saturating_sub(1) > end {
                    end
                } else {
                    b.saturating_sub(1)
                };
                result.push(((current, next_interval_end), next_value));
                break;
            }
        }
    }

    result
}
