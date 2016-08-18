use ::util::core::*;

pub trait Provider<T> {
	fn obtain_next(&mut self) -> GResult<T>;
}

pub struct ServiceError<DATA> {
	pub code : u32,
	pub message : String,
	pub data : DATA
}

pub type ServiceHandler<PARAMS, RETURN_VALUE, ERROR_DATA> = 
	Fn(PARAMS) -> Result<RETURN_VALUE, ServiceError<ERROR_DATA>>;
