use proconio::input;

fn main() {
    input! {
        y: usize,
    };
    for i in 0.. {
        if (y + i + 2) % 4 == 0 {
            println!("{}", y + i);
            return;
        }
    }
}
