use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut count = 0_usize;
    for a in 1..=n {
        if a * a * a > n {
            break;
        }
        for b in a..=n / a {
            if a * b * b > n {
                break;
            }
            let ab = a * b;

            let c = n / ab;
            if b <= c && ab * c <= n {
                count += c - b + 1;
            }
        }
    }

    let ans = count;
    println!("{}", ans);
}
