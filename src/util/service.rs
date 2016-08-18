use ::util::core::*;

pub struct ServiceError<DATA> {
	pub code : u32,
	pub message : String,
	pub data : DATA
}

pub type ServiceHandler<PARAMS, RETURN_VALUE, ERROR_DATA> = 
	Fn(PARAMS) -> Result<RETURN_VALUE, ServiceError<ERROR_DATA>>;
