use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    };
    let mut sum = a;
    for i in 0.. {
        if sum >= b {
            println!("{}", i);
            return;
        }
        sum *= k;
    }
}
