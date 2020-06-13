use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        ftx: [(Usize1, Usize1, Usize1); q],
    };
    let mut x = vec![std::collections::LinkedList::new(); n];
    for (i, x_i) in x.iter_mut().enumerate() {
        x_i.push_back(i);
    }
    for &(f_i, t_i, x_i) in ftx.iter() {
        let p = x[f_i].iter().position(|&x_j| x_j == x_i).unwrap();
        let mut x_s = x[f_i].split_off(p);
        x[t_i].append(&mut x_s);
    }
    let mut ans = vec![0; n];
    for (i, x_i) in x.iter().enumerate() {
        for &x_ij in x_i.iter() {
            ans[x_ij] = i;
        }
    }
    for &a_i in ans.iter() {
        println!("{}", a_i + 1);
    }
}
