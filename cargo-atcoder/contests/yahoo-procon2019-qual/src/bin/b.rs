use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        ab: [(Usize1, Usize1); 3],
    };
    let mut count = vec![0; 4];
    for &(a_i, b_i) in ab.iter() {
        count[a_i] += 1;
        count[b_i] += 1;
    }
    let ans = count.iter().all(|&c| c <= 2);
    println!("{}", if ans { "YES" } else { "NO" });
}
