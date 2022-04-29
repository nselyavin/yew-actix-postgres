use validator::{ValidationError, ValidationErrors};

pub fn validErr_to_str(errs: &ValidationErrors) -> String {
    let mut res = String::new();

    log::warn!("Error to signup: {:?}", errs.field_errors());

    if let Some(err) = errs.field_errors().get("email") {
        res = "Bad email".to_string();
    } else if let Some(err) = errs.field_errors().get("password") {
        res = "Password min length 8".to_string();
    }


    return "Unknow error".to_owned();
}
