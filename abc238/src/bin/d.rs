use proconio::input;

fn f(a: usize, s: usize) -> bool {
    let bits = 60;
    let mask = (1_usize << bits) - 1;
    let max = a + mask;
    let min = a + a;
    if !(min..=max).contains(&s) {
        return false;
    }
    let mut x = mask << 1 | 1;
    for i in 0..=bits {
        let b = 1 << (bits - i);
        if (a & b) != 0 {
            continue;
        }
        x ^= b;
        match (x + a).cmp(&s) {
            std::cmp::Ordering::Less => x ^= b,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => {
                // do nothing
            }
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
