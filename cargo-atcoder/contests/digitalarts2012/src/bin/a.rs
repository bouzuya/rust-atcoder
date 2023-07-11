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
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut t: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let t_i: String = read(&mut stdin_lock, &mut buf, b'\n');
        t.push(t_i.chars().collect::<Vec<char>>());
    }
    // let mut t_i = vec![];
    // std::io::BufRead::read_to_end(&mut stdin_lock, &mut t_i).unwrap();
    // t.push(t_i.iter().map(|&c| c as char).collect::<Vec<char>>());
    let mut s = s.chars().collect::<Vec<char>>();
    for t_i in t.iter() {
        for j in 0..s.len() {
            let mut m = true;
            for k in 0..t_i.len() {
                if j + k >= s.len()
                    || (t_i[k] == '*' && s[j + k] == ' ')
                    || (t_i[k] != '*' && t_i[k] != s[j + k])
                {
                    m = false;
                    break;
                }
            }
            let i_b = j;
            let i_e = j + t_i.len() - 1;
            if m && ((i_b == 0 || s[i_b - 1] == ' ') && (i_e + 1 == s.len() || s[i_e + 1] == ' ')) {
                for k in 0..t_i.len() {
                    s[j + k] = '*';
                }
            }
        }
    }
    println!("{}", s.iter().collect::<String>());
}
