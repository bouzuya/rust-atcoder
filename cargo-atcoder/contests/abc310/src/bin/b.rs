use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut p = vec![];
    let mut c = vec![];
    for _ in 0..n {
        input! {
            p_i: usize,
            c_i: usize,
            f_i: [Usize1; c_i],
        }
        let mut c_i = vec![false; m];
        for f in f_i {
            c_i[f] = true;
        }

        p.push(p_i);
        c.push(c_i);
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if !(p[i] >= p[j]) {
                continue;
            }

            if !(0..m).filter(|&k| c[i][k]).all(|k| c[j][k]) {
                continue;
            }

            if p[i] > p[j] || (0..m).filter(|&k| c[j][k]).any(|k| !c[i][k]) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
