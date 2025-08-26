use itertools::iproduct;
use logic::id::{DimensionRange, SpaceTimeId};
use crate::benchmark_utils::core::{MAX_ROW, ZOOM_LEVEL};

pub fn generate_all_single_stids() -> Vec<SpaceTimeId> {
  let mut all_stids = Vec::new();
        //下記で、直積集合にしてloopを回す
        //すべてのボクセル
        // for (f,x,y) in iproduct!(-MAX_ROW -1..=MAX_ROW, 0..=MAX_ROW as u64, 0..=MAX_ROW as u64) {
        //f が自然数の物のみ
        for (f,x,y) in iproduct!(0..=MAX_ROW, 0..=MAX_ROW as u64, 0..=MAX_ROW as u64) {
            let stid = SpaceTimeId::new(
                ZOOM_LEVEL as u16,
                DimensionRange::Single(f),
                DimensionRange::Single(x),
                DimensionRange::Single(y),
                1,
                DimensionRange::Single(0),
            ).unwrap();

            all_stids.push(stid);

        }
        all_stids
    }