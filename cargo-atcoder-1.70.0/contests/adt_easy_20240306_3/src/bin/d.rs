use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    };
    let (x, y, z) = if x < 0 { (-x, -y, -z) } else { (x, y, z) };
    let ans = if (0..x).contains(&y) {
        if y < z {
            -1
        } else if z > 0 {
            x
        } else {
            x - z * 2
        }
    } else {
        x
    };
    println!("{}", ans);
}
