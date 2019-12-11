use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
	#[display(fmt = "Generic error.")]
	Error,

	#[display(fmt = "Error: {}", _0)]
	StringError(String),

	#[display(fmt = "Formatting error: {}", _0)]
	FormatError(std::fmt::Error),

	#[display(fmt = "I/O error: {}", _0)]
	IoError(std::io::Error),
}

impl From<()> for AppError {
	fn from(_error: ()) -> AppError {
		AppError::Error
	}
}

impl From<String> for AppError {
	fn from(error: String) -> AppError {
		AppError::StringError(error)
	}
}

impl From<&str> for AppError {
	fn from(error: &str) -> AppError {
		AppError::StringError(error.to_owned())
	}
}

impl From<std::fmt::Error> for AppError {
	fn from(error: std::fmt::Error) -> AppError {
		AppError::FormatError(error)
	}
}

impl From<std::io::Error> for AppError {
	fn from(error: std::io::Error) -> AppError {
		AppError::IoError(error)
	}
}

impl From<Box<dyn std::error::Error>> for AppError {
	fn from(_error: Box<dyn std::error::Error>) -> AppError {
		AppError::Error
	}
}
