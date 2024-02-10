use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        d: usize,
    };
    let mut cur = a;
    loop {
        println!("{}", cur);
        if cur == b {
            break;
        }
        cur += d;
    }
}
