use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    };
    let sum_a = a.iter().sum::<i64>();
    let sum_b = b.iter().sum::<i64>();
    if sum_a < sum_b {
        println!("-1");
        return;
    }
    let mut count_minus = 0;
    let mut sum_minus = 0;
    let mut plus = vec![];
    for i in 0..n {
        let d = a[i] - b[i];
        if d > 0 {
            plus.push(d);
        } else if d == 0 {
            // do nothing
        } else {
            count_minus += 1;
            sum_minus += d;
        }
    }
    plus.sort();
    let count_plus = plus.len();
    while sum_minus < 0 {
        sum_minus += plus.pop().unwrap();
    }
    let ans = count_minus + (count_plus - plus.len());
    println!("{}", ans);
}
