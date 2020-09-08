use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    let s = n % 60;
    n /= 60;
    let m = n % 60;
    n /= 60;
    let h = n;
    println!("{:02}:{:02}:{:02}", h, m, s);
}
