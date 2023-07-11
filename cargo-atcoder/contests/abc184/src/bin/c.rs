use proconio::input;

fn main() {
    input! {
        rc1: (i64, i64),
        rc2: (i64, i64),
    };

    if rc1 == rc2 {
        println!("0");
        return;
    }

    let f = |(a, b): (i64, i64), (c, d): (i64, i64)| -> bool {
        (a + b) == (c + d) || (a - b) == (c - d)
    };
    let g = |(a, b): (i64, i64), (c, d): (i64, i64)| -> i64 { (a - c).abs() + (b - d).abs() };

    if f(rc1, rc2) || g(rc1, rc2) <= 3 {
        println!("1");
        return;
    }

    for i in -3..=3 {
        for j in -3..=3 {
            let rc = (rc1.0 + i, rc1.1 + j);
            if g(rc1, rc) > 3 {
                continue;
            }

            if f(rc, rc2) || g(rc, rc2) <= 3 {
                println!("2");
                return;
            }
        }
    }
    let (a, b) = rc1;
    let (c, d) = rc2;
    if (a + b) % 2 == (c + d) % 2 {
        println!("2");
        return;
    }

    println!("3");
}
