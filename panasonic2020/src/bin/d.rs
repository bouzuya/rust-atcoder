use proconio::input;

fn f(s: &mut String, n: usize, mb: u8) {
    if s.len() == n {
        println!("{}", s);
        return;
    }
    for b in b'a'..=mb {
        s.push(b as char);
        f(s, n, mb + if b == mb { 1 } else { 0 });
        s.pop();
    }
}

fn main() {
    input! { n: usize }
    f(&mut String::new(), n, b'a');
}
