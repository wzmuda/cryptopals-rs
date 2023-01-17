//! Cryptopals :: Set 1 :: Challenge 2
//! Write a function that takes two equal-length buffers and produces their XOR combination.

use utils::{hex_str_to_vec, vec_to_hex_str};

fn xor_two_strings(input_a: &str, input_b: &str) -> Option<String> {
    if input_a.len() != input_b.len() {
        return None
    }

    let mut output = Vec::new();
    let input_a = hex_str_to_vec(input_a).unwrap();
    let input_b = hex_str_to_vec(input_b).unwrap();
    for (a, b) in input_a.into_iter().zip(input_b.into_iter()) {
        output.push(a ^ b);
    }

    Some(vec_to_hex_str(output))
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
