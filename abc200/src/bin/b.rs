use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut n = n;
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = format!("{}200", n).parse::<usize>().unwrap();
        }
    }

    let ans = n;
    println!("{}", ans);
}
