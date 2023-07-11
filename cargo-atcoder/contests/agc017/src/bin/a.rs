use proconio::input;

fn choose(n: usize, r: usize) -> usize {
    if n < r {
        return 0;
    }
    if r == 0 {
        return 1;
    }
    if n == 0 {
        return 0;
    }
    let mut m = 1;
    for i in 0..r {
        m *= n - i;
        m /= i + 1;
    }
    m
}

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [i64; n],
    };
    let e = a.iter().filter(|&a_i| a_i % 2 == 0).count();
    let o = n - e;
    let mut sum = 0_usize;
    if p == 0 {
        for i in 0..=n {
            let mut count = 0;
            count += choose(e, i);
            for j in (2..=i).step_by(2) {
                count += choose(e, i - j) * choose(o, j);
            }
            sum += count;
        }
    } else {
        for i in 1..=n {
            let mut count = 0;
            for j in (1..=i).step_by(2) {
                count += choose(e, i - j) * choose(o, j);
            }
            sum += count;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
