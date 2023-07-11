use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    // 010101...
    let mut o = 0;
    // 101010...
    let mut e = 0;
    for (i, s_i) in s.iter().copied().enumerate() {
        match (i % 2 == 0, s_i == '1') {
            (false, false) => o += 1,
            (false, true) => e += 1,
            (true, false) => e += 1,
            (true, true) => o += 1,
        }
    }
    let ans = e.min(o);
    println!("{}", ans);
}
