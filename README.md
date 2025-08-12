# Kasane Logic

**Kasane Logic** is a Rust library that extends the 4-dimensional spatial information notation defined by IPA (Information-technology Promotion Agency) and enables logical operations on space-time IDs. The calculations are implemented using pure Rust functions, and operate correctly and efficiently in any environment without external dependencies.

[🇯🇵 日本語版](./README_JA.md)

## 🌱 Features

- Representation of 4-dimensional (X, Y, F, T) space through `SpaceTimeId`
- Flexible description of range specifications and infinite ranges with `DimensionRange`
- Set management and duplicate elimination with `SpaceTimeIdSet`
- Support for union (OR), intersection (AND), complement (NOT), and symmetric difference (XOR) operators
- Lightweight configuration independent of execution environment

## Installation Option

```toml
logic = { path = "../logic", features = ["serde_support"] }
```

Enable the `serde_support` feature to output types compatible with [`serde`](https://crates.io/crates/serde) and [`jsonschema`](https://crates.io/crates/jsonschema).

## 📦 `SpaceTimeId` Type

`SpaceTimeId` represents a region with 4-dimensional space (X, Y, F, T) plus zoom level and time interval.

> [!NOTE]
> A spacetime ID indicates a value where `i` is not 0, while a space ID is defined as having `i = 0` and `t = Any`.

### Space-Time ID

```rust
let stid = SpaceTimeId::new(
    4,                                      // Zoom level
    DimensionRange::AfterUnLimitRange(10),  // Height (f): 10 and above
    DimensionRange::Single(5),              // x coordinate
    DimensionRange::Single(3),              // y coordinate
    60,                                     // Time interval (seconds)
    DimensionRange::LimitRange(100, 200),   // Time index (t)
).unwrap();
```

### Spatial ID

Spatial ID represents objects that are valid for all time.

```rust
let stid = SpaceTimeId::new(
    4,                                      // Zoom level
    DimensionRange::AfterUnLimitRange(10),  // Height (f): 10 and above
    DimensionRange::Single(5),              // x coordinate
    DimensionRange::Single(3),              // y coordinate
    0,                                      // i=0 specifies spatial ID
    DimensionRange::Any,                    // Time index shows Any for all values
).unwrap();
```

## ✍️ Notation

The following extended notation has been introduced:

### Representation of Missing Dimensions

When there are no constraints on a specific dimension (all values are targeted), use `-`.

Example: When F dimension is undefined

```
2/-/1/3
```

This is equivalent to the following set:

```
2/-4/1/3, 2/-3/1/3, ..., 2/3/1/3
```

### Range Specification

Use `:` to indicate intervals:

```
2/1/3/1:5
```

This is equivalent to:

```
2/1/3/1, 2/1/3/2, 2/1/3/3, 2/1/3/4, 2/1/3/5
```

### All Values from a Starting Point

To mean from a starting point to infinity, use `:` and `-` together:

```
2/1/3/1:-
```

This means

```
2/1/3/1, 2/1/3/2, 2/1/3/3, ...
```

## 📐 `DimensionRange<T>` Type

Generic range type representing values for each dimension (X, Y, F, T):

- `Single(v)`: Single value
- `LimitRange(start, end)`: Closed interval from start to end
- `BeforeUnLimitRange(end)`: From the lower limit of values to `end`
- `AfterUnLimitRange(start)`: From `start` to the upper limit of values
- `Any`: Matches all values (entire domain)

---

## 🔧 `SpaceTimeId` Functions and Methods

### 📍 Coordinate Retrieval Functions

#### `coordinates() -> Coordinates`

Converts the SpaceTimeId to geographic coordinates (latitude, longitude, altitude).

```rust
let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3), DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
let coords = stid.coordinates();
println!("Latitude: {:?}, Longitude: {:?}, Altitude: {:?}", coords.latitude, coords.longitude, coords.altitude);
```

#### `center() -> Point`

Returns the center point of the spatial region.

```rust
let center = stid.center();
println!("Center Point - Latitude: {}, Longitude: {}, Altitude: {}", center.latitude, center.longitude, center.altitude);
```

#### `vertex() -> [Point; 8]`

Returns the eight corner vertices of the spatial region.

```rust
let vertices = stid.vertex();
for (i, vertex) in vertices.iter().enumerate() {
    println!("Vertex{}: Latitude={}, Longitude={}, Altitude={}", i, vertex.latitude, vertex.longitude, vertex.altitude);
}
```

### 🔄 Scale Conversion

#### `change_scale(z: Option<u16>, i: Option<u32>) -> Result<SpaceTimeId, String>`

Changes spatial resolution (zoom level) or temporal resolution (time interval).

```rust
// Change zoom level to 6
let scaled = stid.change_scale(Some(6), None)?;

// Change time interval to 30 seconds
let time_scaled = stid.change_scale(None, Some(30))?;
```

### 🎯 Containment Relationship

#### `containment_relation(&other: &SpaceTimeId) -> Containment`

Determines the containment relationship with another SpaceTimeId.

```rust
let containment = stid1.containment_relation(&stid2);
match containment {
    Containment::Full => println!("stid1 completely contains stid2"),
    Containment::Partial(intersection) => println!("Partially overlapping: {}", intersection),
    Containment::None => println!("No overlap"),
}
```

### ✂️ Complement Operation

#### `complement(&self) -> SpaceTimeIdSet`

Returns the complement of this SpaceTimeId (areas not included).

```rust
let complement_set = stid.complement();
println!("Complement: {}", complement_set);
```

### 🔍 Pure ID Expansion

#### `to_pure(&self) -> Vec<SpaceTimeId>`

Expands all range notations (Any, LimitRange, BeforeUnLimitRange, AfterUnLimitRange) in the spatial dimensions (F, X, Y) into individual SpaceTimeIds with only Single values. The time dimension (T) is preserved as-is.

This function is useful for converting complex range-based SpaceTimeIds into concrete, enumerated SpaceTimeIds for precise processing.

```rust
// Create a SpaceTimeId with range dimensions
let stid = SpaceTimeId::new(
    2,                                    // Zoom level 2
    DimensionRange::LimitRange(0, 1),     // F dimension: 0 to 1
    DimensionRange::LimitRange(1, 2),     // X dimension: 1 to 2  
    DimensionRange::Single(0),            // Y dimension: single value 0
    60,                                   // Time interval
    DimensionRange::Single(100),          // T dimension: single value 100
).unwrap();

// Expand to pure IDs
let pure_ids = stid.to_pure();
println!("Expanded to {} pure IDs", pure_ids.len()); // Will be 4 IDs (2 F values × 2 X values × 1 Y value)

// Each pure ID will have only Single values for F, X, Y dimensions
for pure_id in pure_ids {
    println!("{}", pure_id); // e.g., "2/0/1/0_60/100", "2/0/2/0_60/100", etc.
}
```

### 📊 Value Getter Methods

Getter methods for accessing values and attributes of each dimension:

- `f() -> DimensionRange<i64>`: F dimension (altitude) value
- `x() -> DimensionRange<u64>`: X dimension value
- `y() -> DimensionRange<u64>`: Y dimension value
- `t() -> DimensionRange<u32>`: T dimension (time index) value
- `z() -> u16`: Zoom level
- `i() -> u32`: Time interval (seconds)

---

## 🏗️ Supporting Types

### `Point` Structure

Represents a point in 3-dimensional space.

```rust
pub struct Point {
    pub latitude: f64,   // Latitude
    pub longitude: f64,  // Longitude
    pub altitude: f64,   // Altitude
}
```

### `Coordinates` Structure

Represents the coordinate range of a spatial region.

```rust
pub struct Coordinates {
    pub latitude: (f64, f64),   // Latitude range (min, max)
    pub longitude: (f64, f64),  // Longitude range (min, max)
    pub altitude: (f64, f64),   // Altitude range (min, max)
}
```

### `Containment` Enumeration

Represents the type of containment relationship.

```rust
pub enum Containment {
    Full,                    // Complete containment
    Partial(SpaceTimeId),   // Partial overlap (includes intersection area)
    None,                   // No overlap
}
```

---

## 📚 `SpaceTimeIdSet` Type

Manages multiple `SpaceTimeId` as a set. When adding new elements, any overlapping ranges with existing elements are automatically adjusted, ensuring that only unique IDs for the physical space remain.

### 🧭 Basic Usage

```rust
let mut set = SpaceTimeIdSet::new();
set.insert(stid);
```

### 🛠️ `SpaceTimeIdSet` Methods

#### `new() -> SpaceTimeIdSet`

Creates a new empty set.

```rust
let mut set = SpaceTimeIdSet::new();
```

#### `insert(&mut self, other: SpaceTimeId)`

Adds a SpaceTimeId to the set. Automatically adjusts for duplicates or overlaps.

```rust
set.insert(stid);
```

#### `iter() -> impl Iterator<Item = &SpaceTimeId>`

Returns an iterator for processing elements in the set.

```rust
for id in set.iter() {
    println!("{}", id);
}
```

#### `is_empty() -> bool`

Checks if the set is empty.

```rust
if set.is_empty() {
    println!("Set is empty");
}
```

#### `from(id: SpaceTimeId) -> SpaceTimeIdSet`

Creates a set from a single SpaceTimeId.

```rust
let set = SpaceTimeIdSet::from(stid);
```

### 🔀 Supported Operators

- `|`: Union - Combines two sets
- `&`: Intersection - Extracts common parts
- `^`: Exclusive OR (XOR) - Returns regions in either set but not in both
- `!`: Complement - Returns areas not included
- `==`: Equality comparison - Returns true if they represent the same spatial region

```rust
let union = set_a | set_b;
let intersection = set_a & set_b;
let xor = set_a ^ set_b;
let complement = !set_a;
assert_eq!(set_a, set_b); // true if physically equivalent
```

## 🧪 Usage Examples

```rust
let a = SpaceTimeId::new(...).unwrap();
let b = SpaceTimeId::new(...).unwrap();

let mut set = SpaceTimeIdSet::new();
set.insert(a);

let set2 = SpaceTimeIdSet::from(b);
let union = set | set2;
let common = set & set2;
let outside = !set;
```

## 📋 Complete API Reference

### `SpaceTimeId` Constructor

- `new(z: u16, x: DimensionRange<u64>, y: DimensionRange<u64>, f: DimensionRange<i64>, i: u32, t: DimensionRange<u32>) -> Result<SpaceTimeId, String>`

### `SpaceTimeId` Instance Methods

- `coordinates() -> Coordinates` - Get geographic coordinates
- `center() -> Point` - Get center point of spatial region
- `vertex() -> [Point; 8]` - Get eight corner vertices
- `change_scale(z: Option<u16>, i: Option<u32>) -> Result<SpaceTimeId, String>` - Change resolution
- `containment_relation(&other: &SpaceTimeId) -> Containment` - Check containment relationship
- `complement() -> SpaceTimeIdSet` - Get complement set
- `to_pure() -> Vec<SpaceTimeId>` - Expand range dimensions to individual SpaceTimeIds
- `f() -> DimensionRange<i64>` - Get F dimension value
- `x() -> DimensionRange<u64>` - Get X dimension value
- `y() -> DimensionRange<u64>` - Get Y dimension value
- `t() -> DimensionRange<u32>` - Get T dimension value
- `z() -> u16` - Get zoom level
- `i() -> u32` - Get time interval

### `SpaceTimeIdSet` Methods

- `new() -> SpaceTimeIdSet` - Create new empty set
- `from(id: SpaceTimeId) -> SpaceTimeIdSet` - Create set from single ID
- `insert(&mut self, other: SpaceTimeId)` - Add ID to set
- `iter() -> impl Iterator<Item = &SpaceTimeId>` - Get iterator
- `is_empty() -> bool` - Check if set is empty

### `SpaceTimeIdSet` Operators

- `set1 | set2` - Union operation
- `set1 & set2` - Intersection operation
- `set1 ^ set2` - XOR (exclusive OR) operation
- `!set` - Complement operation
- `set1 == set2` - Equality comparison

## 🤝 Contributing

We welcome all contributions including bug reports, feature suggestions, documentation fixes, and test additions.

## 🧪 Testing Philosophy

This library prioritizes accuracy as its primary goal. Therefore, we maintain a comprehensive testing strategy:

- Tests are extensively developed to ensure maximum coverage
- When code improvements result in behavior changes, we discuss whether it's a bug or an intended modification
- Comprehensive testing enables detection of behavioral changes
- Tests can be executed with `cargo test`

## ⚡ Performance Testing Philosophy

Performance benchmarks are designed to optimize function performance:

- Benchmarks can be executed with `cargo bench`
- Performance improvements are continuously pursued for all functions
- Testing is conducted using the Criterion framework
- Coverage for all functions is still in development
