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

/// Convert vector of integers to a string of lowercase hex numbers without leading 0x.
///
/// # Examples
///
/// ```
/// use utils::vec_to_hex_str;
/// let v = vec![1, 2];
/// assert_eq!(vec_to_hex_str(v), "12");
/// let v = vec![0xde, 0xad, 0xbe, 0xef];
/// assert_eq!(vec_to_hex_str(v), "deadbeef");
/// ```
pub fn vec_to_hex_str(input: Vec<u8>) -> String {
    let mut output = String::new();
    for byte in input.into_iter() {
        output.push_str(format!("{:x?}", byte).as_str());
    }
    output
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

    #[test]
    fn test_vec_to_hex_str() {
        assert_eq!(vec_to_hex_str(vec![1, 2, 3]), "123");
        assert_eq!(vec_to_hex_str(vec![0xde, 0xad, 0xbe, 0xef]), "deadbeef");
        assert_eq!(vec_to_hex_str(vec![0xa]), "a");
    }
}