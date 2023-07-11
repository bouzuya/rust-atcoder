use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        cp: [(Usize1, i64); n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    };

    let mut c = vec![vec![0; n + 1]; 2];
    for (i, &(c_i, p_i)) in cp.iter().enumerate() {
        c[c_i][i + 1] = p_i;
    }
    for i in 0..n {
        for j in 0..2 {
            c[j][i + 1] += c[j][i];
        }
    }

    let mut s = String::new();
    for (l_i, r_i) in lr {
        let sum1 = c[0][r_i + 1] - c[0][l_i];
        let sum2 = c[1][r_i + 1] - c[1][l_i];
        s += &format!("{} {}\n", sum1, sum2);
    }
    print!("{}", s);
}
