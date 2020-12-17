use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars,
    };
    let mut stack = vec![];
    for c in s {
        stack.push(c);
        let l = stack.len();
        if l >= 3 && stack[l - 3] == 'f' && stack[l - 2] == 'o' && stack[l - 1] == 'x' {
            stack.pop(); // x
            stack.pop(); // o
            stack.pop(); // f
        }
    }
    let ans = stack.len();
    println!("{}", ans);
}
