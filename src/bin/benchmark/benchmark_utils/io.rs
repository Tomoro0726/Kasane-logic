use std::{fs::OpenOptions, io::Write, path::Path};

pub fn write_markdown(name: &str,ns: f64) {
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