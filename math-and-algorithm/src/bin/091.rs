use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let mut count = 0;
    for a in 1..=n {
        for b in a + 1..=n {
            for c in b + 1..=n {
                if a + b + c == x {
                    count += 1;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
