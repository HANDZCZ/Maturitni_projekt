use lazy_static::lazy_static;
use fancy_regex::Regex;

lazy_static! {
    pub static ref EMAIL_REGEX: Regex = Regex::new(r#"(?=^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?)(.{5,25}$)"#).unwrap();

    pub static ref NICK_REGEX: Regex = Regex::new(r#"^[a-zA-Z0-9]{3,12}$"#).unwrap();

    pub static ref GENDER_REGEX: Regex = Regex::new(r#"^(?!.*([ &_\-',\.])\1)([a-zA-Z0-9 &_\-',\.áčďéěíňóřšťúůýžÁČĎÉĚÍŇÓŘŠŤÚŮÝŽ]){1,50}$"#).unwrap();

    pub static ref PASSWORD_REGEX: Regex = Regex::new(r#"^(?=(.*[\d]){2,})(?=(.*[a-z]){2,})(?=(.*[A-Z]){2,})(?=(.*[@#$%!?._-]){2,})(?:[\da-zA-Z@#$%!?._-]){8,25}$"#).unwrap();

    pub static ref DESCRIPTION_REGEX: Regex = Regex::new(r#"^(?!.*([ &_\-',\.])\1)([a-zA-Z0-9 &_\-',\.áčďéěíňóřšťúůýžÁČĎÉĚÍŇÓŘŠŤÚŮÝŽ]){20,650}$"#).unwrap();
    
    pub static ref GAME_NAME_REGEX: Regex = Regex::new(r#"^(?!.*([ &_\-',\.])\1)[a-zA-Z0-9 &_\-',\.]{5,15}$"#).unwrap();
}
