use proconio::input;

fn main() {
    input! {
        n: usize
    };
    for i in 1.. {
        if i * i > n {
            println!("{}", (i - 1) * (i - 1));
            break;
        }
    }
}
