use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    time::{Instant},
};

use itertools::iproduct;
use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};

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
    for (f,x,y) in iproduct!(-max_row -1..=max_row, 0..=max_row as u64, 0..=max_row as u64) {
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

    //u64のビットマスクを利用しているため、ボクセル数が32を超えるとうまくいかない
    let total_voxel_count = all_stids.len();
    if total_voxel_count > 16 {
        panic!("Too many voxels for benchmark.");
    }
    for mask in 0..(1u32 << total_voxel_count) {
        
        let mut subset_set_a = SpaceTimeIdSet::new();
        let mut subset_set_b = SpaceTimeIdSet::new();
        for (i, stid) in all_stids.iter().enumerate() {
            //下位のビットがset_a、上位のビットがset_b
            if (mask >> i) & 1 == 1 {
                subset_set_a.insert(*stid);
            }
            if (mask >> (i + total_voxel_count)) & 1 == 1 {
                subset_set_b.insert(*stid);
            }
        }

        // 計測
        let start = Instant::now();
        let _result = calculate(&subset_set_a, &subset_set_b);
        let elapsed = start.elapsed();
        total_benchmark_time += elapsed.as_nanos() as i64;
    }
    // Markdownファイルに追記
    write_markdown(name, total_benchmark_time as f64);
}

pub fn benchmark_insert(set1: &mut SpaceTimeIdSet, set2: &SpaceTimeIdSet, name: &str) {

}

fn write_markdown(name: &str,ns: f64) {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let path_str = format!("benchmark_history/{}.md", name);
    let path = Path::new(&path_str);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    if path.metadata().unwrap().len() == 0 {
        writeln!(file, "# Benchmark History for `SpaceTimeIdSet &`\n").unwrap();
        writeln!(file, "| DateTime | Time (ns) |").unwrap();
        writeln!(file, "|----------|----------|").unwrap();
    }

    writeln!(file, "| {} | {:.3} |", now, ns).unwrap();

    println!("# Benchmark Results {}", name);
    println!("| DateTime | Time (ns) |");
    println!("|----------|----------|");
    println!("| {} | {:.3} |", now, ns);
}