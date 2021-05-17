use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = s.iter().enumerate().all(|(i, &s_i)| {
        if (i + 1) % 2 == 0 {
            "LUD".contains(s_i)
        } else {
            "RUD".contains(s_i)
        }
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
