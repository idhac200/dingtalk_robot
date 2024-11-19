use std::time::SystemTime;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub(crate) fn current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub(crate) fn hmac_sha256<T, U>(key: T, input: U) -> [u8; 32]
where
    T: AsRef<[u8]>,
    U: AsRef<[u8]>,
{
    let mut mac = <Hmac<Sha256>>::new_from_slice(key.as_ref()).unwrap();
    mac.update(input.as_ref());
    mac.finalize().into_bytes().into()
}