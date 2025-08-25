
use itertools::iproduct;
use logic::{set::SpaceTimeIdSet};

use crate::benchmark_utils::{generate_all_stids, measure::measure_benchmark_insert, measure_benchmark, write_markdown};

pub const ZOOM_LEVEL:i32 = 1;
pub const MAX_ROW:i64 = 2_i64.pow(ZOOM_LEVEL as u32) - 1;

///`benchmark` function make arguments for the benchmark test
/// and,or,not,xor,eq are all supported operations
/// insert_test is supported by other function `benchmark_insert`
//Rは複数の型を許容するため
pub fn benchmark_main<F, R>(calculate: F, name: &str, iterations: usize) 
where 
    F: Fn(&SpaceTimeIdSet,&SpaceTimeIdSet) -> R,
{
    let mut total_benchmark_time = 0;
    let all_stids = generate_all_stids();

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
        total_benchmark_time += measure_benchmark(&calculate,iterations, &subset_set_a, &subset_set_b);
    }
    // Markdownファイルに追記
    write_markdown(name, total_benchmark_time as f64);
}

pub fn benchmark_insert(iterations: usize) {
    let name = "insert";
    let mut total_benchmark_time = 0;
    let all_stids = generate_all_stids();
    let total_voxel_count = all_stids.len();
    if total_voxel_count > 16 {
        panic!("Too many voxels for benchmark.");
    }
    for (mask_a, stid) in iproduct!(0..(1u16 << total_voxel_count), all_stids.iter()) {
        let mut subset_set_a = SpaceTimeIdSet::new();
        for (i, stid_to_add) in all_stids.iter().enumerate() {
            if (mask_a >> i) & 1 == 1 {
                subset_set_a.insert(*stid_to_add);
            }
        }
        // 計測
        total_benchmark_time += measure_benchmark_insert(iterations, &mut subset_set_a, stid);
    }
    write_markdown(name, total_benchmark_time as f64);
}

pub fn benchmark_not(){
    
}
