use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn to_md5_str(text: String) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(&text);
    hasher.result_str()
}

#[cfg(test)]
mod test {
    use crate::util::md5_util::to_md5_str;

    #[test]
    fn test_to_md5_str() {
        let m = to_md5_str(String::from("abc123456"));
        assert_eq!(m, String::from("0659c7992e268962384eb17fafe88364"));
    }
}
