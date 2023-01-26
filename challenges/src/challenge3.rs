//! Cryptopals :: Set 1 :: Challenge 3
//! The hex encoded string has been XOR'd against a single character.
//! Find the key, decrypt the message.
use std::collections::HashMap;
use utils::ToVecExt;

fn is_control_char(c: char) -> bool {
    (c as u8) < 32 // ascii values below space are non-printable
}

fn xored_hex_str_find_key(input: &Vec<u8>) -> char {
    // Build a hashmap with bytes' values as keys and their frequency in that vector as values
    let mut m = HashMap::new();
    for b in input {
        *m.entry(*b).or_insert(0) +=1;
    }

    // Assumption: ' ' (space) is the most frequent character in an English sentence
    let (most_frequent_char, _) = m.iter().max_by_key(|(_, count)| *count).unwrap();

    const SPACE_ASCII: u8 = 32;
    (*most_frequent_char ^ SPACE_ASCII) as char
}

fn xored_hex_str_decrypt(input: &str) -> String {
    let v = input.to_vec().unwrap();
    let key = xored_hex_str_find_key(&v);
    let mut out = String::new();

    for b in v.into_iter() {
        let c = (b ^ key as u8) as char;
        if !is_control_char(c) {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_secret() {
        assert_eq!(
            xored_hex_str_decrypt(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            ),
            "Cooking MC's like a pound of bacon"
        )
    }
}
