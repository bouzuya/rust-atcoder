use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [usize; n],
    };
    let ans = h.into_iter().filter(|h_i| h_i >= &k).count();
    println!("{}", ans);
}
