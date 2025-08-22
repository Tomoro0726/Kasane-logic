use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    time::{Instant},
};

use itertools::iproduct;
use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};

pub fn benchmark<F: Fn(&SpaceTimeIdSet,&SpaceTimeIdSet) -> SpaceTimeIdSet>(zoom_level: &i64, calculate: F, name: &str) {

    let mut zoom_level_bench = vec![0; *zoom_level as usize + 1];
    for z in 0..=*zoom_level {
        let max_row:i64 = 2_i64.pow(z as u32) - 1;
        //下記で、直積集合にしてloopを回す
        for (f1,x1,y1,f2,x2,y2) in iproduct!(-max_row..=max_row, 0..=max_row as u64, 0..=max_row as u64, -max_row..=max_row, 0..=max_row as u64, 0..=max_row as u64) {
            //setを作る
            let mut set_a = SpaceTimeIdSet::new();
            let mut set_b = SpaceTimeIdSet::new();
            let stid_a = SpaceTimeId::new(
                *zoom_level as u16,
                DimensionRange::Single(f1),
                DimensionRange::Single(x1),
                DimensionRange::Single(y1),
                1,
                DimensionRange::Single(0),
            ).unwrap();
            let stid_b = SpaceTimeId::new(
                *zoom_level as u16,
                DimensionRange::Single(f2),
                DimensionRange::Single(x2),
                DimensionRange::Single(y2),
                1,
                DimensionRange::Single(0),
            ).unwrap();

            set_a.insert(stid_a);
            set_b.insert(stid_b);

            //計測開始
            let start = Instant::now();
            let _intersection = calculate(&set_a, &set_b);
            let elapsed = start.elapsed();

            zoom_level_bench[z as usize] += elapsed.as_nanos() as i64;
            
        }
        println!("Zoom Level {}: Total Time = {} ns", z, zoom_level_bench[z as usize]);
        
    }

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    //最後に全部足して、合計のかかった時間を出す。
    let ns = zoom_level_bench.iter().sum::<i64>() as f64;

    // Markdownファイルに追記
    let path = Path::new("Benchmark_history.md");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open Benchmark_history.md");

    if path.metadata().unwrap().len() == 0 {
        writeln!(file, "# Benchmark History for `SpaceTimeIdSet &`\n").unwrap();
        writeln!(file, "| DateTime | Function | Time (ns) |").unwrap();
        writeln!(file, "|----------|----------|-----------|").unwrap();
    }

    writeln!(file, "| {} | {} | {:.3} |", now, name, ns).unwrap();

    println!("# Benchmark Results");
    println!("| DateTime | Function | Time (ns) |");
    println!("|----------|----------|-----------|");
    println!("| {} | {} | {:.3} |", now, name, ns);
}

