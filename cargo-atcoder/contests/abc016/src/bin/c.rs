use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut f = vec![std::collections::BTreeSet::new(); n];
    for &(a_i, b_i) in ab.iter() {
        f[a_i].insert(b_i);
        f[b_i].insert(a_i);
    }
    for (i, f_i) in f.iter().enumerate() {
        let mut s = std::collections::BTreeSet::new();
        for &j in f_i.iter() {
            for f_ijk in f[j].iter() {
                s.insert(f_ijk);
            }
        }
        s.remove(&i);
        for j in f_i.iter() {
            s.remove(j);
        }
        println!("{}", s.len());
    }
}
