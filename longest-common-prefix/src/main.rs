// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

// Constraints:

// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters.

fn solution(mut strs: Vec<String>) -> String {
    strs.sort_by(|a, b| a.len().cmp(&b.len()));

    if strs[0].len() == 0 {
        return "".to_string();
    }

    for i in (0..strs[0].len()).rev() {
        let str = &strs[0][..i + 1];

        if strs.iter().all(|s| s.starts_with(str)) {
            return str.to_string();
        }
    }

    return "".to_string();
}

fn main() {
    println!(
        "{}",
        solution(
            [
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]
            .to_vec()
        )
    );

    println!(
        "{}",
        solution(["dog".to_string(), "racecar".to_string(), "car".to_string()].to_vec())
    );

    println!("{}", solution(["".to_string()].to_vec()));
    println!("{}", solution(["a".to_string()].to_vec()));
}
