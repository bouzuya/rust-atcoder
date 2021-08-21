use std::collections::HashSet;

use proconio::input;

fn f(n: usize) -> HashSet<usize> {
    let mut p = HashSet::new();
    if n < 2 {
        return p;
    }
    let mut n = n; // shadowing
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut c = 0;
        while n % i == 0 {
            c += 1;
            n /= i;
        }
        if c > 0 {
            p.insert(i);
        }
    }
    if n != 1 {
        p.insert(n);
    }
    p
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let mut set = HashSet::new();
    for a_i in a.iter().copied() {
        for k in f(a_i) {
            set.insert(k);
        }
    }
    let mut b = vec![true; m + 1];
    for x in 1..=m {
        if b[x] && set.contains(&x) {
            b[x] = false;
            for j in (x + x..=m).step_by(x) {
                b[j] = false;
            }
        }
    }
    let mut ans = vec![];
    for x in 1..=m {
        if b[x] {
            ans.push(x);
        }
    }

    println!("{}", ans.len());
    for x in ans {
        println!("{}", x);
    }
}
