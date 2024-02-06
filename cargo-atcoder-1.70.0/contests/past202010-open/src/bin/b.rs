use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    };
    if y == 0 {
        println!("ERROR");
        return;
    }
    let n = x * 100 / y;
    println!("{}.{:02}", n / 100, n % 100);
}
