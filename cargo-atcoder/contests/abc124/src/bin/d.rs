use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };

    let a = {
        let mut a = if s[0] == '0' { vec![('1', 0)] } else { vec![] };
        let mut p = s[0];
        let mut c = 1;
        for s_i in s.iter().copied().skip(1) {
            if p != s_i {
                a.push((p, c));
                p = s_i;
                c = 1;
            } else {
                c += 1;
            }
        }
        a.push((p, c));
        if p == '0' {
            a.push(('1', 0));
        }
        a
    };

    let zero_count = a.iter().filter(|(s_i, _)| *s_i == '0').count();
    if zero_count <= k {
        println!("{}", n);
        return;
    }

    let m = a.len();
    let mut sum = a[0].1;
    let mut max = sum;
    let mut r = 1;
    for l in (0..m).step_by(2) {
        while r < m && (r + 1 - l) / 2 <= k {
            sum += a[r].1;
            sum += a[r + 1].1;
            r += 2;
        }
        max = max.max(sum);
        if r == l {
            r += 2;
        } else {
            sum -= a[l].1;
            if l + 1 < m {
                sum -= a[l + 1].1;
            }
        }
    }
    let ans = max;
    println!("{}", ans);
}
