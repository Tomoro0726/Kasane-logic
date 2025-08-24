use std::collections::HashSet;

use crate::{
    function::tools::{
        ECEF, ecef_to_point::ecef_to_point, point_to_ecef::point_to_ecef, point_to_id::point_to_id,
    },
    id::{SpaceTimeId, coordinates::Point},
    set::SpaceTimeIdSet,
};

pub fn triangle(z: u16, a: Point, b: Point, c: Point) -> SpaceTimeIdSet {
    let steps = 1000;
    let mut voxels_set = SpaceTimeIdSet::new();

    // Point → ECEF
    let ea = point_to_ecef(a);
    let eb = point_to_ecef(b);
    let ec = point_to_ecef(c);

    for i in 0..=steps {
        let t = i as f64 / steps as f64;

        // 辺 a-b, a-c を補間
        let line1 = ECEF {
            x: ea.x * (1.0 - t) + eb.x * t,
            y: ea.y * (1.0 - t) + eb.y * t,
            z: ea.z * (1.0 - t) + eb.z * t,
        };
        let line2 = ECEF {
            x: ea.x * (1.0 - t) + ec.x * t,
            y: ea.y * (1.0 - t) + ec.y * t,
            z: ea.z * (1.0 - t) + ec.z * t,
        };

        for j in 0..=i {
            if i == 0 {
                // 始点 a のみ
                let p = ecef_to_point(ea);
                let voxel = point_to_id(z, p);
                voxels_set.insert(voxel);
            } else {
                let s = j as f64 / i as f64;

                // line1 と line2 を補間して内部点を得る
                let e = ECEF {
                    x: line1.x * (1.0 - s) + line2.x * s,
                    y: line1.y * (1.0 - s) + line2.y * s,
                    z: line1.z * (1.0 - s) + line2.z * s,
                };

                // ECEF → Point → Voxel
                let p = ecef_to_point(e);
                let voxel = point_to_id(z, p);

                voxels_set.insert(voxel);
            }
        }
    }

    voxels_set
}
