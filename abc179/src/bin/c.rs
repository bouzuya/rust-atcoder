use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut spf = vec![0; n + 1];
    for i in 0..n + 1 {
        spf[i] = i;
    }
    for p in 2.. {
        if p * p > n {
            break;
        }
        for i in (p..n + 1).step_by(p) {
            if spf[i] == i {
                spf[i] = p;
            }
        }
    }

    let mut ans = 0_usize;
    for i in 1..=n - 1 {
        let mut x = i;
        let mut map = std::collections::BTreeMap::new();
        while x != 1 {
            let p = spf[x];
            *map.entry(p).or_insert(0) += 1;
            x /= p;
        }

        let mut c = 1;
        for (_, v) in map {
            c *= v + 1;
        }
        ans += c;
    }
    println!("{}", ans);
}
