use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let t: Vec<&str> = vec!["AC", "WA", "TLE", "RE"];
    let mut c = vec![0; 4];
    for s_i in s.iter() {
        match t.iter().position(|t_i| t_i == s_i) {
            Some(i) => c[i] += 1,
            None => unreachable!(),
        }
    }
    for (t_i, c_i) in t.iter().zip(c.iter()) {
        println!("{} x {}", t_i, c_i);
    }
}
