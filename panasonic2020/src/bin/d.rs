use proconio::input;

fn dfs(s: &mut String, n: usize, mb: u8) {
    if s.len() == n {
        println!("{}", s);
        return;
    }
    for b in b'a'..=mb {
        s.push(b as char);
        dfs(s, n, mb + if b == mb { 1 } else { 0 });
        s.pop();
    }
}

fn main() {
    input! { n: usize }
    dfs(&mut String::new(), n, b'a');
}
