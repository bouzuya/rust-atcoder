use proconio::input;

fn main() {
    input! {
        k: usize,
        n: usize,
        m: usize,
        a: [i64; k],
    };
    if n == m {
        for a_i in a {
            println!("{}", a_i);
        }
        return;
    }

    let mut ng = -1_i64;
    let mut ok = 1_i64 << 60;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        let m = m as i64;
        let n = n as i64;
        let mut l_sum = 0_i64;
        let mut r_sum = 0_i64;
        for a_i in a.iter().copied() {
            let l = (m * a_i - x + (n - 1)) / n;
            let r = (m * a_i + x) / n;
            l_sum += l;
            r_sum += r;
        }
        if (l_sum..=r_sum).contains(&m) {
            ok = x;
        } else {
            ng = x;
        }
    }
    let x = ok;
    let m = m as i64;
    let n = n as i64;
    let mut b = vec![];
    for a_i in a.iter().copied() {
        let l = (m * a_i - x + (n - 1)) / n;
        b.push(l);
    }
    let mut sum = m - b.iter().sum::<i64>();
    for (i, b) in b.iter_mut().enumerate() {
        let r = (m * a[i] + x) / n;
        let d = (r - *b).min(sum);
        *b += d;
        sum -= d;
    }
    for b_i in b {
        println!("{}", b_i);
    }
}
