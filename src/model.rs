//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    main: Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    temp: f64,
}

// token main entry structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub main: TokenMain,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMain {
    pub token: String,
}

// auth login structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SiteUser {
    pub username: String,
    pub password: String,
}

// greeting main entry structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub main: HelloMain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloMain {
    pub greeting: String,
}
