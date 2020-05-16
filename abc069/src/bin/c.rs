use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c4 = 0;
    for &a_i in a.iter() {
        if a_i % 4 == 0 {
            c4 += 1;
        } else if a_i % 2 == 0 {
            c2 += 1;
        } else {
            c1 += 1;
        }
    }
    let ans = c4 + 1 - c1 >= 0 && std::cmp::max(0, c2 % 2 - (c4 + 1 - c1)) == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
