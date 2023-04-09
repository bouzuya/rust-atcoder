use proconio::input;

fn f(count: &mut usize, a: usize, b: usize) {
    if a == b || a == 0 || b == 0 {
        *count = (*count).saturating_sub(1);
    } else if a > b {
        *count += a / b;
        f(count, a % b, b);
    } else if a < b {
        *count += b / a;
        f(count, a, b % a);
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut count = 0_usize;
    f(&mut count, a, b);
    let ans = count;
    println!("{}", ans);
}
