use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let ans = (|| {
        let mut depth = 0_i64;
        for s_i in s {
            match s_i {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => unreachable!(),
            }
            if depth < 0 {
                return false;
            }
        }
        depth == 0
    })();
    println!("{}", if ans { "Yes" } else { "No" });
}
