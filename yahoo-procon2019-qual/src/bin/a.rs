use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut count = 0;
    for _ in (1..=n).step_by(2) {
        count += 1;
        if count >= k {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
