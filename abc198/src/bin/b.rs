use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for i in 0..10 {
        let s = format!("{}{}", "0".repeat(i), n)
            .chars()
            .collect::<Vec<char>>();
        let l = s.len();
        let mut ok = true;
        for j in 0..l / 2 {
            if s[j] != s[l - 1 - j] {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
