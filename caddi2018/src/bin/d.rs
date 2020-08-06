use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut ans = false;
    for &a_i in a.iter() {
        if a_i % 2 != 0 {
            ans |= true;
        }
    }
    println!("{}", if ans { "first" } else { "second" });
}
