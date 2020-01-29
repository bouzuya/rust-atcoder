use std::cmp::max;
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
    let mut map: HashMap<String, i32> = HashMap::new();
    for _ in 0..n {
        let k: String = read(&mut stdin_lock, &mut buf, b'\n');
        let v: i32 = *map.get(&k).unwrap_or(&0);
        map.insert(k, v + 1);
    }
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    for _ in 0..m {
        let k: String = read(&mut stdin_lock, &mut buf, b'\n');
        match map.get(&k) {
            Some(&v) => map.insert(k, v - 1),
            None => None,
        };
    }

    let max_value = map.values().max().unwrap();
    let ans = max(*max_value, 0);
    println!("{}", ans);
}
