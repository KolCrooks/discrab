pub fn get_header_as<T>(headers: &hyper::header::HeaderMap, key: &str) -> Option<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    headers
        .get(key)
        .and_then(|header| header.to_str().ok())
        .and_then(|header_str| header_str.parse().ok())
}
