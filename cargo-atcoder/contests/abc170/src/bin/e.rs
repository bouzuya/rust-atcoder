use proconio::input;
use proconio::marker::Usize1;

type C = (i64, usize);

fn update<F>(
    h: &mut std::collections::BTreeSet<C>,
    p_i: &mut std::collections::BTreeSet<C>,
    f: &mut F,
) where
    F: FnMut(&mut std::collections::BTreeSet<C>) -> (),
{
    let x_o = p_i.iter().last().map(|&x| x); // h.last = max
    f(p_i);
    let x_n = p_i.iter().last().map(|&x| x);
    if x_o != x_n {
        if let Some(x_o) = x_o {
            h.remove(&x_o);
        }
        if let Some(x_n) = x_n {
            h.insert(x_n);
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(i64, Usize1); n],
        cd: [(Usize1, Usize1); q],
    };
    let mut h = std::collections::BTreeSet::new(); // h: 各園の最高値を持つ園児の集合
    let mut c = vec![0; n]; // c[i]: 園児 i の現在の園
    let mut p = vec![std::collections::BTreeSet::new(); 200_000 + 1]; // p[i]: 園 i の園児の集合
    for (i, &(a_i, b_i)) in ab.iter().enumerate() {
        c[i] = b_i;
        update(&mut h, &mut p[b_i], &mut |p_i| {
            p_i.insert((a_i, i));
        });
    }

    for &(c_i, d_i) in cd.iter() {
        let p_f = c[c_i];
        let p_t = d_i;
        let a_i = ab[c_i].0;

        c[c_i] = d_i;
        update(&mut h, &mut p[p_f], &mut |p_i| {
            p_i.remove(&(a_i, c_i));
        });
        update(&mut h, &mut p[p_t], &mut |p_i| {
            p_i.insert((a_i, c_i));
        });

        let x = h.iter().next().unwrap(); // h.first = min
        println!("{}", x.0);
    }
}
