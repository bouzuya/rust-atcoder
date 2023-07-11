use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("1");
        return;
    }

    // 1         =     1 =      1^2
    // 1+3       =     4 =      2^2
    // 1+3+5     =     9 =      3^2
    // 1+3+5+7   =    16 =      4^2
    // 1+3+5+7+9 =    25 =      5^2
    // ...       = 10^18 = (10^9)^2
    let mut ng = 1_usize;
    let mut ok = 1_000_000_000_usize;
    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;
        if n <= mid.pow(2) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let left = ok - 1;
    let right = ok;
    let len = right.pow(2) - left.pow(2);
    let index = n - left.pow(2) - 1;
    // 1 , 2 1 2 , 3 2 1 2 3 , 4 3 2 1 2 3 4
    // 1       4           9              16
    let ans = if index <= len / 2 {
        right - index
    } else {
        right - len / 2 + index - (len / 2)
    };
    println!("{}", ans);
}
