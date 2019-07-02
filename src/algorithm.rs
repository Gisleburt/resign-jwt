use std::str::FromStr;
use frank_jwt::Algorithm as FrankAlgorithm;
use crate::NewType;
use std::fmt::{Debug, Formatter, Error};

pub type Algorithm = NewType<FrankAlgorithm>;

impl Debug for Algorithm {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HS256" => Ok(NewType::new(FrankAlgorithm::HS256)),
            "HS384" => Ok(NewType::new(FrankAlgorithm::HS384)),
            "HS512" => Ok(NewType::new(FrankAlgorithm::HS512)),
            "RS256" => Ok(NewType::new(FrankAlgorithm::RS256)),
            "RS384" => Ok(NewType::new(FrankAlgorithm::RS384)),
            "RS512" => Ok(NewType::new(FrankAlgorithm::RS512)),
            "ES256" => Ok(NewType::new(FrankAlgorithm::ES256)),
            "ES384" => Ok(NewType::new(FrankAlgorithm::ES384)),
            "ES512" => Ok(NewType::new(FrankAlgorithm::ES512)),
            _ => {
                Err(format!(
                    "{}\n{}",
                    format!("No Algorithm matching \"{}\"", s),
                    format!("Supported algorithms: HS256, HS384, HS512, RS256, RS384, RS512, ES256, ES384, ES512")
                ))
            }
        }
    }
}
