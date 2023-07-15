use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut pcf = vec![];
    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            f: [Usize1; c],
        }
        let mut f2 = vec![false; m];
        for f_i in f {
            f2[f_i] = true;
        }
        pcf.push((p, c, f2));
    }

    for i in 0..n {
        let (p_i, _, f_i) = &pcf[i];
        for j in 0..n {
            if i == j {
                continue;
            }
            let (p_j, _, f_j) = &pcf[j];
            if !(p_i >= p_j) {
                continue;
            }

            let mut ok = true;
            for k in 0..m {
                if f_i[k] && !f_j[k] {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }

            if p_i > p_j {
                println!("Yes");
                return;
            }

            let mut ok = false;
            for k in 0..m {
                if !f_i[k] && f_j[k] {
                    ok = true;
                    break;
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
