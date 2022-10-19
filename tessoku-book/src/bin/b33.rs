use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        _h: usize,
        _w: usize,
        ab: [(Usize1, Usize1); n],
    };
    let mut x = vec![];
    for (a_i, b_i) in ab {
        x.push(a_i);
        x.push(b_i);
    }
    let ans = x.into_iter().fold(0, |acc, x_i| acc ^ x_i) != 0;
    println!("{}", if ans { "First" } else { "Second" });
}
