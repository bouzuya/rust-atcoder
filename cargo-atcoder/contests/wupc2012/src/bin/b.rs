use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let mut t = vec![];
    for (i, s_i) in s.iter().enumerate() {
        for (j, s_j) in s.iter().enumerate() {
            if i == j {
                continue;
            }
            t.push(format!("{}{}", s_i, s_j));
        }
    }
    t.sort();

    let ans = &t[0];
    println!("{}", ans);
}
