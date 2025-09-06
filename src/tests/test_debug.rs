use crate::id::DimensionRange::{self, Single};
use crate::{id::SpaceTimeId, map::SpaceTimeIdMap};

#[test]
fn test_debug_octree_structure() {
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
    
    println!("Map structure: {:?}", map);
    
    // Test the iterator with limited items
    let items: Vec<_> = map.iter().take(5).collect();
    println!("First 5 items: {:?}", items);
    assert!(items.len() <= 5);
}

#[test]
fn test_debug_bounds_calculation() {
    // Test the edge case bounds calculation directly
    use crate::map::SpaceTimeIdMap;
    
    let bounds = (1, 1, 1, 1, 1, 1); // All single-value bounds
    
    for mask_bit in 0..8u8 {
        let child_bounds = SpaceTimeIdMap::<String>::calculate_child_bounds(bounds, mask_bit, 2);
        println!("Mask bit {}: {:?} -> {:?}", mask_bit, bounds, child_bounds);
        
        // Check if bounds are valid
        let (f_min, f_max, x_min, x_max, y_min, y_max) = child_bounds;
        assert!(f_min <= f_max, "Invalid f bounds: {} > {}", f_min, f_max);
        assert!(x_min <= x_max, "Invalid x bounds: {} > {}", x_min, x_max);
        assert!(y_min <= y_max, "Invalid y bounds: {} > {}", y_min, y_max);
    }
}