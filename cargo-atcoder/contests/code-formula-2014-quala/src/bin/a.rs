use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for x in 1..=100 {
        if x * x * x == n {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
