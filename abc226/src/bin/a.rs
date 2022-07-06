use proconio::input;

fn main() {
    input! {
        x: f64,
    };
    let ans = x.round();
    println!("{}", ans);
}
