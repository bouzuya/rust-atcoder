use proconio::input;

fn main() {
    input! {
        s: String,
    };
    // 012345678
    // AKIHABARA
    let ans = match s.find("KIH") {
        Some(i) if i == 0 || i == 1 => {
            let s = if i == 0 { format!("A{}", s) } else { s };
            match s.find("B") {
                Some(i) if i == 4 || i == 5 => {
                    let s = if i == 4 {
                        format!(
                            "{}A{}",
                            s.chars().take(4).collect::<String>(),
                            s.chars().skip(4).collect::<String>()
                        )
                    } else {
                        s
                    };
                    match s.find("R") {
                        Some(i) if i == 6 || i == 7 => {
                            let s = if i == 6 {
                                format!(
                                    "{}A{}",
                                    s.chars().take(6).collect::<String>(),
                                    s.chars().skip(6).collect::<String>()
                                )
                            } else {
                                s
                            };
                            let s = if s.len() == 8 { format!("{}A", s) } else { s };
                            s == "AKIHABARA"
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
        _ => false,
    };
    println!("{}", if ans { "YES" } else { "NO" });
}
