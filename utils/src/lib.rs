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

pub trait ToVecExt {
    fn to_vec(&self) -> std::result::Result<Vec<u8>, CryptopalsUtilsError>;
}

impl ToVecExt for &str {
    /// Convert string containing hexadecimal numbers to a vector of bytes containing these numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::ToVecExt;
    /// let s = "0102";
    /// let bytes = s.to_vec().unwrap();
    /// assert_eq!(bytes, vec![1, 2]);
    /// ```
    ///
    /// # Errors
    ///
    /// Return `ParseIntError` if the input string contains an odd number of hexadecimal digits or
    /// any invalid hexadecimal digits.
    /// ```
    /// use utils::ToVecExt;
    /// let s = "0";
    /// assert!(s.to_vec().is_err());
    /// let s = "abcdefgh";
    /// assert!(s.to_vec().is_err());
    /// ```
    fn to_vec(&self) -> std::result::Result<Vec<u8>, CryptopalsUtilsError> {
        if self.len() % 2 != 0 {
            return Err(CryptopalsUtilsError::InvalidInput("the number of characters is even".to_string()));
        }
        let mut bytes = Vec::new();
        for b in (0..self.len()).step_by(2) {
            match u8::from_str_radix(&self[b..b+2], 16) {
                Ok(i) => bytes.push(i),
                Err(e) => return Err(CryptopalsUtilsError::ParseIntError(e.to_string()))
            }
        }
        Ok(bytes)
    }
}

pub trait ToHexStringExt {
    fn to_hex_string(&self) -> String;
}

impl<T: std::fmt::Debug> ToHexStringExt for Vec<T> {
    /// Convert vector of integers to a string of lowercase hex numbers without leading 0x.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::ToHexStringExt;
    /// let v = vec![1, 2];
    /// assert_eq!(v.to_hex_string(), "12");
    /// let v = vec![0xde, 0xad, 0xbe, 0xef];
    /// assert_eq!(v.to_hex_string(), "deadbeef");
    /// ```
    fn to_hex_string(&self) -> String {
        let mut output = String::new();
        for byte in self.into_iter() {
            output.push_str(format!("{:x?}", byte).as_str());
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_str_to_vec_ok() {
        assert_eq!(
            "aabbccdd1122334455".to_vec().unwrap(),
            vec![0xaa,0xbb,0xcc,0xdd,0x11,0x22,0x33,0x44,0x55]
        );

        assert_eq!(
            "deadbeefcafe".to_vec().unwrap(),
            vec![0xde,0xad,0xbe,0xef,0xca,0xfe]
        );

        assert_eq!("2137".to_vec().unwrap(), vec![0x21,0x37]);

        assert_eq!("".to_vec().unwrap(), vec![]);
    }

    #[test]
    fn test_hex_str_to_vec_err() {
        assert!("0".to_vec().is_err());
        assert!("000".to_vec().is_err());
        assert!("elo".to_vec().is_err());
    }

    #[test]
    fn test_vec_to_hex_str() {
        assert_eq!(vec![1, 2, 3].to_hex_string(), "123");
        assert_eq!(vec![0xde, 0xad, 0xbe, 0xef].to_hex_string(), "deadbeef");
        assert_eq!(vec![0xa].to_hex_string(), "a");
    }
}