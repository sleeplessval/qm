use evalexpr::{
	Value,

	EvalexprError
};

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

pub fn hex(arg: &Value) -> Option<Value> {
	if !arg.is_string() {
		if arg.is_int() {
			let num = arg.as_int().ok().unwrap();
			let fmt = format!("0x{:X}", num);
			return Some(fmt.into());
		}
		return None;
	}
	let i_parse = arg.as_string().ok().unwrap();
	let parse = i_parse.strip_prefix("0x").unwrap_or(i_parse.as_str());

	let i_result = i64::from_str_radix(parse, 16);
	if i_result.is_err() { return None; }
	let result: Option<i64> = i_result.ok();
	if result.is_none() { return None; }

	let output: Value = result.unwrap().into();
	return Some(output);
}
