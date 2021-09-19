use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut sum = 0_usize;
    for a in 1..=9 {
        for b in 1..=9 {
            sum += a * b;
        }
    }

    let mut ans = vec![];
    for a in 1..=9 {
        for b in 1..=9 {
            if (sum - (a * b)) == n {
                ans.push((a, b));
            }
        }
    }

    for (a, b) in ans {
        println!("{} x {}", a, b);
    }
}
