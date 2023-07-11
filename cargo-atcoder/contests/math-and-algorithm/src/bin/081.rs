use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let c1 = n / 10_000;
    let n1 = n % 10_000;
    let c2 = n1 / 5_000;
    let n2 = n1 % 5_000;
    let c3 = n2 / 1_000;
    let ans = c1 + c2 + c3;
    println!("{}", ans);
}
