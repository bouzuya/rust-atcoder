use proconio::input;

fn f(cv: Vec<char>, n: usize, max_char: u8) {
    if cv.len() == n {
        let s: String = cv.iter().collect();
        println!("{}", s);
        return;
    }
    for c in b'a'..=max_char {
        let mut t = cv.clone();
        t.push(c as char);
        f(t, n, max_char + if c == max_char { 1 } else { 0 });
    }
}

fn main() {
    input! {
        n: usize
    };
    f(vec![], n, b'a');
}
