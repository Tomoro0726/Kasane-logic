mod benchmark;
mod target_functions;

fn main() {
    //ここにそれぞれの関数のベンチマークを計測する関数を書いていく。
    benchmark::benchmark(target_functions::and, "Intersection");
    benchmark::benchmark(target_functions::or, "Union");
    benchmark::benchmark(target_functions::not, "Complement");
}
