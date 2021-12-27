use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        a: [Usize1; m],
    };
    let mut db = vec![vec![n; n]; 32 + 1];
    for i in 0..n {
        db[0][i] = i;
    }
    for a_i in a.into_iter().rev() {
        db[0].swap(a_i, a_i + 1);
    }
    for i in 0..32 {
        for j in 0..n {
            db[i + 1][j] = db[i][db[i][j]];
        }
    }

    for k in 0..n {
        let mut x = k;
        for i in 0..32 {
            if ((d >> i) & 1) == 1 {
                x = db[i][x];
            }
        }
        println!("{}", x + 1);
    }
}
