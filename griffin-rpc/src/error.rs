use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};

pub fn error_object_from<T: std::fmt::Debug>(err: T) -> ErrorObjectOwned {
    ErrorObject::owned::<u8>(-1, format!("{err:?}"), None)
}
