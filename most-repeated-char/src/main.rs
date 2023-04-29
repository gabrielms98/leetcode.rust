use std::collections::HashMap;

fn solution(s: String) -> char {
    return *s
        .chars()
        .fold(HashMap::<char, i32>::new(), |mut acc, c| {
            if c.is_numeric() {
                return acc;
            }

            let counter = acc.entry(c).or_insert(0);
            *counter += 1;
            acc
        })
        .iter()
        .max_by_key(|&(_, v)| v)
        .unwrap()
        .0;
}

fn main() {
    println!("{}", solution("abcddefda131312313111".to_string()));
    println!(
        "{}",
        solution("AA0AB0BB0ccc0aa0aw00woOBBBw123123".to_string())
    );
}
