use crate::{
    function::tools::point_to_id::point_to_id,
    id::{SpaceTimeId, coordinates::Point},
};
use std::collections::HashSet;

/// a と b の間の voxel 線分を返す
pub fn line(z: u16, a: Point, b: Point) -> HashSet<SpaceTimeId> {
    let steps = 50_000;
    let mut voxels_set = HashSet::new();

    for i in 0..=steps {
        let t = i as f64 / steps as f64;

        let lat = a.latitude * (1.0 - t) + b.latitude * t;
        let lon = a.longitude * (1.0 - t) + b.longitude * t;
        let alt = a.altitude * (1.0 - t) + b.altitude * t;

        let point = Point {
            latitude: lat,
            longitude: lon,
            altitude: alt,
        };
        let voxel = point_to_id(z, point);

        voxels_set.insert(voxel);
    }

    voxels_set
}
