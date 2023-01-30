use proconio::input;

fn f(s: &mut String, n: usize, c: u8) {
    if s.len() == n {
        println!("{}", s);
        return;
    }
    for i in b'a'..=c {
        s.push(i as char);
        f(s, n, c + if i == c { 1 } else { 0 });
        s.pop();
    }
}

fn main() {
    input! {
        n: usize,
    };
    f(&mut String::new(), n, b'a');
}
