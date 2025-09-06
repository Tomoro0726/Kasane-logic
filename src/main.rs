use kasane_logic::id::DimensionRange::{self, Single};
use kasane_logic::{id::SpaceTimeId, map::SpaceTimeIdMap};

fn main() {
    let id = SpaceTimeId::new(10, Single(3), Single(1), Single(1), 0, DimensionRange::Any).unwrap();

    let mut map = SpaceTimeIdMap::new();

    let _ = map.insert(id, "猫".to_string());

    println!("{:?}", map);
}
