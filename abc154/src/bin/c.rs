use proconio::input;

fn main() {
    input! {
        n: usize,
        mut av: [usize; n]
    };
    av.sort();
    av.dedup();
    let ans = if av.len() == n { "YES" } else { "NO" };
    println!("{}", ans);
}
