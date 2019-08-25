use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut abv = vec![(0i32, 0i32); n];
    for i in 0..n {
        abv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        abv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut min_a_heap = BinaryHeap::new();
    for i in 0..n {
        min_a_heap.push((Reverse(abv[i].0), abv[i].1));
    }
    let mut max_b_heap = BinaryHeap::new();
    let mut ans = 0;
    for i in 0..m {
        loop {
            match min_a_heap.peek() {
                None => break,
                Some(&(Reverse(a), b)) => {
                    if a > (i + 1) as i32 {
                        break;
                    }
                    min_a_heap.pop();
                    max_b_heap.push(b);
                }
            }
        }
        match max_b_heap.pop() {
            None => continue,
            Some(b) => ans += b,
        }
    }
    println!("{}", ans);
}
