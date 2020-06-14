use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(i64, Usize1); n],
        cd: [(Usize1, Usize1); q],
    };
    let mut min_pv = std::collections::BTreeSet::new();
    let mut c = vec![0; n];
    let mut p = vec![std::collections::BTreeSet::new(); 200_000 + 1];
    for (i, &(a_i, b_i)) in ab.iter().enumerate() {
        c[i] = b_i;
        let x_o = p[b_i].iter().rev().map(|&x| x).next(); // last = max
        p[b_i].insert((a_i, i));
        let x_n = p[b_i].iter().rev().map(|&x| x).next();
        if x_o != x_n {
            if let Some(x_o) = x_o {
                min_pv.remove(&x_o);
            }
            if let Some(x_n) = x_n {
                min_pv.insert(x_n);
            }
        }
    }

    for &(c_i, d_i) in cd.iter() {
        let p_f = c[c_i];
        let p_t = d_i;
        let a_i = ab[c_i].0;

        c[c_i] = d_i;

        let x_o = p[p_f].iter().rev().map(|&x| x).next();
        p[p_f].remove(&(a_i, c_i));
        let x_n = p[p_f].iter().rev().map(|&x| x).next();
        if x_o != x_n {
            if let Some(x_o) = x_o {
                min_pv.remove(&x_o);
            }
            if let Some(x_n) = x_n {
                min_pv.insert(x_n);
            }
        }

        let x_o = p[p_t].iter().rev().map(|&x| x).next();
        p[p_t].insert((a_i, c_i));
        let x_n = p[p_t].iter().rev().map(|&x| x).next();
        if x_o != x_n {
            if let Some(x_o) = x_o {
                min_pv.remove(&x_o);
            }
            if let Some(x_n) = x_n {
                min_pv.insert(x_n);
            }
        }

        let x = min_pv.iter().next().unwrap(); // first = min
        println!("{}", x.0);
    }
}
