use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    };
    d.sort();
    let ans = d[n / 2] - d[n / 2 - 1];
    println!("{}", ans);
}
