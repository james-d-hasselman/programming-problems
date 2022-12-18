use std::collections::HashMap;
use std::env;
use std::io;

fn compute_next_number(n: u32) -> u32 {
    return if n % 2 == 1 {
        n * 3 + 1
    } else {
        n / 2
    }
}

fn compute_sequence_length(n: u32, lookup_table: &HashMap<u32, u32>) -> u32 {
    // if n = 1, return 1
    // else if input is in lookup table return sequence length
    // else return 1 + sequence_length(next number in sequence)
    let mut sequence_length: u32 = 0;
    let mut n = n;
    loop {
        if n == 1 {
            sequence_length = sequence_length + 1;
            return sequence_length;
        } else if lookup_table.contains_key(&n) {
            return sequence_length + lookup_table.get(&n).unwrap(); 
        } else {
            sequence_length = sequence_length + 1;
            n = compute_next_number(n);
        }
    }
}

fn main() {
    let lookup_table : HashMap<u32, u32> = HashMap::new();
    for line in io::stdin().lines() {
        let pair = line.unwrap().split(" ");
        println!("{}", line.unwrap());
    }
}