use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut stack = vec![vec![]];
    for &s_i in s.iter() {
        match s_i {
            '(' => {
                stack.push(vec![]);
            }
            ')' => {
                let mut a = stack.pop().unwrap();
                let mut reva = a.iter().rev().map(|&c| c).collect();
                let mut b = stack.pop().unwrap();
                b.append(&mut a);
                b.append(&mut reva);
                stack.push(b);
            }
            ch => {
                let mut a = stack.pop().unwrap();
                a.push(ch);
                stack.push(a);
            }
        }
    }
    println!("{}", stack.pop().unwrap().into_iter().collect::<String>());
}
