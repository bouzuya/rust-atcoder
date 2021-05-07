use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    if n == 0 {
        println!("{}", 0);
        return;
    }
    let mut digits = vec![];
    let mut m = n;
    for k in 1.. {
        if m == 0 {
            break;
        }
        digits.push(if m % 2_i64.pow(k) == 0 {
            0
        } else {
            m -= (-2_i64).pow(k - 1);
            1
        });
    }
    digits.reverse();
    let ans = digits.iter().map(|&c| format!("{}", c)).collect::<String>();
    println!("{}", ans);
}
