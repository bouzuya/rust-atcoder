use proconio::input;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum D {
    TC,
    TR,
    MR,
    BR,
    BC,
    BL,
    ML,
    TL,
}

fn f((x_a, y_a): (i64, i64), (x_b, y_b): (i64, i64)) -> D {
    match (x_b - x_a, y_b - y_a) {
        (0, y) if y > 0 => D::TC,
        (x, y) if x > 0 && y > 0 => D::TR,
        (x, 0) if x > 0 => D::MR,
        (x, y) if x > 0 && y < 0 => D::BR,
        (0, y) if y < 0 => D::BC,
        (x, y) if x < 0 && y < 0 => D::BL,
        (x, 0) if x < 0 => D::ML,
        (x, y) if x < 0 && y > 0 => D::TL,
        _ => unreachable!(),
    }
}

fn g((x_a, y_a): (i64, i64), (x_b, y_b): (i64, i64)) -> i64 {
    (x_b - x_a).abs() + (y_b - y_a).abs()
}

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
    };

    let dir_ab = f(a, b);
    let dir_bc = f(b, c);
    let dist_ab = g(a, b);
    let dist_bc = g(b, c);
    let x = match (dir_ab, dir_bc) {
        (D::TC, D::TC) | (D::BC, D::BC) | (D::MR, D::MR) | (D::ML, D::ML) => 0,

        (D::TC, D::TR) => 1,
        (D::TC, D::MR) => 1,
        (D::TC, D::BR) => 2,
        (D::TC, D::BC) => 2,
        (D::TC, D::BL) => 2,
        (D::TC, D::ML) => 1,
        (D::TC, D::TL) => 1,

        (D::TR, D::TC) => 0,
        (D::TR, D::TR) => 1,
        (D::TR, D::MR) => 0,
        (D::TR, D::BR) => 1,
        (D::TR, D::BC) => 1,
        (D::TR, D::BL) => 2,
        (D::TR, D::ML) => 1,
        (D::TR, D::TL) => 1,

        (D::MR, D::TC) => 1,
        (D::MR, D::TR) => 1,
        (D::MR, D::BR) => 1,
        (D::MR, D::BC) => 1,
        (D::MR, D::BL) => 2,
        (D::MR, D::ML) => 2,
        (D::MR, D::TL) => 2,

        (D::BR, D::TC) => 1,
        (D::BR, D::TR) => 1,
        (D::BR, D::MR) => 0,
        (D::BR, D::BR) => 1,
        (D::BR, D::BC) => 0,
        (D::BR, D::BL) => 1,
        (D::BR, D::ML) => 1,
        (D::BR, D::TL) => 2,

        (D::BC, D::TC) => 2,
        (D::BC, D::TR) => 2,
        (D::BC, D::MR) => 1,
        (D::BC, D::BR) => 1,
        (D::BC, D::BL) => 1,
        (D::BC, D::ML) => 1,
        (D::BC, D::TL) => 2,

        (D::BL, D::TC) => 1,
        (D::BL, D::TR) => 2,
        (D::BL, D::MR) => 1,
        (D::BL, D::BR) => 1,
        (D::BL, D::BC) => 0,
        (D::BL, D::BL) => 1,
        (D::BL, D::ML) => 0,
        (D::BL, D::TL) => 1,

        (D::ML, D::TC) => 1,
        (D::ML, D::TR) => 2,
        (D::ML, D::MR) => 2,
        (D::ML, D::BR) => 2,
        (D::ML, D::BC) => 1,
        (D::ML, D::BL) => 1,
        (D::ML, D::TL) => 1,

        (D::TL, D::TC) => 0,
        (D::TL, D::TR) => 1,
        (D::TL, D::MR) => 1,
        (D::TL, D::BR) => 2,
        (D::TL, D::BC) => 1,
        (D::TL, D::BL) => 1,
        (D::TL, D::ML) => 0,
        (D::TL, D::TL) => 1,
    };

    let ans = dist_ab - 1 + dist_bc + x * 2;
    println!("{}", ans);
}
