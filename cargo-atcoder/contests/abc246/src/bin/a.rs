use proconio::input;

fn main() {
    input! {
        xy: [(i64, i64); 3],
    };
    let (x1, y1) = xy[0];
    let (x2, y2) = xy[1];
    let (x3, y3) = xy[2];
    let x = if x1 == x2 {
        x3
    } else if x1 == x3 {
        x2
    } else {
        x1
    };
    let y = if y1 == y2 {
        y3
    } else if y1 == y3 {
        y2
    } else {
        y1
    };
    println!("{} {}", x, y);
}
