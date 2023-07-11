use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
    };

    let b = {
        let s = std::iter::once(0)
            .chain(a.iter().scan(0, |acc, &i| {
                *acc += i;
                Some(*acc)
            }))
            .collect::<Vec<u64>>();
        let mut b = vec![];
        for i in 0..=n {
            for j in i + 1..=n {
                b.push(s[j] - s[i]);
            }
        }
        b
    };

    let mut res = 0_u64;
    let mut c = b;
    for i in (0..64).rev() {
        let mut next = vec![];
        for c_i in c.iter().copied() {
            if ((c_i >> i) & 1) == 1 {
                next.push(c_i);
            }
        }
        if next.len() >= k {
            c = next;
            res += 2_u64.pow(i);
        }
    }

    let ans = res;
    println!("{}", ans);
}
