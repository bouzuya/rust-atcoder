use std::cmp::max;

fn read<T: std::str::FromStr>(
    stdin_lock: &mut std::io::StdinLock,
    buf: &mut Vec<u8>,
    delimiter: u8,
) -> T {
    buf.clear();
    let l = std::io::BufRead::read_until(stdin_lock, delimiter, buf).unwrap();
    buf.truncate(l - 1); // remove delimiter
    let s = unsafe { std::str::from_utf8_unchecked(&buf) };
    s.parse().unwrap_or_else(|_| panic!("read"))
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cv: Vec<char> = s.chars().collect();

    let n = s.len();
    let mut xv = vec![1i32; n];
    for i in 0..n - 1 {
        if cv[i] == 'R' && cv[i + 1] == 'R' {
            xv[i + 1] += xv[i];
            xv[i] = 0;
        }
    }
    for i in 0..n - 1 {
        let j = n - i - 1;
        if cv[j - 1] == 'L' && cv[j] == 'L' {
            xv[j - 1] += xv[j];
            xv[j] = 0;
        }
    }

    let mut yv = vec![0i32; n];
    let mut i = 0;
    while i < n - 1 {
        if xv[i] != 0 && xv[i + 1] != 0 {
            let y = xv[i] + xv[i + 1];
            if y % 2 == 0 {
                yv[i] = y / 2;
                yv[i + 1] = y / 2;
            } else {
                if max(xv[i], xv[i + 1]) % 2 == 0 {
                    if xv[i] > xv[i + 1] {
                        yv[i] = y / 2;
                        yv[i + 1] = y / 2 + 1;
                    } else {
                        yv[i] = y / 2 + 1;
                        yv[i + 1] = y / 2;
                    }
                } else {
                    if xv[i] > xv[i + 1] {
                        yv[i] = y / 2 + 1;
                        yv[i + 1] = y / 2;
                    } else {
                        yv[i] = y / 2;
                        yv[i + 1] = y / 2 + 1;
                    }
                }
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    for i in 0..n - 1 {
        print!("{} ", yv[i]);
    }
    println!("{}", yv[n - 1]);
}
