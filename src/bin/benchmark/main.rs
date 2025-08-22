mod benchmark;
mod target_functions;

fn main() {
    let zoom_level :i64 = 2;
    //ここにそれぞれの関数のベンチマークを計測する関数を書いていく。
    benchmark::benchmark(&zoom_level, target_functions::and, "Intersection");
}
