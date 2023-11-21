use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    };
    let mut i = s;
    while i != t {
        if i == x {
            println!("Yes");
            return;
        }
        i += 1;
        i %= 24;
    }
    println!("No");
}
