use crate::{
    function::tools::{
        ecef_to_point::ecef_to_point, point_to_ecef::point_to_ecef, point_to_id::point_to_id, ECEF,
    },
    id::coordinates::Point,
    set::SpaceTimeIdSet,
};

/// 並列版 (ネイティブ: full feature)
#[cfg(feature = "full")]
pub fn triangle(z: u8, a: Point, b: Point, c: Point) -> SpaceTimeIdSet {
    println!("Triangle Start");
    let start = Instant::now();

    use rayon::prelude::*;
    use std::{collections::HashSet, time::Instant};

    let steps = 1000;
    let ea = point_to_ecef(a);
    let eb = point_to_ecef(b);
    let ec = point_to_ecef(c);

    let voxels_set: HashSet<_> = (0..=steps)
        .into_par_iter()
        .fold(HashSet::new, |mut local_set, i| {
            if i == 0 {
                // 始点 a のみ
                let p = ecef_to_point(ea);
                let voxel = point_to_id(z, p);
                local_set.insert(voxel);
            } else {
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

                    local_set.insert(voxel);
                }
            }
            local_set
        })
        .reduce(HashSet::new, |mut acc, local_set| {
            acc.extend(local_set);
            acc
        });

    let result;
    let end = start.elapsed();

    println!("三角形の計算にかかった時間:{:?}ms", end.as_millis());

    unsafe { result = SpaceTimeIdSet::from_hash(voxels_set) };

    result
}

/// 逐次版 (WASM: wasm feature)
#[cfg(feature = "wasm")]
pub fn triangle(z: u8, a: Point, b: Point, c: Point) -> SpaceTimeIdSet {
    use std::{collections::HashSet, time::Instant};
    println!("Triangle Start");
    let start = Instant::now();
    let steps = 1000;
    let mut voxels_set = HashSet::new();

    // Point → ECEF
    let ea = point_to_ecef(a);
    let eb = point_to_ecef(b);
    let ec = point_to_ecef(c);

    for i in 0..=steps {
        if i == 0 {
            let p = ecef_to_point(ea);
            let voxel = point_to_id(z, p);
            voxels_set.insert(voxel);
        } else {
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
    let end = start.elapsed();

    println!("三角形の計算にかかった時間:{:?}ms", end.as_millis());
    println!("時空間IDの数{}", voxels_set.iter().len());
    let start = Instant::now();

    let mut result = SpaceTimeIdSet::new();

    for ele in voxels_set {
        result.insert(ele);
    }

    let end = start.elapsed();
    println!("Insertにかけた時間:{:?}ms", end.as_millis());

    println!("短縮後時空間IDの数{}", result.clone().into_iter().len());

    result
}
