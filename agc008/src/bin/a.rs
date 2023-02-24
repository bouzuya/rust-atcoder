use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    };

    let mut min = 1_i64 << 60;
    for &b1 in &[false, true] {
        for &b2 in &[false, true] {
            let mut a = x;
            let mut b = y;
            let mut count = 0;
            if b1 {
                a *= -1;
                count += 1;
            }
            if b2 {
                b *= -1;
                count += 1;
            }
            if a <= b {
                min = min.min(b - a + count);
            }
        }
    }

    println!("{}", min);
}
