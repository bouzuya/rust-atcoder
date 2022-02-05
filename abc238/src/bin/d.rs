use proconio::input;

fn f(a: usize, s: usize) -> bool {
    let p = 60;
    let mask = (1_usize << p) - 1;
    let max = a + mask;
    let min = a + a;
    if !(min..=max).contains(&s) {
        return false;
    }
    let mut x = mask << 1 | 1;
    for i in 0..=p {
        let b = 1 << (p - i);
        if (a & b) != 0 {
            continue;
        }
        x ^= b;
        if x + a == s {
            return true;
        } else if x + a < s {
            x ^= b;
        }
    }

    false
}

fn main() {
    input! {
        t: usize,
        r#as: [(usize, usize); t],
    };
    for (a, s) in r#as {
        let ans = f(a, s);
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
