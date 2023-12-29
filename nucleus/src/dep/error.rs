use derive_more::{From, Display};
use crate::model;
use mongodb::error::Error as MongoError;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
	// -- Modules
	#[from]
	Model(model::Error),
	#[from]
    Mongo(MongoError),
	MissingEnv(&'static str)
}


// impl core::fmt::Display for Error {
// 	fn fmt(
// 		&self,
// 		fmt: &mut core::fmt::Formatter,
// 	) -> core::result::Result<(), core::fmt::Error> {
// 		write!(fmt, "{self:?}")
// 	}
// }

impl std::error::Error for Error {}
