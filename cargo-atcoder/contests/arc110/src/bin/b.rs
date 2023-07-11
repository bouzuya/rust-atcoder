use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut t: Chars,
    };
    let count = 10_000_000_000;
    let ans = if n == 1 {
        match t[0] {
            '0' => count,
            '1' => count * 2,
            _ => unreachable!(),
        }
    } else if n == 2 {
        match (t[0], t[1]) {
            ('0', '0') => 0,
            ('0', '1') => count - 1, // 110_110
            ('1', '0') => count,     // 110
            ('1', '1') => count,     // 110
            _ => unreachable!(),
        }
    } else if n == 3 {
        match (t[0], t[1], t[2]) {
            ('0', '0', _) => 0,
            ('0', '1', '0') => 0,
            ('0', '1', '1') => count - 1, // 110_110
            ('1', '0', '0') => 0,
            ('1', '0', '1') => count - 1, // 110_110
            ('1', '1', '0') => count,     // 110
            ('1', '1', '1') => 0,
            _ => unreachable!(),
        }
    } else {
        let t = match (t[0], t[1]) {
            ('0', _) => {
                let mut x = vec!['1', '1'];
                x.append(&mut t);
                x
            }
            ('1', '0') => {
                let mut x = vec!['1'];
                x.append(&mut t);
                x
            }
            ('1', '1') => t,
            _ => unreachable!(),
        };
        let n = t.len();
        let t = match (t[n - 2], t[n - 1]) {
            (_, '0') => t,
            ('1', '1') => {
                let mut x = t.clone();
                x.push('0');
                x
            }
            ('0', '1') => {
                let mut x = t.clone();
                x.append(&mut vec!['1', '0']);
                x
            }
            _ => unreachable!(),
        };
        let n = t.len();
        if n % 3 != 0 {
            println!("0");
            return;
        }
        for i in 0..n {
            let c = match i % 3 {
                0 | 1 => '1',
                2 => '0',
                _ => unreachable!(),
            };
            if t[i] != c {
                println!("0");
                return;
            }
        }
        count + 1 - n / 3
    };
    println!("{}", ans);
}
