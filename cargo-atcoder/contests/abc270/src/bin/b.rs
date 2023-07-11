use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    };
    let (x, y, z) = if x < 0 { (-x, -y, -z) } else { (x, y, z) };
    if !(0..x).contains(&y) {
        println!("{}", x);
        return;
    }

    if z < 0 {
        println!("{}", z.abs() * 2 + x);
    } else if z < y {
        println!("{}", x);
    } else {
        println!("-1");
    }
}
