mod target_functions;
mod benchmark_utils;

use crate::benchmark_utils::core::{benchmark, benchmark_insert, benchmark_not};

fn main() {
    //ここにそれぞれの関数のベンチマークを計測する関数を書いていく。
    benchmark(target_functions::and, "Intersection",3);
    benchmark(target_functions::or, "Union",3);
    benchmark(target_functions::xor, "Symmetric Difference",1);
    benchmark(target_functions::eq, "Equality",1);
    // benchmark_not();
    // benchmark_insert();
}
