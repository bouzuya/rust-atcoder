use proconio::input;

fn main() {
    input! {
        s: [String; 3],
    };
    let all = vec!["ABC", "ARC", "AGC", "AHC"];
    for a in all {
        match s.iter().find(|s_i| s_i.as_str() == a) {
            Some(_) => continue,
            None => {
                println!("{}", a);
                return;
            }
        }
    }
}
