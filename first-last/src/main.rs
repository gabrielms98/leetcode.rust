fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![-1, -1];
    }

    let mut i: i32 = 0;
    let mut j: i32 = nums.len() as i32 - 1;

    let mut res: Vec<i32> = vec![-1, -1];

    while i <= j {
        if nums[i as usize] == target {
            res[0] = i;
        } else {
            i += 1;
        }

        if nums[j as usize] == target {
            res[1] = j;
        } else {
            j -= 1;
        }

        if res[0] != -1 && res[1] != -1 {
            break;
        }
    }

    return res;
}

fn main() {
    println!("{:?}", solution([5, 7, 7, 8, 8, 10].to_vec(), 8));
    println!("{:?}", solution([5, 7, 7, 8, 8, 10].to_vec(), 6));
    println!("{:?}", solution([].to_vec(), 0));
    println!("{:?}", solution([1].to_vec(), 1));
}
