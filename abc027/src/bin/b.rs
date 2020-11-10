use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let sum = a.iter().sum::<i64>();
    let avg = sum / n as i64;
    if avg * n as i64 != sum {
        println!("-1");
        return;
    }
    let mut count = 0;
    let mut over = a[0] - avg;
    for &a_i in a.iter().skip(1) {
        if over != 0 {
            count += 1;
        }
        let c = over + a_i;
        if c == avg {
            over = 0;
        } else if c > avg {
            over = c - avg;
        } else if c < avg {
            over = c - avg;
        } else {
            unreachable!();
        }
    }
    let ans = count;
    println!("{}", ans);
}
