use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort_by_key(|&a_i| -a_i);
    let mut s_a = 0;
    let mut s_b = 0;
    for (i, &a_i) in a.iter().enumerate() {
        if i % 2 == 0 {
            s_a += a_i;
        } else {
            s_b += a_i;
        }
    }
    let ans = s_a - s_b;
    println!("{}", ans);
}
