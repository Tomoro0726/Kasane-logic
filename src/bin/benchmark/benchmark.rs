use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    time::{Instant},
};

use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};

#[test]
fn benchmark() {
    let zoom_level :i64 = 2;

    //ここにそれぞれの関数のベンチマークを計測する関数を書いていく。
    benchmark_and(&zoom_level);

    
}



