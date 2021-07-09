use std::cmp;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: Chars
    };
    let num = |c: char| -> usize { (c as u8 - b'a' + 1) as usize };
    let chr = |n: usize| -> char { (n as u8 + b'a' - 1) as char };
    let d = c.iter().copied().map(num).collect::<Vec<usize>>();
    let sum = d.iter().sum::<usize>();
    if sum == 1 || sum == 20 * 26 {
        println!("NO");
        return;
    }
    if c.len() == 1 {
        assert!((2..=26).contains(&sum));
        println!("a{}", chr(sum - 1));
    } else if 2 <= c.len() && c.len() < 20 {
        assert!((2..=26 * c.len()).contains(&sum));
        let mut ds = vec![];
        let mut x = sum;
        while x > 0 {
            ds.push(cmp::min(x, 26));
            x = x.saturating_sub(26);
        }
        let ans = if d != ds {
            ds.iter().copied().map(chr).collect::<Vec<char>>()
        } else {
            let l = ds.len();
            if ds.last() != Some(&1) {
                ds[l - 1] -= 1;
            } else {
                ds[l - 2] -= 1;
            }
            ds.push(1);
            ds.iter().copied().map(chr).collect::<Vec<char>>()
        };
        println!("{}", ans.iter().collect::<String>());
    } else {
        assert!(c.len() == 20);
        let mut ds = vec![];
        let mut x = sum;
        while x > 0 {
            ds.push(cmp::min(x, 26));
            x = x.saturating_sub(26);
        }
        let ans = if d != ds {
            ds.iter().copied().map(chr).collect::<Vec<char>>()
        } else {
            let l = ds.len();
            ds[l - 2] -= 1;
            ds[l - 1] += 1;
            ds.iter().copied().map(chr).collect::<Vec<char>>()
        };
        println!("{}", ans.iter().collect::<String>());
    }
}
