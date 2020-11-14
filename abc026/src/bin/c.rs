use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        b: [Usize1; n - 1],
    };
    let mut p = vec![0; n];
    let mut c = vec![vec![]; n];
    for (i, &b_i) in b.iter().enumerate() {
        c[b_i].push(i + 1);
        p[i + 1] = b_i;
    }
    let mut v = vec![0; n];
    let mut l = c.iter().map(|s_i| s_i.len()).collect::<Vec<usize>>();
    let mut l_next = l.clone();
    for i in 1..n {
        if v[i] == 0 && l[i] == 0 {
            v[i] = 1;
            l_next[p[i]] -= 1;
        }
    }
    while v.iter().skip(1).position(|&v_i| v_i == 0).is_some() {
        l = l_next;
        l_next = l.clone();
        for i in 1..n {
            if v[i] == 0 && l[i] == 0 {
                let mut max = 0;
                let mut min = 1_000_000_000;
                for &s_ij in c[i].iter() {
                    max = std::cmp::max(max, v[s_ij]);
                    min = std::cmp::min(min, v[s_ij]);
                }
                v[i] = max + min + 1;
                l_next[p[i]] -= 1;
            }
        }
    }

    let ans = {
        let mut max = 0;
        let mut min = 1_000_000_000;
        for &s_ij in c[0].iter() {
            max = std::cmp::max(max, v[s_ij]);
            min = std::cmp::min(min, v[s_ij]);
        }
        max + min + 1
    };
    println!("{}", ans);
}
