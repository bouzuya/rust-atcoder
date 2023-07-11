use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut x = 1;
    let mut d = true;
    while x <= 200 {
        for i in 1..=20 {
            let g = if d { i } else { 20 - (i - 1) };
            if x == n {
                println!("{}", g);
                return;
            }
            x += 1;
        }
        d = !d;
    }
}
