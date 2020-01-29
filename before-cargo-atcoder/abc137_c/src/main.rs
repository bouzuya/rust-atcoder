use std::collections::HashMap;

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
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut sv: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        sv.push(
            read::<String>(&mut stdin_lock, &mut buf, b'\n')
                .chars()
                .collect(),
        );
    }

    for i in 0..n {
        sv[i].sort();
    }

    sv.reverse();

    let mut cv = vec![0i64; n];
    let mut map = HashMap::new();
    for i in 0..n {
        let s = sv[i].iter().map(|&c| c).collect::<String>();
        let count = match map.get(&s) {
            None => 0,
            Some(&c) => c,
        };
        cv[i] = count;
        map.insert(s, count + 1);
    }

    let ans: i64 = cv.iter().sum();
    println!("{}", ans);
}
