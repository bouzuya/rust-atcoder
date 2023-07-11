use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let ans = s
        .windows(t.len())
        .map(|s_i| {
            s_i.iter()
                .zip(t.iter())
                .filter(|(s_ij, t_j)| s_ij != t_j)
                .count()
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
