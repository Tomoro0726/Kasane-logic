mod target_functions;
mod benchmark_utils;

use crate::benchmark_utils::core::{benchmark_main, benchmark_insert, benchmark_not};

fn main() {
    //ここにそれぞれの関数のベンチマークを計測する関数を書いていく。
    benchmark_main(target_functions::and, "Intersection",3);
    benchmark_main(target_functions::or, "Union",3);
    benchmark_main(target_functions::xor, "Symmetric Difference",1);
    benchmark_main(target_functions::eq, "Equality",1);
    benchmark_not(1);
    benchmark_insert(1);
}
