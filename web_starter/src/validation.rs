use regex::Regex;
use std::borrow::Cow;
use std::cell::LazyCell;
use std::collections::HashMap;
use validator::ValidationError;

const MOBILE_PHONE_REGED: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^(13[0-9]|14[01456879]|15[0-35-9]|16[2567]|17[0-8]|18[0-9]|19[0-35-9])\d{8}$")
        .expect("Failed to complie MOBILE_PHONE_REGED")
});

pub fn is_mobile_phone(value: &str) -> Result<(), ValidationError> {
    if MOBILE_PHONE_REGED.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError::new("MOBILE_PHONE is error"))
    }
}

fn build_validation_error(msg: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(msg)),
        params: HashMap::new(),
    }
}
