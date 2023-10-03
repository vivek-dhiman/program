fn longest_increasing_subsequence(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    if n == 0 {
        return Vec::new();
    }

    // Initialize a vector to store the length of the longest increasing subsequence
    let mut lis_length = vec![1; n];

    // Initialize a vector to store the previous index for each element in the LIS
    let mut previous_index = vec![None; n];

    for i in 1..n {
        for j in 0..i {
            if arr[i] > arr[j] && lis_length[i] < lis_length[j] + 1 {
                lis_length[i] = lis_length[j] + 1;
                previous_index[i] = Some(j);
            }
        }
    }

    // Find the index of the maximum value in lis_length
    let max_length_index = lis_length.iter().enumerate().fold(0, |max_index, (i, &val)| {
        if val > lis_length[max_index] {
            i
        } else {
            max_index
        }
    });

    // Reconstruct the longest increasing subsequence
    let mut longest_subsequence = Vec::new();
    let mut current_index = max_length_index;
    while let Some(index) = previous_index[current_index] {
        longest_subsequence.push(arr[current_index]);
        current_index = index;
    }
    longest_subsequence.push(arr[current_index]);
    longest_subsequence.reverse();

    longest_subsequence
}

fn main() {
    let arr = vec![10, 22, 9, 33, 21, 50, 41, 60, 80];
    let lis = longest_increasing_subsequence(&arr);
    println!("Longest Increasing Subsequence: {:?}", lis);
}
