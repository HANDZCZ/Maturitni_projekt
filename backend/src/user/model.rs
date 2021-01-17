use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // length 3..=12
    // character set a-z|A-Z|0-9
    static ref NICK_REGEX: Regex = Regex::new(r#"^[a-zA-Z0-9]{3,12}$"#).unwrap();

    // length 1..=50
    // characters <space>&_-', cant repeat after another
    // character set a-z|A-Z|0-9|<space>|&|_|-|'|,|<czech chars>
    static ref GENDER_REGEX: Regex = Regex::new(r#"^(?!.*([ &_\-',\.])\1)([a-zA-Z0-9 &_\-',\.áčďéěíňóřšťúůýžÁČĎÉĚÍŇÓŘŠŤÚŮÝŽ]){1,50}$"#).unwrap();

    // valid email by RFC2822
    // length 5..=25
    static ref EMAIL_REGEX: Regex = Regex::new(r#"(?=^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?)(.{5,25}$)"#).unwrap();

    // length 8..=25
    // min 2 lowercase chars, 2 uppercase chars, 2 digits, 2 special chars
    static ref PASSWORD_REGEX: Regex = Regex::new(r#"^(?=(.*[\d]){2,})(?=(.*[a-z]){2,})(?=(.*[A-Z]){2,})(?=(.*[@#$%!?._-]){2,})(?:[\da-zA-Z@#$%!?._-]){8,25}$"#).unwrap();

    // length 1..=650
    // characters <space>&_-', cant repeat after another
    // character set a-z|A-Z|0-9|<space>|&|_|-|'|,|<czech chars>
    static ref DESCRIPTION_REGEX: Regex = Regex::new(r#"^(?!.*([ &_\-',\.])\1)([a-zA-Z0-9 &_\-',\.áčďéěíňóřšťúůýžÁČĎÉĚÍŇÓŘŠŤÚŮÝŽ]){20,650}$"#).unwrap();
}

macro_rules! option_matches {
    ($field:ident, $regex:ident) => {
        if let Some(val) = $field {
            if !$regex.is_match(val.as_ref()).unwrap() {
                return false;
            }
        }
    };
}

pub fn valid_user_fields<T>(
    nick: Option<T>,
    gender: Option<T>,
    email: Option<T>,
    password: Option<T>,
    description: Option<T>
) -> bool
where
    T: AsRef<str>,
{
    option_matches!(nick, NICK_REGEX);
    option_matches!(gender, GENDER_REGEX);
    option_matches!(email, EMAIL_REGEX);
    option_matches!(password, PASSWORD_REGEX);
    option_matches!(description, DESCRIPTION_REGEX);
    true
}

use serde_repr::Deserialize_repr;

#[roles::get_roles_from_db]
#[derive(Deserialize_repr, Debug, sqlx::Type, Copy, Clone, Eq, PartialEq)]
#[repr(i16)]
pub enum Role {
    Admin,
    Banned,
}
