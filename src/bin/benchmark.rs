use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    time::{Instant},
};

use logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};

fn main() {
    let mut set_a = SpaceTimeIdSet::new();
    let mut set_b = SpaceTimeIdSet::new();

    let i = 1;
    let stid_a = SpaceTimeId::new(
        4,
        DimensionRange::Single(i),
        DimensionRange::Single((i + 1) as u64),
        DimensionRange::Single((i + 2) as u64),
        60,
        DimensionRange::Single((i + 3) as u32),
    ).unwrap();

    let stid_b = SpaceTimeId::new(
        4,
        DimensionRange::Single(i),
        DimensionRange::Single((i + 1) as u64),
        DimensionRange::Single((i + 2) as u64),
        60,
        DimensionRange::Single((i + 3) as u32),
    ).unwrap();
    set_a.insert(stid_a);
    set_b.insert(stid_b);

    // 計測
    let start = Instant::now();
    let _intersection = &set_a & &set_b;
    let elapsed = start.elapsed();

    let ms = elapsed.as_secs_f64() * 1000.0;

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Markdownファイルに追記
    let path = Path::new("Benchmark_history.md");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("Failed to open Benchmark_history.md");

    if path.metadata().unwrap().len() == 0 {
        writeln!(file, "# Benchmark History for `SpaceTimeIdSet &`\n").unwrap();
        writeln!(file, "| DateTime | Function | Time (ms) |").unwrap();
        writeln!(file, "|----------|----------|-----------|").unwrap();
    }

    writeln!(file, "| {} | Intersection | {:.3} |", now, ms).unwrap();

    println!("# Benchmark Results");
    println!("| DateTime | Function | Time (ms) |");
    println!("|----------|----------|-----------|");
    println!("| {} | Intersection | {:.3} |", now, ms);
}


