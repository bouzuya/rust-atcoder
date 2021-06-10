use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    let mut count = 0_usize;
    for a in 1..=k {
        for b in a..=k / a {
            for c in b..=k / (a * b) {
                if a * b * c <= k {
                    if a == b && b == c {
                        count += 1;
                    } else if a == b || b == c || a == c {
                        count += 3;
                    } else {
                        count += 6;
                    }
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
