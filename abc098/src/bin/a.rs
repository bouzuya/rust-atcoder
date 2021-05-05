use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = *vec![a + b, a - b, a * b].iter().max().unwrap();
    println!("{}", ans);
}
