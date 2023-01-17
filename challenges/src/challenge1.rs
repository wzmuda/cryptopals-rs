//! Cryptopals :: Set 1 :: Challenge 1
//! Convert hex to base64
use utils::ToVecExt;

fn vec_to_base64(input: Vec<u8>) -> String {
    let lut = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();

    for chunk in input.chunks(3) {
        let mut bytes = 0;
        for (shift, byte) in chunk.iter().enumerate() {
            bytes |= (*byte as u32) << (16 - 8*shift);
        }
        for shift in (0..4).rev() {
            let index = (bytes >> 6*shift) & 0x3F;
            // the unwrap below can't fail - index is always between 0 and 63
            // and lookup_table is 64-chars long
            output.push(lut.chars().nth(index as usize).unwrap());
        }
    }

    if input.len() % 3 == 1 {
        output.push_str("==");
    } else if input.len() % 3 == 2 {
        output.push_str("=");
    }

    output
}

fn hex_str_to_base64(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(vec_to_base64(input.to_vec()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(
            vec_to_base64(
                vec![0x49,0x27,0x6d,0x20,0x6b,0x69,0x6c,0x6c,
                     0x69,0x6e,0x67,0x20,0x79,0x6f,0x75,0x72,
                     0x20,0x62,0x72,0x61,0x69,0x6e,0x20,0x6c,
                     0x69,0x6b,0x65,0x20,0x61,0x20,0x70,0x6f,
                     0x69,0x73,0x6f,0x6e,0x6f,0x75,0x73,0x20,
                     0x6d,0x75,0x73,0x68,0x72,0x6f,0x6f,0x6d]
            ),
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
            );

        assert_eq!(
            hex_str_to_base64(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6\
                f7573206d757368726f6f6d",
            ).unwrap(),
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
            );
    }
}