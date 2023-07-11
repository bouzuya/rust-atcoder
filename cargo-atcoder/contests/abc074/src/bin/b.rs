use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n],
    };
    let ans = x.into_iter().map(|x_i| x_i.min(k - x_i) * 2).sum::<usize>();
    println!("{}", ans);
}
