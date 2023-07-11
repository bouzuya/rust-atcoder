use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: usize,
    };
    for i in 0..n {
        print!(
            "{}{}",
            if i < k {
                s
            } else {
                if s + 1 > 1_000_000_000 {
                    1
                } else {
                    s + 1
                }
            },
            if i == n - 1 { "\n" } else { " " }
        );
    }
}
