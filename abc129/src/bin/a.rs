use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        r: usize,
    };
    let ans = *vec![p + q, p + r, q + r].iter().min().unwrap();
    println!("{}", ans);
}
