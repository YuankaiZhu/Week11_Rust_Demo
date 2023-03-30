use std::io::{self, Write};

fn main() {
    println!("Longest Increasing Subsequence Finder");
    println!("Enter a sequence of integers separated by spaces:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    let sequence: Result<Vec<i32>, _> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect();

    match sequence {
        Ok(seq) => {
            let lis = longest_increasing_subsequence(&seq);
            println!("The length of the longest increasing subsequence is: {}", lis);
        }
        Err(_) => {
            eprintln!("Error: Please enter a valid sequence of integers separated by spaces.");
        }
    }
}

fn longest_increasing_subsequence(seq: &[i32]) -> usize {
    if seq.is_empty() {
        return 0;
    }

    let n = seq.len();
    let mut dp = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if seq[i] > seq[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }

    *dp.iter().max().unwrap()
}
