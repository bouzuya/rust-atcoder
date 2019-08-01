use std::collections::{BinaryHeap, HashSet};

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
    let k_len: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let cv: Vec<char> = s.chars().collect();
    let mut heap = BinaryHeap::new();
    let mut set = HashSet::new();

    for i in 0..cv.len() {
        for j in i + 1..cv.len() + 1 {
            let t = &s[i..j];
            if set.contains(t) {
                continue;
            }
            set.insert(t);
            heap.push(t);
            if heap.len() > k_len {
                if t >= heap.pop().unwrap() {
                    break;
                }
            }
        }
    }

    let ans = heap.into_sorted_vec()[k_len - 1];
    println!("{}", ans);
}
