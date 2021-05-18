use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    };
    let ans = a.iter().filter(|&&a_i| a_i >= k).count();
    println!("{}", ans);
}
