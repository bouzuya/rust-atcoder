use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mod_p = 998_244_353;

    let mut count = 0_usize;
    for q in 1.. {
        if q * q > n {
            break;
        }
        count += (n / q - (q - 1) + 1) / 2;
        count %= mod_p;
    }

    let ans = count;
    println!("{}", ans);
}
