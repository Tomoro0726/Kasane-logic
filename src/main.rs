use std::any::Any;

use kasane_logic::{
    function::{
        ecef::{
            ECEF,
            ecef_to_point::ecef_to_point,
            point_to_ecef::{self},
        },
        line::{self},
    },
    id::{DimensionRange, SpaceTimeId, coordinates::Point},
    set::SpaceTimeIdSet,
};

fn main() {
    let test = ECEF {
        x: 11111111.0,
        y: 11111111.0,
        z: 11111111.0,
    };

    println!("{:?}", ecef_to_point(test));
}
