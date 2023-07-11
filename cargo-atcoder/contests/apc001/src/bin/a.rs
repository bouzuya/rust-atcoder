use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    if x % y == 0 {
        println!("-1");
        return;
    }
    for n in (x + x..).step_by(x) {
        if n % y != 0 {
            println!("{}", n);
            return;
        }
    }
}
