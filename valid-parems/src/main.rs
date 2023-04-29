fn solution(input: String) -> bool {
    let mut arr = vec![];

    let mut is_false = false;

    if input.len() == 1 {
        return false;
    }

    input.chars().for_each(|c| match c {
        '(' | '[' | '{' => arr.push(c),
        ')' => {
            if arr.pop() != Some('(') {
                is_false = true;
                return;
            }
        }
        ']' => {
            if arr.pop() != Some('[') {
                is_false = true;
                return;
            }
        }
        '}' => {
            if arr.pop() != Some('{') {
                is_false = true;
                return;
            }
        }
        _ => {}
    });

    return !is_false && arr.len() == 0;
}

fn main() {
    println!("{}", solution("()".to_string()));
    println!("{}", solution("(]".to_string()));
    println!("{}", solution("()[]{}".to_string()));
}
