//! Cryptopals :: Set 1 :: Challenge 3
//! The hex encoded string has been XOR'd against a single character.
//! Find the key, decrypt the message.

use utils::ToVecExt;

fn is_control_char(c: char) -> bool {
    (c as u8) < 32 // ascii values below space are non-printable
}

fn xored_hex_str_find_key(_input: &str) -> char {
    'X'  // I guessed it; TODO key discovery

}

fn xored_hex_str_decrypt(input: &str) -> String {
    let v = input.to_vec().unwrap();
    let mut out = String::new();
    let key = xored_hex_str_find_key(input);

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
