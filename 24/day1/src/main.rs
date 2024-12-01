use aoc_driver::*;

fn solution(i: &str) -> String {
    print(i)
    todo!();
}


fn main() {
    let session = std::fs::read_to_string("../../../.session.txt").unwrap();
    aoc_magic!(&session, 2024:1:1, solution1).unwrap();
}
