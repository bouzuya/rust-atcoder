use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for x in 0..=n / 4 {
        for y in 0..=n / 7 {
            if x * 4 + y * 7 == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
