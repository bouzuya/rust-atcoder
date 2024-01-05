use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut x = vec![vec![]];
    for c in s {
        match c {
            '(' => x.push(vec![]),
            ')' => {
                let t = x.pop().unwrap();
                x.last_mut().unwrap().extend(t.iter().copied());
                x.last_mut().unwrap().extend(t.iter().rev().copied());
            }
            _ => x.last_mut().unwrap().push(c),
        }
    }
    let ans = x[0].iter().collect::<String>();
    println!("{}", ans);
}
