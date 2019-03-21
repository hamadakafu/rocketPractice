// jsonを返すようにする error のときも
use mongodb;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum ApiError {
    #[fail(display = "{} does not exist.", _0)]
    NotExistRoom(String),
    #[fail(display = "{} already exist.", _0)]
    AlreadyExistRoom(String),
    #[fail(display = "server and mongo error: {}", _0)]
    MongoError(#[fail(cause)] mongodb::Error),
}

impl From<mongodb::Error> for ApiError {
    fn from(error: mongodb::Error) -> Self {
        ApiError::MongoError(error)
    }
}
