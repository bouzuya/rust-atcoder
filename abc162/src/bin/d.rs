use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut rcv = vec![n; n]; // rcv[i] は i 以降の r の個数
    let mut gcv = vec![n; n];
    let mut bcv = vec![n; n];

    let mut rc = 0;
    let mut gc = 0;
    let mut bc = 0;
    for i in (0..n).rev() {
        match s[i] {
            'R' => rc += 1,
            'G' => gc += 1,
            'B' => bc += 1,
            _ => unreachable!(),
        }
        rcv[i] = rc;
        gcv[i] = gc;
        bcv[i] = bc;
    }

    // println!("{:?}", rcv);
    // println!("{:?}", gcv);
    // println!("{:?}", bcv);

    let mut ans = 0_usize;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            if s[i] == s[j] {
                continue;
            }
            if (s[i] == 'G' && s[j] == 'B') || (s[i] == 'B' && s[j] == 'G') {
                ans += rcv[j + 1]
                    - if j + (j - i) < n && s[j + (j - i)] == 'R' {
                        1
                    } else {
                        0
                    };
            } else if (s[i] == 'R' && s[j] == 'B') || (s[i] == 'B' && s[j] == 'R') {
                ans += gcv[j + 1]
                    - if j + (j - i) < n && s[j + (j - i)] == 'G' {
                        1
                    } else {
                        0
                    };
            } else if (s[i] == 'R' && s[j] == 'G') || (s[i] == 'G' && s[j] == 'R') {
                ans += bcv[j + 1]
                    - if j + (j - i) < n && s[j + (j - i)] == 'B' {
                        1
                    } else {
                        0
                    };
            } else {
                unreachable!();
            }
        }
    }

    println!("{}", ans);
}
