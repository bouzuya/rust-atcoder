use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut count = 0_usize;
    for i in 0..=9999 {
        let x = format!("{:04}", i).chars().collect::<Vec<char>>();
        let mut ok = true;
        for (j, s_j) in s.iter().copied().enumerate() {
            match s_j {
                'o' => {
                    if x.contains(&((j as u8 + b'0') as char)) {
                        continue;
                    } else {
                        ok = false;
                        break;
                    }
                }
                'x' => {
                    if x.contains(&((j as u8 + b'0') as char)) {
                        ok = false;
                        break;
                    } else {
                        continue;
                    }
                }
                '?' => continue,
                _ => unreachable!(),
            }
        }
        if ok {
            count += 1;
        }
    }

    let ans = count;
    println!("{}", ans);
}
