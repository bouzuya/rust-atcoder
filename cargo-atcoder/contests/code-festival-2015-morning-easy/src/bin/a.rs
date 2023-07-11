use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for x in 1.. {
        if x * x >= n {
            println!("{}", x * x - n);
            break;
        }
    }
}
