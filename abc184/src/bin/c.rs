use proconio::input;

fn f(a: i64, b: i64, c: i64, d: i64) -> bool {
    (a + b == c + d) || (a - b == c - d) || ((a - c).abs() + (b - d).abs() <= 3)
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    if a == c && b == d {
        println!("0");
        return;
    }
    if f(a, b, c, d) {
        println!("1");
        return;
    }
    if (a + b) % 2 == (c + d) % 2 {
        println!("2");
        return;
    }
    for y in -3..=3 {
        for x in -3..=3 {
            if (a - (a + x)).abs() + (b - (b + y)).abs() > 3 {
                continue;
            }
            if f(a + x, b + y, c, d) {
                println!("2");
                return;
            }
        }
    }
    println!("3");
}
