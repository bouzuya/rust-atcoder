use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
    }

    for a in 0..=n {
        for b in 0..=n - a {
            let c = n - (a + b);
            if ((10_000 * a) + (5_000 * b) + (1_000 * c)) == y {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
