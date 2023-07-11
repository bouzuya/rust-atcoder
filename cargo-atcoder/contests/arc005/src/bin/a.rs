use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    };
    let words = vec!["TAKAHASHIKUN", "Takahashikun", "takahashikun"];
    let mut count = 0;
    for (i, w_i) in w.iter().enumerate() {
        let w_i = if i == w.len() - 1 {
            &w_i[..w_i.len() - 1]
        } else {
            &w_i[..]
        };
        if words.contains(&w_i) {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
