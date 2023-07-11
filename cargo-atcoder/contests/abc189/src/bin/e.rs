// 解説 AC <https://www.youtube.com/watch?v=u2woAs3M1c0>
use proconio::{input, marker::Usize1};

#[derive(Clone, Debug)]
struct A {
    a: Vec<Vec<i64>>,
    b: Vec<i64>,
}

impl A {
    fn new() -> Self {
        Self {
            a: vec![vec![1, 0], vec![0, 1]],
            b: vec![0, 0],
        }
    }

    fn new_a(a: i64, b: i64, c: i64, d: i64) -> Self {
        Self {
            a: vec![vec![a, b], vec![c, d]],
            b: vec![0, 0],
        }
    }

    fn new_ab(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> Self {
        Self {
            a: vec![vec![a, b], vec![c, d]],
            b: vec![e, f],
        }
    }

    fn mul(&self, x: A) -> A {
        let mut res = A::new_a(0, 0, 0, 0);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    res.a[i][j] += x.a[i][k] * self.a[k][j];
                }
            }
        }
        res.b = vec![0, 0];
        for i in 0..2 {
            for j in 0..2 {
                res.b[i] += x.a[i][j] * self.b[j];
            }
        }
        for i in 0..2 {
            res.b[i] += x.b[i];
        }
        res
    }

    fn mul_b(&self, b: &Vec<i64>) -> Vec<i64> {
        let mut res = self.b.clone();
        for i in 0..2 {
            for j in 0..2 {
                res[i] += self.a[i][j] * b[j];
            }
        }
        res
    }
}

fn main() {
    input! {
        n: usize,
        xy: [[i64; 2]; n],
        m: usize,
    };
    let mut d = vec![A::new()];
    for _ in 0..m {
        input! {
            t: usize,
        };
        let x = match t {
            1 => A::new_a(0, 1, -1, 0),
            2 => A::new_a(0, -1, 1, 0),
            3 => {
                input! { p: i64 };
                A::new_ab(-1, 0, 0, 1, 2 * p, 0)
            }
            4 => {
                input! { p: i64 };
                A::new_ab(1, 0, 0, -1, 0, 2 * p)
            }
            _ => unreachable!(),
        };
        d.push(d.last().unwrap().mul(x));
    }
    input! {
        q: usize,
        ab: [(usize, Usize1); q]
    };
    for (a_i, b_i) in ab {
        let ans = d[a_i].mul_b(&xy[b_i]);
        println!("{} {}", ans[0], ans[1]);
    }
}
