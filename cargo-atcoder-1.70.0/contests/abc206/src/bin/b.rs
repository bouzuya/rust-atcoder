use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut sum = 0_usize;
    for i in 1.. {
        sum += i;
        if sum >= n {
            println!("{}", i);
            return;
        }
    }
}
