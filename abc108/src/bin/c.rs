use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut count = vec![0_usize; k];
    for x in 1..=n {
        count[x % k] += 1;
    }

    let mut sum = 0_usize;
    for a in 0..k {
        let b = (k - a) % k;
        let c = b;
        if (b + c) % k == 0 {
            sum += count[a] * count[b] * count[c]
        }
    }

    let ans = sum;
    println!("{}", ans);
}
