pub mod bench_and;

fn main() {
    let zoom_level :i64 = 3;
    //ここにそれぞれの関数のベンチマークを計測する関数を書いていく。
    bench_and::benchmark_and(&zoom_level);

    println!("benchmark({})", zoom_level);

}
