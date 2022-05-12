use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for x in 1.. {
        let y = x * 108 / 100;
        if y > n {
            println!(":(");
            return;
        }
        if y == n {
            println!("{}", x);
            return;
        }
    }
}
