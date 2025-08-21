use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    time::{Instant},
};

use itertools::iproduct;
use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};

pub fn benchmark_and(zoom_level: &i64) {

    let mut zoom_level_bench = vec![0; *zoom_level as usize + 1];
    for z in 0..=*zoom_level {
        let max_row:i64 = 2_i64.pow(z as u32);
        //下記で、直積集合にしてloopを回す
        for (f1,x1,y1,f2,x2,y2) in iproduct!(0..=max_row, 0..=max_row as u64, 0..=max_row as u64, 0..=max_row, 0..=max_row as u64, 0..=max_row as u64) {
            //setを作る
            let mut set_a = SpaceTimeIdSet::new();
            let mut set_b = SpaceTimeIdSet::new();
            let stid_a = SpaceTimeId::new(
                *zoom_level as u16,
                DimensionRange::Single(f1),
                DimensionRange::Single(x1),
                DimensionRange::Single(y1),
                0,
                DimensionRange::Single(0),
            ).unwrap();
            let stid_b = SpaceTimeId::new(
                *zoom_level as u16,
                DimensionRange::Single(f2),
                DimensionRange::Single(x2),
                DimensionRange::Single(y2),
                0,
                DimensionRange::Single(0),
            ).unwrap();

            set_a.insert(stid_a);
            set_b.insert(stid_b);

            //計測開始
            let start = Instant::now();
            let _intersection = &set_a & &set_b;
            let elapsed = start.elapsed();

            zoom_level_bench[z as usize] += elapsed.as_nanos() as i32;
            //最後に全部足して、合計のかかった時間を出す。
        }

        // let total_time: i32 = &zoom_level_bench.iter().sum();
        println!("Zoom Level {}: Total Time = {} ns", z, zoom_level_bench[z as usize]);
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Markdownファイルに追記
        // let path = Path::new("Benchmark_history.md");
        // let mut file = OpenOptions::new()
        //     .create(true)
        //     .append(true)
        //     .open(&path)
        //     .expect("Failed to open Benchmark_history.md");

        // if path.metadata().unwrap().len() == 0 {
        //     writeln!(file, "# Benchmark History for `SpaceTimeIdSet &`\n").unwrap();
        //     writeln!(file, "| DateTime | Function | Time (ms) |").unwrap();
        //     writeln!(file, "|----------|----------|-----------|").unwrap();
        // }

        // writeln!(file, "| {} | Intersection | {:.3} |", now, ms).unwrap();

        // println!("# Benchmark Results");
        // println!("| DateTime | Function | Time (ms) |");
        // println!("|----------|----------|-----------|");
        // println!("| {} | Intersection | {:.3} |", now, ms);
    }
}

