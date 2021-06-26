use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = *vec![a + b, a + c, b + c].iter().max().unwrap();
    println!("{}", ans);
}
