use proconio::input;

fn main() {
    input! {
        a: i64,
    };
    let ans = a + a * a + a * a * a;
    println!("{}", ans);
}
