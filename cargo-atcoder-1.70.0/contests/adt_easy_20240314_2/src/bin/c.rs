use proconio::input;

fn main() {
    input! {
        x: i64
    };
    let ans = num::Integer::div_floor(&x, &10);
    println!("{}", ans);
}
