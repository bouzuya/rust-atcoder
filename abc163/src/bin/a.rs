use proconio::input;

fn main() {
    input! {
        r: f64,
    };
    let ans = 2_f64 * 3.14159265358979323846264338327950288_f64 * r;
    println!("{}", ans);
}
