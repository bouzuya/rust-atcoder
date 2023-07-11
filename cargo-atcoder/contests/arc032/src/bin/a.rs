use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let sum = (1..=n).sum::<usize>();
    let mut sieve = vec![true; sum + 1];
    sieve[0] = false;
    sieve[1] = false;
    for x in 2..=sum {
        if sieve[x] && sum % x == 0 {
            for y in (x + x..=sum).step_by(x) {
                sieve[y] = false;
            }
        }
    }
    let ans = sieve[sum];
    println!("{}", if ans { "WANWAN" } else { "BOWWOW" });
}
