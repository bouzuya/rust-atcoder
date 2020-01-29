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
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut av = vec![0i32; n];
    for i in 0..n {
        av[i] = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        let mut tmp = Vec::new();
        let a = av[i];
        loop {
            match heap.pop() {
                Some(x) => {
                    if x < a {
                        heap.push(a);
                        for j in 0..tmp.len() {
                            heap.push(tmp[j]);
                        }
                        break;
                    } else {
                        tmp.push(x);
                        continue;
                    }
                }
                None => {
                    heap.push(a);
                    for j in 0..tmp.len() {
                        heap.push(tmp[j]);
                    }
                    break;
                }
            }
        }
    }

    let ans = heap.len();
    println!("{}", ans);
}
