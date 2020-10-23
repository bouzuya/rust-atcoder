use proconio::input;
use proconio::marker::Chars;

fn f(s: &Vec<char>, t: &Vec<char>) -> bool {
    for s_i in s.windows(t.len()) {
        if s_i
            .iter()
            .zip(t.iter())
            .all(|(&s_j, &t_j)| t_j == '.' || s_j == t_j)
        {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        s: Chars,
    };
    let chars = "abcdefghijklmnopqrstuvwxyz.".chars().collect::<Vec<char>>();
    let mut count = 0;
    for &c1 in chars.iter() {
        let t = vec![c1];
        if f(&s, &t) {
            count += 1;
        }
    }
    for &c1 in chars.iter() {
        for &c2 in chars.iter() {
            let t = vec![c1, c2];
            if f(&s, &t) {
                count += 1;
            }
        }
    }
    for &c1 in chars.iter() {
        for &c2 in chars.iter() {
            for &c3 in chars.iter() {
                let t = vec![c1, c2, c3];
                if f(&s, &t) {
                    count += 1;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
