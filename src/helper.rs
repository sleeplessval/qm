use evalexpr::{
	Value,

	EvalexprError
};

use crate::util;

//	Mathematics
pub fn logarithm(arg: &Value) -> Result<Value, EvalexprError> {
	let arguments: Vec<Value>;
	let count: usize;
	if arg.is_tuple() {
		arguments = arg.as_tuple()?;
		count = arguments.len();
	} else if arg.is_float() {
		arguments = vec!(arg.as_float()?.into());
		count = 1;
	} else if arg.is_int() {
		arguments = vec!((arg.as_int()? as f64).into());
		count = 1;
	} else {
		return Err(EvalexprError::CustomMessage("Expected numbers".to_string()));
	}

	let output: Value;
	match count {
		1 => {
			let argument = &arguments[0];
			if !argument.is_number() {
				return Err(EvalexprError::CustomMessage("Expected number".to_string()));
			}
			let number = if argument.is_float() { argument.as_float()? } else { argument.as_int()? as f64 };
			output = number.ln().into();
		},
		2 => {
			let arg_value = &arguments[0];
			let arg_base = &arguments[1];
			if !(arg_value.is_number() && arg_base.is_number()) {
				return Err(EvalexprError::CustomMessage("Expected two numbers".to_string()));
			}
			let value: f64 = if arg_value.is_float() { arg_value.as_float()? } else { arg_value.as_int()? as f64 };
			let base: f64 = if arg_base.is_float() { arg_base.as_float()? } else { arg_base.as_int()? as f64 };
			output = value.log(base).into();
		},
		_ => {
			return Err(EvalexprError::WrongFunctionArgumentAmount { expected: 2, actual: count });
		}
	}
	return Ok(output);
}

pub fn square_root(arg: &Value) -> Result<Value, EvalexprError> {
	if !arg.is_number() {
		return Err(EvalexprError::CustomMessage("Expected a number.".to_string()));
	}
	let value: f64 = if arg.is_float() { arg.as_float()? } else { arg.as_int()? as f64 };
	return Ok(value.sqrt().into());
}

//	Data Science
pub fn average(arg: &Value) -> Result<Value, EvalexprError> {
	let arguments = arg.as_tuple()?;
	let count = arguments.len() as i64;
	let mut is_float = false;
	let mut total_i = 0i64;
	let mut total_f = 0f64;

	for argument in arguments {
		if let Value::Float(float) = argument {
			if !is_float {
				total_f = total_i as f64;
				is_float = true;
			}
			total_f += float;
		} else if let Value::Int(int) = argument {
			if is_float {
				total_f += int as f64;
			} else {
				total_i += int;
			}
		}
	}
	
	let result_i: i64;
	let result_f: f64;
	if !is_float {
		is_float = total_i % count == 0;
		total_f = total_i as f64;
	}

	if is_float {
		result_f = total_f / (count as f64);
		return Ok(result_f.into());
	} else {
		result_i = total_i / count;
		return Ok(result_i.into());
	}
}

//	Radix conversion
pub fn binary(arg: &Value) -> Result<Value, EvalexprError> {
	if !arg.is_string() {
		let num = arg.as_int()?;
		let fmt = format!("0b{:b}", num);
		return Ok(fmt.into());
	}
	util::parse_radix("0b", 2, arg)
}

pub fn hexadecimal(arg: &Value) -> Result<Value, EvalexprError> {
	if !arg.is_string() {
		let num = arg.as_int()?;
		let fmt = format!("0x{:X}", num);
		return Ok(fmt.into());
	}
	util::parse_radix("0x", 16, arg)
}

pub fn octal(arg: &Value) -> Result<Value, EvalexprError> {
	if !arg.is_string() {
		let num = arg.as_int()?;
		let fmt = format!("{:#o}", num);
		return Ok(fmt.into());
	}
	util::parse_radix("0o", 8, arg)
}
