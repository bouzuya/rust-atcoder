use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    };
    let ans = s.into_iter().filter(|s_i| s_i <= &x).sum::<usize>();
    println!("{}", ans);
}
