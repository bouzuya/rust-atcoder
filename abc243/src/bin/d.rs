use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut x: u128,
        s: Chars,
    };
    let mut count = 0;
    for s_i in s {
        match s_i {
            'U' => {
                if count > 0 {
                    count -= 1;
                } else {
                    x /= 2;
                }
            }
            'L' => match x.checked_mul(2) {
                Some(y) => x = y,
                None => count += 1,
            },
            'R' => match x.checked_mul(2) {
                Some(y) => match y.checked_add(1) {
                    Some(y) => x = y,
                    None => count += 1,
                },
                None => count += 1,
            },
            _ => unreachable!(),
        }
    }
    let ans = x;
    println!("{}", ans);
}
