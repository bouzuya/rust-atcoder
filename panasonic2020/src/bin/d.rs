use proconio::input;

fn f(s: &mut String, n: usize, max_char: u8) {
    if s.len() == n {
        println!("{}", s);
        return;
    }
    for c in b'a'..=max_char {
        s.push(c as char);
        f(s, n, max_char + if c == max_char { 1 } else { 0 });
        s.pop();
    }
}

fn main() {
    input! {
        n: usize
    };
    f(&mut String::new(), n, b'a');
}
