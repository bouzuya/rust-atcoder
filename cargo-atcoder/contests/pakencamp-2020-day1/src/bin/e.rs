use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    if x == y {
        if x == 0 {
            assert!(y == 0);
            println!("1 1");
        } else {
            println!("-1");
        }
    } else if x < y {
        println!("{} {}", x + 2 * y, y);
    } else if x > y {
        println!("{} {}", x, 2 * x + y);
    }
}
