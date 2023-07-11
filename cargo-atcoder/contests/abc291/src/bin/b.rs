use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; 5 * n],
    };
    x.sort();

    let sum = x.iter().skip(n).take(3 * n).sum::<usize>();
    let ans = sum as f64 / (3 * n) as f64;
    println!("{}", ans);
}
