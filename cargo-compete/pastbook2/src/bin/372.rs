use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }

    let mut s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();

    s.sort();

    let mut ok = 1_i64 << 60;
    let mut ng = -1_i64;
    while ok - ng > 1 {
        let x = (ok + ng) / 2;

        let mut count = 0_usize;
        let mut l = 0_usize;
        let mut r = 0_usize;
        for i in 0..=n {
            while s[l] < s[i] - x {
                l += 1;
            }
            while r < n && s[r + 1] <= s[i] + x {
                r += 1;
            }

            count += r - l;
        }

        if count >= 2 * k {
            ok = x;
        } else {
            ng = x;
        }
    }

    println!("{}", ok);
}
