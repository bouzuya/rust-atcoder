use proconio::input;

fn main() {
    input! {
        n: usize,
        capital_y: usize,
    };
    for x in 0..=n {
        for y in 0..=n - x {
            let z = n - (x + y);
            if 10_000 * x + 5_000 * y + 1_000 * z == capital_y {
                println!("{} {} {}", x, y, z);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
