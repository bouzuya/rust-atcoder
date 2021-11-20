use proconio::{input, marker::Usize1};

fn f(used: &mut [bool], n: usize, a: &[usize], x: usize) {
    if used[x] {
        return;
    }
    used[x] = true;

    f(used, n, a, a[x]);
}

fn main() {
    input! {
        n: usize,
        x: Usize1,
        a: [Usize1; n],
    };
    let mut used = vec![false; n];
    f(&mut used, n, &a, x);
    let ans = used.iter().filter(|&&b| b).count();
    println!("{}", ans);
}
