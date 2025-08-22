use crate::{function::ecef::ECEF, id::coordinates::Point};

pub fn ecef_to_point(ecef: ECEF) -> Point {
    // WGS84 定数
    let a = 6378137.0_f64; // 長半径
    let inv_f = 298.257223563_f64;
    let f = 1.0 / inv_f;
    let b = a * (1.0 - f);
    let e2 = 1.0 - (b * b) / (a * a);
    let ep2 = (a * a - b * b) / (b * b);

    let x = ecef.x;
    let y = ecef.y;
    let z = ecef.z;

    let p = (x * x + y * y).sqrt();
    let theta = (z * a).atan2(p * b);

    let sin_theta = theta.sin();
    let cos_theta = theta.cos();

    let lat = (z + ep2 * b * sin_theta.powi(3)).atan2(p - e2 * a * cos_theta.powi(3));
    let lon = y.atan2(x);

    let sin_lat = lat.sin();
    let n = a / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    let h = p / lat.cos() - n;

    Point {
        latitude: lat.to_degrees(),
        longitude: lon.to_degrees(),
        altitude: h,
    }
}
