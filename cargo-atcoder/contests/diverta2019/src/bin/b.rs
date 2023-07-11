use proconio::input;

fn main() {
    input! {
        r: i64,
        g: i64,
        b: i64,
        n: i64,
    };
    let mut c = 0;
    for x in 0..=3000 {
        let n_r = x * r;
        if n_r > n {
            break;
        }
        for y in 0..=3000 {
            let n_g = y * g;
            if n_r + n_g > n {
                break;
            }
            let z = (n - (n_r + n_g)) / b;
            if !(0..=3000).contains(&z) {
                continue;
            }
            let n_b = z * b;
            if n_r + n_g + n_b == n {
                c += 1;
            }
        }
    }
    let ans = c;
    println!("{}", ans);
}
