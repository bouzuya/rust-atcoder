use std::cmp::min;

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
    let cv = s.chars().collect::<Vec<_>>();
    let mut ans = true;
    let mut i = 0;
    let l = cv.len();
    while i < l {
        match cv[i] {
            'd' => {
                if cv[i..min(i + 5, l)].iter().collect::<String>() != "dream" {
                    ans = false;
                    break;
                }
                i += 5;
                if i >= l {
                    break;
                }
                match cv[i] {
                    'd' => continue,
                    'e' => {
                        let t = cv[i..min(i + 3, l)].iter().collect::<String>();
                        if t == "er" {
                            break;
                        } else if t == "era" {
                            continue;
                        } else if t == "erd" || t == "ere" {
                            i += 2;
                            continue;
                        } else {
                            ans = false;
                            break;
                        }
                    }
                    _ => {
                        ans = false;
                        break;
                    }
                }
            }
            'e' => {
                if cv[i..min(i + 5, l)].iter().collect::<String>() != "erase".chars() {
                    ans = false;
                    break;
                }
                i += 5;
                if i >= l {
                    break;
                }
                if cv[i] == 'r' {
                    i += 1;
                }
            }
            _ => {
                ans = false;
                break;
            }
        }
    }
    println!("{}", if ans { "YES" } else { "NO" });
}
