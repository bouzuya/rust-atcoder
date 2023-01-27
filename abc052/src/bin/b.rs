use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut max = 0_i64;
    let mut cur = 0_i64;
    for s_i in s {
        match s_i {
            'I' => cur += 1,
            'D' => cur -= 1,
            _ => unreachable!(),
        }
        max = max.max(cur);
    }
    let ans = max;
    println!("{}", ans);
}
