use proconio::input;

fn f((x_a, y_a): (i64, i64), (x_b, y_b): (i64, i64), (x, y): (i64, i64)) -> i64 {
    if x_a == x && y_a == y {
        return 0;
    }
    (x_a - x).abs()
        + (y_a - y).abs()
        + if x_a == x && x_a == x_b && (y_a < y_b) != (y < y_b) {
            2
        } else {
            0
        }
        + if y_a == y && y_a == y_b && (x_a < x_b) != (x < x_b) {
            2
        } else {
            0
        }
}

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
    };

    // c が原点になるよう平行移動
    let a = (a.0 - c.0, a.1 - c.1);
    let b = (b.0 - c.0, b.1 - c.1);

    // b.0 >= 0 になるよう移動
    let (a, b) = if b.0 < 0 {
        ((-a.0, a.1), (-b.0, b.1))
    } else {
        (a, b)
    };

    // b.1 >= 0 になるよう移動
    let (a, b) = if b.1 < 0 {
        ((a.0, -a.1), (b.0, -b.1))
    } else {
        (a, b)
    };

    // b.1 > 0 になるよう移動
    let (a, b) = if b.1 == 0 {
        ((a.1, a.0), (b.1, b.0))
    } else {
        (a, b)
    };

    let ans = if b.0 == 0 {
        b.1 + f(a, b, (b.0, b.1 + 1))
    } else {
        b.0 + b.1 + 2 + f(a, b, (b.0, b.1 + 1)).min(f(a, b, (b.0 + 1, b.1)))
    };
    println!("{}", ans);
}
