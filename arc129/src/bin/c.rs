use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut c = vec![0; 7];
    let mut sum = 0;
    for i in 0..7 {
        for c_v in 1_usize.. {
            if c_v * (c_v - 1) / 2 > n - sum {
                c[i] = c_v - 1;
                sum += c[i] * (c[i] - 1) / 2;
                break;
            }
        }
        if n == sum {
            break;
        }
    }
    c[0] -= 1;

    let mut ans = vec![];
    let mut base = 1_usize;
    let mut prev = 0_usize;
    for (i, c_i) in c.iter().copied().enumerate() {
        for _ in 0..c_i {
            for j in 1..=7 {
                if (j * base + prev) % 7 == i {
                    ans.push((j as u8 + b'0') as char);
                    prev = i;
                    base *= 10;
                    base %= 7;
                    break;
                }
            }
        }
    }
    ans.reverse();
    let ans = ans.into_iter().collect::<String>();
    assert_eq!(check(&ans), n);
    println!("{}", ans);
}

fn check(s: &str) -> usize {
    let s = s.chars().rev().collect::<Vec<char>>();
    let mut b = 1_usize;
    let mut t = vec![];
    for s_i in s {
        let d = (s_i as u8 - b'0') as usize;
        t.push((d * b) % 7);
        b *= 10;
        b %= 7;
    }
    let cs = std::iter::once(0)
        .chain(t.iter().rev().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let mut c = vec![0; 7];
    for x in cs {
        c[x % 7] += 1;
    }
    let mut sum = 0_usize;
    for c_i in c.iter().copied() {
        if c_i == 0 {
            continue;
        }
        sum += c_i * (c_i - 1) / 2;
    }
    sum
}
