use std::time::Instant;

pub fn merge_sort(vector: &Vec<i32>) -> Vec<i32> {
    if vector.len() == 1 {
        return vector.to_vec();
    }
    let mut result_vector: Vec<i32> = Vec::new();
    let median: usize = (vector.len() / 2) as usize;
    let left = &vector[0..median];
    let right = &vector[median..vector.len()];
    let mut left_node = merge_sort(&left.to_vec());
    let mut right_node = merge_sort(&right.to_vec());
    let i: usize = 0;
    while left_node.len() != i && right_node.len() != i {
        if left_node[i] <= right_node[i] {
            let val = left_node.remove(i);
            result_vector.push(val);
        } else {
            let val = right_node.remove(i);
            result_vector.push(val);
        }
    }
    result_vector.extend(left_node.iter());
    result_vector.extend(right_node.iter());
    result_vector
}

pub fn bubble_sort(vector: &Vec<i32>) -> String {
    let now = Instant::now();
    let mut result_vector: Vec<i32> = vector.clone();
    for i in 0..(result_vector.len() - 1) {
        for k in 0..(result_vector.len() - i - 1) {
            if result_vector[k] > result_vector[k+1] {
                result_vector.swap(k, k+1);
            }
        }
    }
    let new_now = Instant::now();
    format!("time is {:?}", new_now.duration_since(now))
    //result_vector
}

pub fn selection_sort(vector: &Vec<i32>) -> String {
    let now = Instant::now();
    let mut result_vector: Vec<i32> = vector.clone();
    for i in 0..(result_vector.len() - 1) {
        let mut min_idx = i;
        for k in (i + 1)..result_vector.len() {
            if result_vector[k] < result_vector[min_idx] {
                min_idx = k;
            }
        }
        result_vector.swap(min_idx, i);
    }
    let new_now = Instant::now();
    format!("time is {:?}", new_now.duration_since(now))
    //result_vector
}