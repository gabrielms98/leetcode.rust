use std::collections::HashMap;

fn solution(s: String) -> i32 {
    let roman_map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let prev_map: HashMap<char, [char; 2]> =
        HashMap::from([('I', ['V', 'X']), ('X', ['L', 'C']), ('C', ['D', 'M'])]);

    let mut prev = s.chars().nth(0).unwrap();

    let res = s.chars().fold(0, |acc, c| match roman_map.get(&c) {
        Some(val) => {
            let prev_val = roman_map.get(&prev).unwrap();

            if let Some(p) = prev_map.get(&prev) {
                if p.contains(&c) {
                    prev = c;
                    return acc + (val - 2 * prev_val);
                }
            }

            prev = c;
            return acc + val;
        }
        _ => acc,
    });

    return res;
}

fn main() {
    println!("{}", solution("III".to_string()));
    println!("{}", solution("LVIII".to_string()));
    println!("{}", solution("MCMXCIV".to_string()));
    println!("{}", solution("XCII".to_string()));
}
