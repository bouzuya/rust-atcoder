use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeMap;

fn map(s: &Vec<char>) -> BTreeMap<char, usize> {
    let mut m = BTreeMap::new();
    for &c in s.iter() {
        *m.entry(c).or_insert(0) += 1;
    }
    m
}

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    };
    let n = s1.len();
    let mut p1 = (0, 0);
    let mut p2 = (0, 0);
    let map1 = map(&s1);
    let map2 = map(&s2);
    let map3 = map(&s3);
    for i in 0..26 {
        let c = ('A' as u8 + i as u8) as char;
        let &c1 = map1.get(&c).unwrap_or(&0);
        let &c2 = map2.get(&c).unwrap_or(&0);
        let &c3 = map3.get(&c).unwrap_or(&0);
        if c1 + c2 < c3 {
            println!("NO");
            return;
        }
        if c1 + c2 == c3 {
            p1.0 += c1;
            p2.0 += c2;
            if p1.0 > n / 2 || p2.0 > n / 2 {
                println!("NO");
                return;
            }
            continue;
        }
        assert!(c1 + c2 > c3);
        if c3 == 0 {
            p1.1 += c1;
            p2.1 += c2;
            if p1.1 > n / 2 || p2.1 > n / 2 {
                println!("NO");
                return;
            }
            continue;
        }
        assert!(c3 > 0);
        if c1 == 0 {
            p2.0 += c3;
            p2.1 += c2 - c3;
            continue;
        }
        assert!(c1 > 0);
        if c2 == 0 {
            p1.0 += c3;
            p1.1 += c1 - c3;
            continue;
        }
        assert!(c2 > 0);
        let l1 = c1.saturating_sub(c3);
        let l2 = c2.saturating_sub(c3);
        p1.1 += l1;
        p2.1 += l2;
        if p1.1 > n / 2 || p2.1 > n / 2 {
            println!("NO");
            return;
        }
        let c1 = c1 - l1;
        let c2 = c2 - l2;
        assert!(c1 <= c3 && c2 <= c3);
        p1.0 += c3 - c2;
        p2.0 += c3 - c1;
        if p1.0 > n / 2 || p2.0 > n / 2 {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
