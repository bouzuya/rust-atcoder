use proconio::input;

fn main() {
    input! {
        t: i64,
        n: usize,
        a: [i64; n],
        m: usize,
        b: [i64; m],
    };
    let mut i_b = 0;
    for &a_i in a.iter() {
        if i_b < b.len() && (a_i..=a_i + t).contains(&b[i_b]) {
            i_b += 1;
        }
    }
    println!("{}", if i_b == b.len() { "yes" } else { "no" });
}
