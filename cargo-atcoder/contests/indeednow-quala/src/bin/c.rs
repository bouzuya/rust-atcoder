use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
        q: usize,
        k: [usize; q],
    };
    let mut s = s.into_iter().filter(|&s_i| s_i != 0).collect::<Vec<i64>>();
    s.sort_by_key(|s_i| -s_i);
    let n = s.len();

    for &k_i in k.iter() {
        let ans = if k_i >= n { 0 } else { s[k_i] + 1 };
        println!("{}", ans);
    }
}
