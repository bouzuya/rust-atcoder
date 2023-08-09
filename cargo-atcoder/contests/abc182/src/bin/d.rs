use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut vec = 0_i64;
    let mut vec_max = 0_i64;
    let mut cur = 0_i64;
    let mut max = 0_i64;
    for a_i in a {
        vec += a_i;
        vec_max = vec_max.max(vec);
        max = max.max(cur + vec_max);
        cur += vec;
    }
    let ans = max;
    println!("{}", ans);
}
