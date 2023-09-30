use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    };
    if !s.contains("ABC") {
        println!("-1");
        return;
    }
    let chars = s.chars().collect::<Vec<char>>();
    for i in 0..n {
        if chars[i] != 'A' {
            continue;
        }
        let mut ok = true;
        for j in 0..3 {
            if chars[i + j] != ['A', 'B', 'C'][j] {
                ok = false;
                break;
            }
        }
        if ok {
            println!("{}", i + 1);
            return;
        }
    }
}
