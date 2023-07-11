use std::collections::BTreeSet;

fn read<T: std::str::FromStr>(
    stdin_lock: &mut std::io::StdinLock,
    buf: &mut Vec<u8>,
    delimiter: u8,
) -> T {
    buf.clear();
    let _ = std::io::BufRead::read_until(stdin_lock, delimiter, buf).unwrap();
    let s = unsafe { std::str::from_utf8_unchecked(&buf) };
    s.parse().unwrap_or_else(|_| panic!("read"))
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();

    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let s = s.trim_end().chars().collect::<Vec<char>>();

    let mut set = BTreeSet::new();
    let mut in_name = false;
    let mut name = vec![];
    for c in s.iter() {
        match c {
            '@' => {
                if in_name {
                    if !name.is_empty() {
                        set.insert(name.into_iter().collect::<String>());
                        name = vec![];
                    }
                }
                in_name = true;
            }
            ' ' => {
                if in_name {
                    in_name = false;
                    if !name.is_empty() {
                        set.insert(name.into_iter().collect::<String>());
                        name = vec![];
                    }
                }
            }
            'a'..='z' => {
                if in_name {
                    name.push(c);
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    if in_name {
        if !name.is_empty() {
            set.insert(name.into_iter().collect::<String>());
        }
    }
    for s in set {
        println!("{}", s);
    }
}
