// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

// solution if the response didn't need to be the index of the element
fn solution_wrong(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.sort();

    let mut j = nums.len() - 1;
    let mut i = 0;

    while i < j {
        let sum = nums[i] + nums[j];

        if sum == target {
            return vec![nums[i], nums[j]];
        }

        if sum > target {
            j -= 1;
        }

        if sum < target {
            i += 1;
        }
    }

    return nums;
}

fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n) in nums.iter().enumerate() {
        for (j, m) in nums.iter().enumerate() {
            if i == j {
                continue;
            }

            if m + n == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    return vec![1];
}

fn main() {
    println!("{:?}", solution([2, 7, 11, 15].to_vec(), 9));
    println!("{:?}", solution([3, 2, 4].to_vec(), 6));
    println!("{:?}", solution([3, 3].to_vec(), 6));
}
