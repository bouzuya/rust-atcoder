use proconio::input;
use proconio::marker::Chars;

#[derive(Clone, Copy, Eq, PartialEq)]
enum P {
    // a
    // a
    V,
    // aa
    // bb
    H,
}

fn main() {
    input! {
        n: usize,
        s1: Chars,
        s2: Chars,
    };

    let mut p = vec![];
    let mut index = 0;
    while index < n {
        if s1[index] == s2[index] {
            p.push(P::V);
            index += 1;
        } else {
            p.push(P::H);
            index += 2;
        }
    }

    let modp = 1_000_000_007;
    let mut ans = match p[0] {
        P::V => 3_i64,
        P::H => 6_i64,
    };
    for w in p.windows(2) {
        match w {
            [P::H, P::H] => {
                ans *= 3;
                ans %= modp;
            }
            [P::H, P::V] => {}
            [P::V, P::H] => {
                ans *= 2;
                ans %= modp;
            }
            [P::V, P::V] => {
                ans *= 2;
                ans %= modp;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
