fn solution(s: String) -> String {
    let mut i = 0;
    let j = s.len();

    if j == 0 {
        return s.to_string();
    }

    while i <= j {
        let str = &s[i..j];

        let mut is_palim = true;

        for k in i..j {
            let mut c = s.chars();
            if c.nth(k) != c.nth(j - k) {
                is_palim = false;
            }

            if i >= j || k < j {
                break;
            }
        }

        if is_palim {
            return str.to_string();
        }

        i += 1;
    }

    return s.chars().nth(0).unwrap().to_string();
}

fn main() {
    println!("{}", solution("babad".to_string())); // aba
    println!("{}", solution("cbbd".to_string())); // bb
    println!("{}", solution("ac".to_string())); // a
    println!("{}", solution("bb".to_string())); // bb
    println!("{}", solution("abb".to_string())); // bb
}
