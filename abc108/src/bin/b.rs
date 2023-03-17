use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    };
    let dx = x2 - x1;
    let dy = y2 - y1;
    let x3 = x2 - dy;
    let y3 = y2 + dx;
    let x4 = x3 - dx;
    let y4 = y3 - dy;
    println!("{} {} {} {}", x3, y3, x4, y4);
}
