use std::collections::BTreeSet;

use proconio::input;

fn f(base: &mut BTreeSet<usize>, set: &mut BTreeSet<usize>, a: usize, b: usize) {
    base.insert(a * b);
    for x in 1.. {
        if a * x > 10000 {
            break;
        }
        for y in 1.. {
            if a * x * b * y > 10000 {
                break;
            }
            set.insert(a * x * b * y);
        }
    }
}

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn g(a: &[usize]) -> bool {
    let mut x = a[0];
    for a_i in a.iter().copied() {
        x = gcd(x, a_i);
    }
    if x != 1 {
        return false;
    }

    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if !(i != j && gcd(a[j], a[i]) > 1) {
                return false;
            }
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
    };

    let mut base = BTreeSet::new();
    let mut set = BTreeSet::new();
    f(&mut base, &mut set, 2, 3);
    f(&mut base, &mut set, 2, 5);
    f(&mut base, &mut set, 3, 5);

    let mut ans = vec![];
    for a in base.iter().copied() {
        ans.push(a);
    }
    for a in set.into_iter() {
        if base.contains(&a) {
            continue;
        }
        ans.push(a)
    }

    assert!(g(&ans));

    for (i, a) in ans.into_iter().enumerate().take(n) {
        print!("{}{}", a, if i == n - 1 { "\n" } else { " " });
    }
}
