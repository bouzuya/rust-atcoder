use proconio::input;

fn main() {
    input! {
        p: usize,
    };
    let mut fib = vec![1, 1];
    for i in 0..=p * p + 2 {
        fib.push((fib[i] + fib[i + 1]) % p);
    }
    for (i, &x) in fib.iter().enumerate() {
        if x % p == 0 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
