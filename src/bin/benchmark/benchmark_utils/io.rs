use std::fs::{OpenOptions, create_dir_all};
use std::path::Path;
use std::io::Write;

use crate::benchmark_utils::core::ZOOM_LEVEL;

pub fn write_markdown(name: &str,ns: f64, count: usize) {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let remarks = "Only voxels with an elevation of zero or higher are considered.";
    let dir = "benchmark_history";
    let path_str = format!("{}/{}.md", dir, name);
    let path = Path::new(&path_str);
    
    create_dir_all(dir).expect("Failed to create benchmark_history directory");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    if path.metadata().unwrap().len() == 0 {
        writeln!(file, "# Benchmark History for `{}`\n", name).unwrap();
        writeln!(file, "These results were obtained at the zoom levels shown in the table by executing the function for all possible combinations of arguments and measuring the execution time.`\n").unwrap();
        writeln!(file, "| DateTime | Time (ns) | Execution time per run (ns) | Zoom Level | Operation Count | Remarks |").unwrap();
        writeln!(file, "|----------|----------|-----------------------|------------|----------------|--------|").unwrap();
    }

    writeln!(file, "| {} | {:.3} | {} | {} | {} | {} |", now, ns, ns / count as f64, ZOOM_LEVEL, count, remarks).unwrap();

    println!("# Benchmark Results {}", name);
    println!("| DateTime | Time (ns) | Execution time per run (ns) | Zoom Level | Operation Count | Remarks |");
    println!("|----------|----------|-----------------------|------------|----------------|--------|");
    println!("| {} | {:.3} | {} | {} | {} | {} |", now, ns, ns / count as f64, ZOOM_LEVEL, count, remarks);
}