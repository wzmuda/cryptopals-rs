use std::error::Error;
use std::fmt::{Display, Result};

#[derive(Debug)]
pub enum CryptopalsUtilsError {
    InvalidInput(String),
    ParseIntError(String)
}

impl Error for CryptopalsUtilsError {}
impl Display for CryptopalsUtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        write!(f, "error: {:?}", self)
    }
}

/// Convert string containing hexadecimal numbers to a vector of bytes containing these numbers.
///
/// # Examples
///
/// ```
/// use utils::hex_str_to_vec;
/// let bytes = hex_str_to_vec("0102").unwrap();
/// assert_eq!(bytes, vec![1, 2]);
/// ```
///
/// # Errors
///
/// Return `ParseIntError` if the input string contains an odd number of hexadecimal digits or
/// any invalid hexadecimal digits.
/// ```
/// use utils::hex_str_to_vec;
/// assert!(hex_str_to_vec("0").is_err());
/// assert!(hex_str_to_vec("abcdefgh").is_err());
/// ```
pub fn hex_str_to_vec(input: &str) -> std::result::Result<Vec<u8>, CryptopalsUtilsError> {
    if input.len() % 2 != 0 {
        return Err(CryptopalsUtilsError::InvalidInput("the number of characters is even".to_string()));
    }
    let mut bytes = Vec::new();
    for b in (0..input.len()).step_by(2) {
        match u8::from_str_radix(&input[b..b+2], 16) {
            Ok(i) => bytes.push(i),
            Err(e) => return Err(CryptopalsUtilsError::ParseIntError(e.to_string()))
        }
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_str_to_vec_ok() {
        assert_eq!(
            hex_str_to_vec("aabbccdd1122334455").unwrap(),
            vec![0xaa,0xbb,0xcc,0xdd,0x11,0x22,0x33,0x44,0x55]
        );

        assert_eq!(
            hex_str_to_vec("deadbeefcafe").unwrap(),
            vec![0xde,0xad,0xbe,0xef,0xca,0xfe]
        );

        assert_eq!(hex_str_to_vec("2137").unwrap(), vec![0x21,0x37]);

        assert_eq!(hex_str_to_vec("").unwrap(), vec![]);
    }

    #[test]
    fn test_hex_str_to_vec_err() {
        assert!(hex_str_to_vec("0").is_err());
        assert!(hex_str_to_vec("000").is_err());
        assert!(hex_str_to_vec("elo").is_err());
    }
}