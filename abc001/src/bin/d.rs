use proconio::input;

fn main() {
    input! {
        n: usize,
        se: [String; n],
    };
    let mut ses = vec![];
    for se_i in se {
        let se_i = se_i
            .split('-')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let (s, e) = (se_i[0], se_i[1]);
        let (h_s, m_s) = (s / 100, s % 100);
        let (h_e, m_e) = (e / 100, e % 100);
        let (m_s, m_e) = (m_s / 5 * 5, (m_e + 5 - 1) / 5 * 5);
        let (h_s, h_e) = (
            h_s + if m_s == 60 { 1 } else { 0 },
            h_e + if m_e == 60 { 1 } else { 0 },
        );
        let (s, e) = (h_s * 100 + m_s % 60, h_e * 100 + m_e % 60);
        ses.push((s, e));
    }
    ses.sort();

    let mut count = vec![0; 2400 + 2];
    for (s, e) in ses.iter().copied() {
        count[s] += 1;
        count[e + 1] -= 1;
    }
    for i in 1..2400 + 1 {
        count[i] += count[i - 1];
    }

    let mut s = None;
    for i in 0..2400 + 1 {
        s = match s {
            None => {
                if count[i] > 0 {
                    Some(i)
                } else {
                    None
                }
            }
            Some(s) => {
                if count[i] == 0 {
                    println!("{:04}-{:04}", s, i - 1);
                    None
                } else {
                    Some(s)
                }
            }
        }
    }
    if let Some(s) = s {
        println!("{:04}-{:04}", s, 2400);
    }
}
