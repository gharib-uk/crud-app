use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<&str>>();
        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        return Self::from_base64_encoded(split[1]);
    }

    fn from_base64_encoded(base64_str: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_str).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<&str>>();

        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());
        return Some(BasicAuth { username, password });
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(basic_auth) =
                BasicAuth::from_authorization_header(auth_header) {
                return Outcome::Success(basic_auth);
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}