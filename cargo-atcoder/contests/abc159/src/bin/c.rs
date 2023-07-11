use proconio::input;

fn main() {
    input! {
        l: f64,
    };
    let ans = (l / 3_f64).powf(3_f64);
    println!("{}", ans);
}
