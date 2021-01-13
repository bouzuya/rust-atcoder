use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
        l: usize,
    };
    let &ans = vec![a * k, b * ((k + l - 1) / l), b * (k / l) + a * (k % l)]
        .iter()
        .min()
        .unwrap();
    println!("{}", ans);
}
