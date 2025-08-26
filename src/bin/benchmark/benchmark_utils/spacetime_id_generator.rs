use crate::benchmark_utils::core::{MAX_ROW, ZOOM_LEVEL};
use itertools::iproduct;
use logic::id::{DimensionRange, SpaceTimeId};

pub fn generate_all_single_stids() -> Vec<SpaceTimeId> {
    let mut all_stids = Vec::new();
    //下記で、直積集合にしてloopを回す
    //f が自然数の物のみ
    for (f, x, y) in iproduct!(0..=MAX_ROW, 0..=MAX_ROW as u64, 0..=MAX_ROW as u64) {
        let stid = SpaceTimeId::new(
            ZOOM_LEVEL as u16,
            DimensionRange::Single(f),
            DimensionRange::Single(x),
            DimensionRange::Single(y),
            1,
            DimensionRange::Single(0),
        )
        .unwrap();

        all_stids.push(stid);
    }
    all_stids
}

pub fn generate_all_range_stids() -> Vec<SpaceTimeId> {
    let mut all_range_stids = generate_all_single_stids();
    for (f_end, x_end, y_end) in iproduct!(0..=MAX_ROW, 0..=MAX_ROW as u64, 0..=MAX_ROW as u64) {
        for (f_start, x_start, y_start) in iproduct!(0..=f_end, 0..=x_end, 0..=y_end) {
            let stid = SpaceTimeId::new(
                ZOOM_LEVEL as u16,
                DimensionRange::LimitRange(f_start, f_end),
                DimensionRange::LimitRange(x_start, x_end),
                DimensionRange::LimitRange(y_start, y_end),
                1,
                DimensionRange::Single(0),
            )
            .unwrap();
            all_range_stids.push(stid);
        }
    }
    all_range_stids
}