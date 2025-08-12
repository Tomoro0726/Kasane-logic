use crate::id::SpaceTimeId;

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

impl SpaceTimeId {
    /// Returns the center point of the spatial volume represented by this `SpaceTimeId`.
    ///
    /// The center is the midpoint of the latitude, longitude, and altitude ranges
    /// obtained from the `coordinates()` method.
    pub fn center(&self) -> Point {
        let coordinates = self.coordinates();

        Point {
            latitude: (coordinates.latitude.0 + coordinates.latitude.1) / 2.0,
            longitude: (coordinates.longitude.0 + coordinates.longitude.1) / 2.0,
            altitude: (coordinates.altitude.0 + coordinates.altitude.1) / 2.0,
        }
    }

    /// Returns the eight corner vertices of the spatial volume represented by this `SpaceTimeId`.
    ///
    /// The vertices represent each corner of the cuboid volume defined by the minimum and maximum
    /// values of latitude, longitude, and altitude ranges obtained from the `coordinates()` method.
    pub fn vertex(&self) -> [Point; 8] {
        let coordinates = self.coordinates();

        let lat0 = coordinates.latitude.0;
        let lat1 = coordinates.latitude.1;
        let lng0 = coordinates.longitude.0;
        let lng1 = coordinates.longitude.1;
        let alt0 = coordinates.altitude.0;
        let alt1 = coordinates.altitude.1;

        [
            Point {
                latitude: lat0,
                longitude: lng0,
                altitude: alt0,
            },
            Point {
                latitude: lat0,
                longitude: lng0,
                altitude: alt1,
            },
            Point {
                latitude: lat0,
                longitude: lng1,
                altitude: alt0,
            },
            Point {
                latitude: lat0,
                longitude: lng1,
                altitude: alt1,
            },
            Point {
                latitude: lat1,
                longitude: lng0,
                altitude: alt0,
            },
            Point {
                latitude: lat1,
                longitude: lng0,
                altitude: alt1,
            },
            Point {
                latitude: lat1,
                longitude: lng1,
                altitude: alt0,
            },
            Point {
                latitude: lat1,
                longitude: lng1,
                altitude: alt1,
            },
        ]
    }
}
