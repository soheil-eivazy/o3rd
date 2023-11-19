use crate::model;
use mongodb::error::Error as MongoError;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Modules
	Model(model::Error),
    Mongo(MongoError),
}

impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Self::Model(val)
	}
}

impl From<MongoError> for Error {
    fn from(val: MongoError) -> Self {
		Self::Mongo(val)
	}
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
