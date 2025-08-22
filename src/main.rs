use kasane_logic::{
    function::{
        ecef::point_to_ecef::{self, point_to_ecef},
        line::{self},
    },
    id::coordinates::Point,
};

fn main() {
    // let a = Point {
    //     latitude: todo!(),
    //     longitude: todo!(),
    //     altitude: todo!(),
    // };
    // let b = Point {
    //     latitude: todo!(),
    //     longitude: todo!(),
    //     altitude: todo!(),
    // };

    // let result = line(a, b);

    // println!("{}", result);

    let point = Point {
        latitude: 139.4,
        longitude: 135.4,
        altitude: 100.0,
    };

    println!("{:?}", point_to_ecef(point));
}
