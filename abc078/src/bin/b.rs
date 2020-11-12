use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    };
    let mut a = 1;
    while a * (y + z) + z <= x {
        a += 1;
    }
    let ans = a - 1;
    println!("{}", ans);
}
