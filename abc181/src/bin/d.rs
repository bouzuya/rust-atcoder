use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    };
    let mut ans = vec![];
    for i in 1.. {
        if 8 * i >= 1000 {
            break;
        }
        let mut map = HashMap::new();
        let s = format!("{:03}", 8 * i);
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        ans.push(map);
    }
    if s.len() == 1 {
        let ans = (s[0] as u8 - '0' as u8) == 8;
        println!("{}", if ans { "Yes" } else { "No" });
        return;
    } else if s.len() == 2 {
        let xs = vec![format!("{}{}", s[0], s[1]), format!("{}{}", s[1], s[0])];
        for x in xs {
            if x.parse::<i64>().unwrap() % 8 == 0 {
                println!("Yes");
                return;
            }
        }
        println!("No");
        return;
    } else if s.len() == 3 {
        let xs = vec![
            format!("{}{}{}", s[0], s[1], s[2]),
            format!("{}{}{}", s[0], s[2], s[1]),
            format!("{}{}{}", s[1], s[0], s[2]),
            format!("{}{}{}", s[1], s[2], s[0]),
            format!("{}{}{}", s[2], s[0], s[1]),
            format!("{}{}{}", s[2], s[1], s[0]),
        ];
        for x in xs {
            if x.parse::<i64>().unwrap() % 8 == 0 {
                println!("Yes");
                return;
            }
        }
        println!("No");
        return;
    }

    let mut map = HashMap::new();
    for c in s {
        *map.entry(c).or_insert(0) += 1;
    }
    for a in ans {
        if a.iter().all(|(k, v)| match map.get(k) {
            None => false,
            Some(v1) => v1 >= v,
        }) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
