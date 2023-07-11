use proconio::input;

fn main() {
    input! {
        n: usize,
        md: [String; n],
    };
    let mut h = vec![];
    for md_i in md.iter() {
        let v = md_i.split('/').collect::<Vec<&str>>();
        let m_i = v[0].parse::<usize>().unwrap();
        let d_i = v[1].parse::<usize>().unwrap();
        h.push((m_i, d_i));
    }

    let dom = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m = 1;
    let mut d = 1;
    let mut b = 0;
    let mut c = 0;
    let mut max_c = 0;
    for i in 0..366 {
        let wd = i % 7; // Sun: 0, Mon: 1, Tue: 2, ...

        let h_i = h.contains(&(m, d));
        if wd == 0 || wd == 6 {
            c += 1;
            if h_i {
                b += 1;
            }
        } else if h_i {
            c += 1;
        } else if b > 0 {
            c += 1;
            b -= 1;
            if h_i {
                b += 1;
            }
        } else {
            c = 0;
        }
        max_c = std::cmp::max(max_c, c);

        let m_next = if d == dom[m - 1] { m + 1 } else { m };
        let d_next = if d == dom[m - 1] { 1 } else { d + 1 };
        m = m_next;
        d = d_next;
    }

    let ans = max_c;
    println!("{}", ans);
}
