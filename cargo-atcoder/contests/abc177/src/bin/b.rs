use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut min = t.len();
    for i in 0..s.len() - t.len() + 1 {
        let mut count = 0_usize;
        for (j, t_j) in t.iter().copied().enumerate() {
            if s[i + j] != t_j {
                count += 1;
            }
        }
        min = min.min(count);
    }
    let ans = min;
    println!("{}", ans);
}
