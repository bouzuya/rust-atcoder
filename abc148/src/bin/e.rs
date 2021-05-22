use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    if n % 2 != 0 {
        println!("0");
        return;
    }

    assert!(n % 2 == 0);
    let mut count = 0;
    let mut d = 10;
    while d <= n {
        count += n / d;
        d *= 5;
    }
    println!("{}", count);
}
