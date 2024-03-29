use derive_more::Display;

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BizCode(&'static str);

impl BizCode {
    pub fn code(&self) -> &'static str {
        self.0
    }

    pub fn reason(&self) -> Option<&'static str> {
        canonical_reason(self.code())
    }

    pub fn code_reason(&self) -> String {
        format!("{}: {}", self.code(), self.reason().unwrap())
    }
}

macro_rules! biz_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl BizCode {
        $(
            $(#[$docs])*
            pub const $konst: BizCode = BizCode($num);
        )+

        }

        fn canonical_reason(num: &'static str) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

biz_codes! {
    ("000000", SUCCESS, "Success");
    ("000001", SYSTEM_ERROR, "System error");
    ("000002", STATIC_OAUTH_NOT_CONFIG, "static oauth info not config");
    // Bcrypt
    ("BE0001", BCRYPT_ERROR, "Bcrypt error");
    // auth
    ("AU0001", WRONG_CREDENTIALS, "wrong credentials");
    ("AU0002", JWT_INVALID, "jwt token not valid");
    ("AU0003", JWT_CREATION_ERR, "jwt token creation error");
    ("AU0004", LOGIN_TIMEOUT, "Login timeout");
    ("AU0005", INVALID_AUTH_HEADER, "invalid auth2 header");
    ("AU0006", NO_PERMISSION, "no permission");
    ("AU0007", LOGOUT_SUCCESS, "Logout success");
    ("AU0008", LOGIN_UID_ERR, "Login uid incorrect");
    ("AU0009", LOGIN_PWD_ERR, "Login password incorrect");
    // database
    ("DB0001", DATABASE_ERROR, "Database error");
    ("ORM001", DIESEL_ERROR, "Diesel error");
    // user
    ("UR0001", USER_NOT_FOUND, "User not found");
    // log
    ("LG0001", LOG_TO_JSON_STRING_ERROR, "LogBody to json string error");
    // network
    ("NET001", REQWEST_ERROR, "Send log to server error");
}
