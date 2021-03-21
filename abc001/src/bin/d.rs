use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        mut se: [Chars; n],
    };

    let mut se2 = vec![];
    for se_i in se.iter() {
        let s = &se_i[0..4];
        let e = &se_i[5..9];
        let s2 = format!("{}{}", s[0], s[1]).parse::<usize>().unwrap() * 60
            + format!(
                "{}{}",
                s[2],
                match s[3] {
                    '0' | '1' | '2' | '3' | '4' => '0',
                    '5' | '6' | '7' | '8' | '9' => '5',
                    _ => unreachable!("invalid char"),
                }
            )
            .parse::<usize>()
            .unwrap();
        let e2 = {
            let (e3, e2plus1) = match e[3] {
                '0' => ('0', false),
                '1' | '2' | '3' | '4' | '5' => ('5', false),
                '6' | '7' | '8' | '9' => ('0', true),
                _ => unreachable!("invalid char"),
            };
            let (e2, e1plus1) = if e2plus1 {
                if e[2] == '5' {
                    ('0', true)
                } else {
                    ((e[2] as u8 + 1) as char, false)
                }
            } else {
                (e[2], false)
            };
            let (e1, e0plus1) = if e1plus1 {
                if e[1] == '9' {
                    ('0', true)
                } else {
                    ((e[1] as u8 + 1) as char, false)
                }
            } else {
                (e[1], false)
            };
            let e0 = (e[0] as u8 + if e0plus1 { 1 } else { 0 }) as char;
            format!("{}{}", e0, e1).parse::<usize>().unwrap() * 60
                + format!("{}{}", e2, e3).parse::<usize>().unwrap()
        };
        se2.push((s2, e2));
    }

    se2.sort();

    let mut se3 = vec![];
    for (s, e) in se2.iter() {
        match se3.pop() {
            None => se3.push((s, e)),
            Some((s_p, e_p)) => {
                if e_p < s {
                    se3.push((s_p, e_p));
                    se3.push((s, e));
                } else {
                    se3.push((min(s_p, s), max(e_p, e)));
                }
            }
        }
    }

    for (s, e) in se3 {
        println!("{:02}{:02}-{:02}{:02}", s / 60, s % 60, e / 60, e % 60);
    }
}
