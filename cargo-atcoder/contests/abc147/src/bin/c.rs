use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![vec![]; n];
    for i in 0..n {
        input! {
            a_i: usize,
            xy: [(Usize1, usize); a_i],
        }
        a[i] = xy;
    }
    let mut ans = 0;
    for bits in 0..1 << n {
        let mut ok = true;
        let is = (0..n)
            .map(|i| ((bits >> i) & 1) == 1)
            .collect::<Vec<bool>>();
        for i in 0..n {
            if !is[i] {
                continue;
            }
            for (x, y) in a[i].iter().copied() {
                if (y == 1) != is[x] {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            ans = ans.max(is.iter().filter(|&&b| b).count());
        }
    }
    println!("{}", ans);
}
