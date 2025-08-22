use std::any::Any;

use kasane_logic::{
    function::{
        ecef::point_to_ecef::{self, point_to_ecef},
        line::{self},
    },
    id::{DimensionRange, SpaceTimeId, coordinates::Point},
    set::SpaceTimeIdSet,
};

fn main() {
    let id = SpaceTimeId::new(
        4,
        DimensionRange::Single(3),
        DimensionRange::Single(2),
        DimensionRange::LimitRange(1, 2),
        3,
        DimensionRange::LimitRange(1, 2),
    )
    .unwrap();

    let id2 = SpaceTimeId::new(
        4,
        DimensionRange::Single(3),
        DimensionRange::Single(2),
        DimensionRange::LimitRange(1, 2),
        3,
        DimensionRange::LimitRange(3, 4),
    )
    .unwrap();

    let mut set1 = SpaceTimeIdSet::from(id);

    set1.insert(id2);

    println!("{}", set1);
}
