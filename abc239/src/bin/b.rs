use proconio::input;

fn main() {
    input! {
        x: i128,
    };

    let ans = if x >= 0 { x / 10 } else { -((-x + 9) / 10) };
    println!("{}", ans);
}
