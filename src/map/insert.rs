use crate::{
    id::{DimensionRange, SpaceTimeId},
    map::{Children, Inner, SpaceTimeIdMap},
};

use crate::id::z_range::{F_MAX, F_MIN, XY_MAX};
use crate::id::DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single};

impl<T: Default + Clone> SpaceTimeIdMap<T> {
    pub fn insert(&mut self, id: SpaceTimeId, v: T) -> Result<(), String> {
        //Todo:上下の判定と挿入の場合分けの関数を作成
        //Zの自動的な昇格
        if id.z() == 0 {
            match &mut self.up_inner {
                Inner::Value(_) => return Err("既にZ=0はValueを持っています".to_string()),
                Inner::Children(children) => {
                    if children.mask == 0 {
                        self.up_inner = Inner::Value(v);
                        return Ok(());
                    } else {
                        return Err("既にZ=0はそれ以下のZoomLevelでValueを持っています".to_string());
                    }
                }
            }
        }

        Self::insert_innner(&mut self.up_inner, id, v, 1)
    }

    fn insert_innner(o: &mut Inner<T>, id: SpaceTimeId, v: T, top_z: u8) -> Result<(), String> {
        // 目的の深さに到達した場合
        if top_z == id.z() {
            match o {
                Inner::Value(_) => {
                    return Err(format!("既にZ={}はValueを持っています", top_z));
                }
                Inner::Children(children) => {
                    if children.mask != 0 {
                        return Err(format!(
                            "既にZ={}はそれ以下のZoomLevelでValueを持っています",
                            top_z
                        ));
                    }
                    *o = Inner::Value(v);
                    return Ok(());
                }
            }
        }

        // まだ目的の深さに到達していない場合
        match o {
            Inner::Value(_) => {
                return Err(format!("既にZ={}はValueを持っています", top_z));
            }
            Inner::Children(children) => {
                // 現在の深さでのIDの分割を取得
                let ids = split_id(id, top_z);

                // 各子ノードに対して処理
                for (child_id, child_mask) in ids {
                    // child_maskに対応する子ノードの位置を探す
                    let child_pos = Self::find_child_position(children, child_mask);

                    match child_pos {
                        Some(pos) => {
                            // 既存の子ノードがある場合、再帰的に挿入
                            let child_node = &mut children.nodes[pos];
                            Self::insert_innner(
                                &mut child_node.up_inner,
                                id,
                                v.clone(),
                                top_z + 1,
                            )?;
                        }
                        None => {
                            // 新しい子ノードを作成
                            let mut new_map = SpaceTimeIdMap::new();
                            Self::insert_innner(&mut new_map.up_inner, id, v.clone(), top_z + 1)?;

                            // 子ノードを正しい位置に挿入
                            Self::insert_child_node(children, new_map, child_mask);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // 子ノードの位置を見つけるヘルパー関数（修正版）
    fn find_child_position(children: &Children<T>, target_mask: u8) -> Option<usize> {
        // target_maskが存在するかチェック
        let target_bit = 1u8 << target_mask;
        if children.mask & target_bit == 0 {
            return None; // 存在しない
        }

        // target_maskより小さいビットが立っている数を数える（これが配列インデックス）
        let mut position = 0;
        for i in 0..target_mask {
            let bit = 1u8 << i;
            if children.mask & bit != 0 {
                position += 1;
            }
        }

        Some(position)
    }

    // 子ノードを正しい位置に挿入するヘルパー関数（修正版）
    fn insert_child_node(children: &mut Children<T>, new_node: SpaceTimeIdMap<T>, child_mask: u8) {
        // child_maskより小さいビットが立っている数を計算（挿入位置）
        let mut insert_pos = 0;
        for i in 0..child_mask {
            let bit = 1u8 << i;
            if children.mask & bit != 0 {
                insert_pos += 1;
            }
        }

        // ノードを挿入
        children.nodes.insert(insert_pos, Box::new(new_node));

        // maskを更新
        let target_bit = 1u8 << child_mask;
        children.mask |= target_bit;
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
    let f_intervals = intervals_until_boundary_f(top_z - id.z(), f_start, f_end);

    let x_intervals = intervals_until_boundary_xy(top_z - id.z(), x_start, x_end);
    let y_intervals = intervals_until_boundary_xy(top_z - id.z(), y_start, y_end);

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
    let step: u64 = 1u64 << n;
    let mut result = Vec::new();

    // オフセットして u64 に変換
    let offset = i64::from(i32::MIN);
    let start_u = (i64::from(start) - offset) as u64;
    let end_u = (i64::from(end) - offset) as u64;

    let mut current = start_u;
    let mut b = (start_u >> n) << n;
    if b < start_u {
        b = b.saturating_add(step);
    }

    while current <= end_u {
        let next = b.saturating_sub(1);
        let interval_end = if next > end_u { end_u } else { next };
        let value = ((current >> n) & 1) == 0;

        result.push((
            (
                (current as i64 + offset) as i32,
                (interval_end as i64 + offset) as i32,
            ),
            value,
        ));

        current = b;
        b = b.saturating_add(step);

        if current <= end_u {
            let next_value = ((current >> n) & 1) == 0;
            if next_value != value {
                let next_interval_end = if b.saturating_sub(1) > end_u {
                    end_u
                } else {
                    b.saturating_sub(1)
                };
                result.push((
                    (
                        (current as i64 + offset) as i32,
                        (next_interval_end as i64 + offset) as i32,
                    ),
                    next_value,
                ));
                break;
            }
        }
    }

    result
}

fn intervals_until_boundary_xy(n: u8, start: u32, end: u32) -> Vec<((u32, u32), bool)> {
    let step: u32 = 1u32 << n; // safe
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
