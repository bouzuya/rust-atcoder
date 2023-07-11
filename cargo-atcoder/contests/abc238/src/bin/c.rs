use proconio::input;

fn f(a_1: u128, a_n: u128) -> u128 {
    let n = a_n - a_1 + 1;
    let a_n = a_1 + (n - 1);
    (a_1 + a_n) * n / 2
}

fn main() {
    input! {
        n: u128,
    };

    let mod_p = 998_244_353_u128;
    let mut sum = 0_u128;
    for i in 0..n.to_string().len() - 1 {
        let l = 10_u128.pow(i as u32);
        let r = 10_u128.pow(i as u32 + 1) - 1;
        sum += f(l, r) - (l - 1) * (r - l + 1);
        sum %= mod_p;
    }

    let l = 10_u128.pow(n.to_string().len() as u32 - 1);
    let r = n;
    sum += f(l, r) - (l - 1) * (r - l + 1);
    sum %= mod_p;

    let ans = sum;
    println!("{}", ans);
}
