use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        t: Chars,
    };
    enum D {
        R,
        D,
        L,
        U,
    };
    let mut d = D::R;
    let mut p = (0_i64, 0_i64);
    for t_i in t {
        match t_i {
            'S' => match d {
                D::R => p.0 += 1,
                D::D => p.1 -= 1,
                D::L => p.0 -= 1,
                D::U => p.1 += 1,
            },
            'R' => {
                d = match d {
                    D::R => D::D,
                    D::D => D::L,
                    D::L => D::U,
                    D::U => D::R,
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{} {}", p.0, p.1);
}
