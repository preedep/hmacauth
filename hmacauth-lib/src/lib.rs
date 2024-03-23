//pub fn add(left: usize, right: usize) -> usize {
//    left + right
//}


pub mod utils;
pub mod models;

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    #[test]
    fn it_compute_sha256() {
        let payload = String::from("Hello, world!");
        let result = utils::compute_content_sha256(&payload);
        assert_eq!(result, "MV9b23bQeMQ7isAGTkoBZGErH853yGk0W/yUx1iU7dM=");
    }

    #[test]
    fn it_generate_string_to_sign_with_uri_query_string() {
        let url = Url::parse("https://example.com/apis/v1/hello?version=2024-03-01").unwrap();
        let http_method = "GET";
        let http_date = "Sun, 06 Nov 1994 08:49:37 GMT".to_string();
        let json_payload = "Hello, world!".to_string();
        let (compute_hash, string_to_sign) = utils::generate_string_to_sign(&url,
                                                                            http_method,
                                                                            &http_date,
                                                                            &json_payload);

        assert_eq!(compute_hash, "MV9b23bQeMQ7isAGTkoBZGErH853yGk0W/yUx1iU7dM=");
        assert_eq!(string_to_sign, "GET\n/apis/v1/hello?version=2024-03-01\nSun, 06 Nov 1994 08:49:37 GMT;example.com;MV9b23bQeMQ7isAGTkoBZGErH853yGk0W/yUx1iU7dM=");
    }

    #[test]
    fn it_generate_string_to_sign() {
        let url = Url::parse("https://example.com/apis/v1/hello").unwrap();
        let http_method = "GET";
        let http_date = "Sun, 06 Nov 1994 08:49:37 GMT".to_string();
        let json_payload = "Hello, world!".to_string();
        let (compute_hash, string_to_sign) = utils::generate_string_to_sign(&url,
                                                                            http_method,
                                                                            &http_date,
                                                                            &json_payload);

        assert_eq!(compute_hash, "MV9b23bQeMQ7isAGTkoBZGErH853yGk0W/yUx1iU7dM=");
        assert_eq!(string_to_sign, "GET\n/apis/v1/hello\nSun, 06 Nov 1994 08:49:37 GMT;example.com;MV9b23bQeMQ7isAGTkoBZGErH853yGk0W/yUx1iU7dM=");
    }

    #[test]
    fn it_compute_signature() {
        let string_to_sign = "GET\n/\nSun, 06 Nov 1994 08:49:37 GMT;example.com;MV9b23bQeMQ7isAGTkoBZGErH853yGk0W/yUx1iU7dM=".to_string();
        let access_key = "IbNSH3Lc5ffMHo/wnQuiOD4C0mx5FqDmVMQaAMKFgaQ=".to_string();
        let result = utils::compute_signature(&string_to_sign, &access_key);
        assert_eq!(result, "HyXBLCEqeI6AfpCUvWsbH1lX9hTAF7u6XwSAicsWiVs=");
    }
}
