use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };
    let mut ans = vec![];
    let mut set = std::collections::BTreeSet::new();
    for &a_i in a.iter().rev() {
        if set.insert(a_i) {
            ans.push(a_i);
        }
    }
    for i in 0..n {
        if set.insert(i) {
            ans.push(i);
        }
    }
    for &ans_i in ans.iter() {
        println!("{}", ans_i + 1);
    }
}
