use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let s_a = a.to_string().repeat(b);
    let s_b = b.to_string().repeat(a);
    let ans = if s_a < s_b { s_a } else { s_b };
    println!("{}", ans);
}
