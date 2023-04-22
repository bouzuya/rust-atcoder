use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut b = vec![];
    let mut x = 0;
    for (i, s_i) in s.iter().copied().enumerate() {
        match s_i {
            '|' => b.push(i),
            '*' => x = i,
            '.' => {}
            _ => unreachable!(),
        }
    }
    let ans = b[0] <= x && x <= b[1];
    println!("{}", if ans { "in" } else { "out" });
}
