use crate::id::DimensionRange::{self, Single};
use crate::{id::SpaceTimeId, map::SpaceTimeIdMap};

#[test]
fn test_simple_map_iter() {
    let id = SpaceTimeId::new(
        2,
        Single(1),
        Single(1),
        Single(1),
        0,
        DimensionRange::Any,
    )
    .unwrap();

    let mut map = SpaceTimeIdMap::new();

    let result = map.insert(id, "test".to_string());
    assert!(result.is_ok());
    
    // Test the iterator
    let items: Vec<_> = map.iter().collect();
    println!("Items: {:?}", items);
    assert_eq!(items.len(), 1);
}