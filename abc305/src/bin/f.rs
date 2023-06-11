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

    let nm: String = read(&mut stdin_lock, &mut buf, b'\n');
    let nm = nm.split(' ').collect::<Vec<&str>>();
    let n = nm[0].parse::<usize>().unwrap();
    let mut used = vec![false; n + 1];
    let mut stack = vec![1_usize];
    loop {
        let curr = *stack.last().unwrap();
        used[curr] = true;

        let kv: String = read(&mut stdin_lock, &mut buf, b'\n');
        let kv = kv
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if let Some(v) = kv.iter().copied().skip(1).find(|&v| !used[v]) {
            stack.push(v);
        } else {
            stack.pop();
        }
        println!("{}", *stack.last().unwrap());
        if *stack.last().unwrap() == n {
            break;
        }
    }
}
