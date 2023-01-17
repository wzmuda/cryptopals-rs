//! Cryptopals :: Set 1 :: Challenge 2
//! Write a function that takes two equal-length buffers and produces their XOR combination.

use utils::{ToVecExt, ToHexStringExt};

fn xor_two_strings(input_a: &str, input_b: &str) -> Option<String> {
    if input_a.len() != input_b.len() {
        return None
    }

    let mut output = Vec::new();
    let input_a = input_a.to_vec().unwrap();
    let input_b = input_b.to_vec().unwrap();
    for (a, b) in input_a.into_iter().zip(input_b.into_iter()) {
        output.push(a ^ b);
    }

    Some(output.to_hex_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_two_strings() {
        assert_eq!(
            xor_two_strings(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ).unwrap(),
            "746865206b696420646f6e277420706c6179"
        );
    }
}
