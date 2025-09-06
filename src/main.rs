use std::ops::{Add, BitAnd, Shl, Shr, Sub};

fn intervals_and_values_f(n: u8, start: i32, end: i32) -> Vec<((i32, i32), bool)> {
    let step = 1 << n; // 2^n
    let mut intervals = Vec::new();

    let mut current = start;
    let mut b = (start >> n) << n;
    if b < start {
        b += step;
    }

    while current <= end {
        // saturating_sub でオーバーフロー防止
        let next = b.saturating_sub(1);
        let interval_end = if next > end { end } else { next };
        let value = ((current >> n) & 1) == 0;
        intervals.push(((current, interval_end), value));

        current = b;
        b = b.saturating_add(step); // u32 と i32 両方で安全
    }

    intervals
}

fn main() {
    let n = 3;

    // u32 の場合
    let result_u32 = intervals_and_values_f(n, 3, 3);
    println!("\nu32:");

    for ((s, e), val) in result_u32 {
        println!("x = {}..{} => {}", s, e, val);
    }
}
