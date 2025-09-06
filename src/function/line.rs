use crate::{
    function::tools::{
        ECEF, ecef_to_point::ecef_to_point, point_to_ecef::point_to_ecef, point_to_id::point_to_id,
    },
    id::{SpaceTimeId, coordinates::Point},
    set::SpaceTimeIdSet,
};
use std::collections::HashSet;

/// a と b の間の voxel 線分を返す
pub fn line(z: u8, a: Point, b: Point) -> SpaceTimeIdSet {
    let steps = 50_000;
    let mut voxels_set = SpaceTimeIdSet::new();

    // Point → ECEF
    let ea = point_to_ecef(a);
    let eb = point_to_ecef(b);

    for i in 0..=steps {
        let t = i as f64 / steps as f64;

        // ECEF補間
        let e = ECEF {
            x: ea.x * (1.0 - t) + eb.x * t,
            y: ea.y * (1.0 - t) + eb.y * t,
            z: ea.z * (1.0 - t) + eb.z * t,
        };

        // ECEF → Point
        let p = ecef_to_point(e);

        // Point → Voxel
        let voxel = point_to_id(z, p);

        voxels_set.insert(voxel);
    }

    voxels_set
}
