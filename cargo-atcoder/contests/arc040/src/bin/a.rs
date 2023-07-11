use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut c_r = 0;
    let mut c_b = 0;
    for s_i in s.iter() {
        for &s_ij in s_i.iter() {
            match s_ij {
                'R' => c_r += 1,
                'B' => c_b += 1,
                '.' => {}
                _ => unreachable!(),
            }
        }
    }
    let ans = if c_r == c_b {
        "DRAW"
    } else if c_r > c_b {
        "TAKAHASHI"
    } else if c_r < c_b {
        "AOKI"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
