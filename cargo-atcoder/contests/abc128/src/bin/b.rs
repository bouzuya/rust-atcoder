use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        sp: [(String, usize); n],
    };
    let mut spi = sp
        .into_iter()
        .enumerate()
        .map(|(i, (s, p))| (s, p, i))
        .collect::<Vec<(String, usize, usize)>>();
    spi.sort_by_key(|(s, p, _)| (s.clone(), Reverse(*p)));
    for (_, _, i) in spi {
        println!("{}", i + 1);
    }
}
