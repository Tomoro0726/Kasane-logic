use kasane_logic::{
    function::{
        line::line,
        tools::{
            ECEF,
            ecef_to_point::ecef_to_point,
            point_to_ecef::{self},
        },
        triangle::triangle,
    },
    id::{DimensionRange, SpaceTimeId, coordinates::Point},
    set::SpaceTimeIdSet,
};
use std::io::{BufWriter, Write};
use std::{fs::File, time::Instant};

fn main() {
    let a = Point {
        latitude: 35.6809591,
        longitude: 139.7673068,
        altitude: 1000.0,
    };

    let b = Point {
        latitude: 33.6291112,
        longitude: 138.7389313,
        altitude: 100.0,
    };

    let c = Point {
        latitude: 35.2291112,
        longitude: 139.7089313,
        altitude: 10000.0,
    };

    let result = triangle(30, a, b, c);
}
