use crate::{
    id::{DimensionRange, SpaceTimeId, z_range::{F_MAX, F_MIN, XY_MAX}},
    map::{Inner, SpaceTimeIdMap},
};

impl<T: Default + Clone> SpaceTimeIdMap<T> {
    pub fn iter(&self) -> impl Iterator<Item = (SpaceTimeId, T)> {
        let mut items = Vec::new();

        // up方向を探索 (z >= 1)
        if let Some(bounds) = Self::get_initial_bounds(1) {
            self.collect_recursive(&self.up_inner, &mut items, 1, bounds, 0);
        }

        // down方向を探索 (z = 0) - 特別な処理が必要かもしれませんが、今は同じように扱います
        if let Some(bounds) = Self::get_initial_bounds(0) {
            self.collect_recursive(&self.down_inner, &mut items, 0, bounds, 0);
        }

        items.into_iter()
    }

    fn get_initial_bounds(z: u8) -> Option<(i32, i32, u32, u32, u32, u32)> {
        if z >= 32 {
            return None;
        }
        
        let f_min = F_MIN[z as usize];
        let f_max = F_MAX[z as usize];
        let xy_max = XY_MAX[z as usize];
        
        Some((f_min, f_max, 0, xy_max, 0, xy_max))
    }

    fn collect_recursive(
        &self, 
        inner: &Inner<T>, 
        acc: &mut Vec<(SpaceTimeId, T)>, 
        top_z: u8,
        bounds: (i32, i32, u32, u32, u32, u32), // (f_min, f_max, x_min, x_max, y_min, y_max)
        depth: u8
    ) {
        // 深度制限を追加してスタックオーバーフローを防ぐ
        if depth > 32 {
            return;
        }
        
        let (f_min, f_max, x_min, x_max, y_min, y_max) = bounds;
        
        match inner {
            Inner::Value(v) => {
                // 現在の境界からSpaceTimeIdを再構築
                let f_range = if f_min == f_max {
                    DimensionRange::Single(f_min)
                } else {
                    DimensionRange::LimitRange(f_min, f_max)
                };
                
                let x_range = if x_min == x_max {
                    DimensionRange::Single(x_min)
                } else {
                    DimensionRange::LimitRange(x_min, x_max)
                };
                
                let y_range = if y_min == y_max {
                    DimensionRange::Single(y_min)
                } else {
                    DimensionRange::LimitRange(y_min, y_max)
                };

                // SpaceTimeIdを作成（時間は0、tはAnyとして設定）
                if let Ok(id) = SpaceTimeId::new(top_z, f_range, x_range, y_range, 0, DimensionRange::Any) {
                    acc.push((id, v.clone()));
                }
            }
            Inner::Children(children) => {
                // マスクから子ノードの位置を特定し、再帰的に探索
                let mut child_index = 0;
                for mask_bit in 0..8u8 {
                    let bit = 1u8 << mask_bit;
                    if children.mask & bit != 0 {
                        if child_index < children.nodes.len() {
                            // マスクビットから f, x, y の分割を計算
                            let child_bounds = Self::calculate_child_bounds(bounds, mask_bit, top_z);
                            
                            // 無効な境界をチェック
                            let (cf_min, cf_max, cx_min, cx_max, cy_min, cy_max) = child_bounds;
                            if cf_min <= cf_max && cx_min <= cx_max && cy_min <= cy_max {
                                self.collect_recursive(
                                    &children.nodes[child_index].up_inner, 
                                    acc, 
                                    top_z + 1, 
                                    child_bounds,
                                    depth + 1
                                );
                            }
                        }
                        child_index += 1;
                    }
                }
            }
        }
    }

    pub(crate) fn calculate_child_bounds(
        parent_bounds: (i32, i32, u32, u32, u32, u32),
        mask_bit: u8,
        _top_z: u8
    ) -> (i32, i32, u32, u32, u32, u32) {
        let (f_min, f_max, x_min, x_max, y_min, y_max) = parent_bounds;
        
        // マスクビットの解析: bit 2 = f, bit 1 = x, bit 0 = y
        // 0 = 下半分/左半分、1 = 上半分/右半分
        let f_bit = (mask_bit >> 2) & 1;
        let x_bit = (mask_bit >> 1) & 1;
        let y_bit = mask_bit & 1;
        
        // F次元の分割 - 範囲が1より大きい場合のみ分割
        let (new_f_min, new_f_max) = if f_max > f_min {
            let f_mid = f_min + (f_max - f_min) / 2;
            if f_bit == 0 {
                (f_min, f_mid)
            } else {
                (f_mid + 1, f_max)
            }
        } else {
            // 範囲が分割できない場合はそのまま
            (f_min, f_max)
        };
        
        // X次元の分割 - 範囲が1より大きい場合のみ分割
        let (new_x_min, new_x_max) = if x_max > x_min {
            let x_mid = x_min + (x_max - x_min) / 2;
            if x_bit == 0 {
                (x_min, x_mid)
            } else {
                (x_mid + 1, x_max)
            }
        } else {
            // 範囲が分割できない場合はそのまま
            (x_min, x_max)
        };
        
        // Y次元の分割 - 範囲が1より大きい場合のみ分割
        let (new_y_min, new_y_max) = if y_max > y_min {
            let y_mid = y_min + (y_max - y_min) / 2;
            if y_bit == 0 {
                (y_min, y_mid)
            } else {
                (y_mid + 1, y_max)
            }
        } else {
            // 範囲が分割できない場合はそのまま
            (y_min, y_max)
        };
        
        (new_f_min, new_f_max, new_x_min, new_x_max, new_y_min, new_y_max)
    }
}
