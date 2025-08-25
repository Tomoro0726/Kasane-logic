use itertools::iproduct;
use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};

use crate::benchmark_utils::{measure_benchmark, write_markdown};

///`benchmark` function make arguments for the benchmark test
/// and,or,not,xor,eq are all supported operations
/// insert_test is supported by other function `benchmark_insert`
//Rは複数の型を許容するため
pub fn benchmark<F, R>(calculate: F, name: &str) 
where 
    F: Fn(&SpaceTimeIdSet,&SpaceTimeIdSet) -> R,
{
    let zoom_level = 1;
    let mut total_benchmark_time = 0;
    let max_row:i64 = 2_i64.pow(zoom_level as u32) - 1;

    let mut all_stids = Vec::new();
    //下記で、直積集合にしてloopを回す
    //すべてのボクセル
    // for (f,x,y) in iproduct!(-max_row -1..=max_row, 0..=max_row as u64, 0..=max_row as u64) {
    //f が自然数の物のみ
    for (f,x,y) in iproduct!(0..=max_row, 0..=max_row as u64, 0..=max_row as u64) {
        let stid = SpaceTimeId::new(
            zoom_level as u16,
            DimensionRange::Single(f),
            DimensionRange::Single(x),
            DimensionRange::Single(y),
            1,
            DimensionRange::Single(0),
        ).unwrap();

        all_stids.push(stid);

    }

    //ビットマスクで全組み合わせを生成
    let total_voxel_count = all_stids.len();
    if total_voxel_count > 16 {
        panic!("Too many voxels for benchmark.");
    }    
    for (mask_a,mask_b) in iproduct!(0..(1u16 << total_voxel_count), 0..(1u16 << total_voxel_count)) {
        let mut subset_set_a = SpaceTimeIdSet::new();
        let mut subset_set_b = SpaceTimeIdSet::new();
        for (i, stid) in all_stids.iter().enumerate() {
            if (mask_a >> i) & 1 == 1 {
                subset_set_a.insert(*stid);
            }
            if (mask_b >> i) & 1 == 1 {
                subset_set_b.insert(*stid);
            }
        }
        // 計測
        total_benchmark_time += measure_benchmark(&calculate, &subset_set_a, &subset_set_b);
    }
    // Markdownファイルに追記
    write_markdown(name, total_benchmark_time as f64);
}

pub fn benchmark_insert() {

}

pub fn benchmark_not(){

}