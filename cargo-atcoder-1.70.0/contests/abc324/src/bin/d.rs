use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
    };

    if s == vec!['0'] {
        println!("1");
        return;
    }

    s.sort();
    s.reverse();
    let max_s = s
        .iter()
        .copied()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let (sd, sz) = {
        let mut d = vec![0_usize; 9];
        let mut z = 0_usize;
        for c in s {
            if c == '0' {
                z += 1;
            } else {
                d[(c as u8 - b'1') as usize] += 1;
            }
        }
        (d, z)
    };

    let mut count = 0_usize;
    for i in 1.. {
        let x = i * i;
        if x > max_s {
            break;
        }

        let mut d = vec![0_usize; 9];
        let mut z = 0_usize;
        for c in x.to_string().chars() {
            if c == '0' {
                z += 1;
            } else {
                d[(c as u8 - b'1') as usize] += 1;
            }
        }

        if d == sd && z <= sz {
            count += 1;
        }
    }

    let ans = count;
    println!("{}", ans);
}
