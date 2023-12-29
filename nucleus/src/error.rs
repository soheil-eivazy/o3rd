use derive_more::From;
use mongodb::error::Error as MongoError;
use mongodb::bson::oid::Error as ObjectIdError;

pub type Result<T> = core::result::Result<T, Error>;


#[derive(Debug, From)]
pub enum Error {
    MissingEnv(&'static str),
	WrongFormat(&'static str),
    #[from]
    Mongo(MongoError),
    #[from]
    ObjectId(ObjectIdError),
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}