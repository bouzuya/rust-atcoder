use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _n: usize,
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
        s: Chars,
    };

    for (s_i, s_j) in (a..c).map(|i| s[i]).zip((a..c).skip(1).map(|j| s[j])) {
        if s_i == '#' && s_j == '#' {
            println!("No");
            return;
        }
    }
    for (s_i, s_j) in (b..d).map(|i| s[i]).zip((b..d).skip(1).map(|j| s[j])) {
        if s_i == '#' && s_j == '#' {
            println!("No");
            return;
        }
    }
    if c < d {
        println!("Yes");
    } else {
        for i in b + 1..=d + 1 {
            let (s_i, s_j, s_k) = (s[i - 2], s[i - 1], s[i]);
            if s_i == '.' && s_j == '.' && s_k == '.' {
                println!("Yes");
                return;
            }
        }
        println!("No");
    }
}
