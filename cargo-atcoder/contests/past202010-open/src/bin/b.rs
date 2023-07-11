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
    let v = x * 100 / y;
    let r = v % 100;
    let q = v / 100;
    println!("{}.{:02}", q, r);
}
