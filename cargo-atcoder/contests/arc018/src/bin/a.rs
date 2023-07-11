use proconio::input;

fn main() {
    input! {
        height: f64,
        bmi: f64,
    };
    let h = height / 100_f64;
    let weight = bmi * h * h;
    let ans = weight;
    println!("{:.3}", ans);
}
