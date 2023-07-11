use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let mut max = 1_usize;
    for b in 1..=x {
        for p in 2..=1_000 {
            match b.checked_pow(p as u32) {
                Some(a) => {
                    if a > x {
                        break;
                    } else {
                        max = max.max(a);
                    }
                }
                None => break,
            }
        }
    }
    let ans = max;
    println!("{}", ans);
}
