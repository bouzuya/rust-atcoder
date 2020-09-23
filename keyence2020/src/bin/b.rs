use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xl: [(i64, i64); n],
    };
    xl.sort_by_key(|(x_i, l_i)| x_i - l_i);
    let mut c = 0;
    let mut r_p = xl[0].0 + xl[0].1;
    for &(x_i, l_i) in xl.iter().skip(1) {
        let (l_c, r_c) = (x_i - l_i, x_i + l_i);
        if l_c >= r_p {
            r_p = r_c;
        } else {
            c += 1;
            if r_c < r_p {
                r_p = r_c;
            }
        }
    }
    let ans = n - c;
    println!("{}", ans);
}
