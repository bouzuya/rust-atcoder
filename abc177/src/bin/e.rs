use proconio::input;

fn gcd(n: usize, m: usize) -> usize {
    if n < m {
        gcd(m, n)
    } else if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    // SPF: Smallest Prime Factor 最小の素因数
    let max_a_i = 1_000_000_usize;
    let mut spf = vec![1_usize; max_a_i + 1];
    for i in 2..=max_a_i {
        if spf[i] == 1 {
            for j in (i..=max_a_i).step_by(i) {
                spf[j] = i;
            }
        }
    }

    let mut pairwise = true;
    let mut pairwise_set = std::collections::BTreeSet::new();
    let mut setwise = 0;
    for &a_i in a.iter() {
        let mut set = std::collections::BTreeSet::new();
        let mut x = a_i;
        while x != 1 && set.insert(spf[x]) {
            x /= spf[x];
        }
        for &p in set.iter() {
            if !pairwise_set.insert(p) {
                pairwise = false;
            }
        }

        setwise = if setwise == 0 { a_i } else { gcd(setwise, a_i) };
    }

    let ans = if pairwise {
        "pairwise coprime"
    } else if !pairwise && setwise == 1 {
        "setwise coprime"
    } else {
        "not coprime"
    };
    println!("{}", ans);
}
