use evalexpr::{
	Value,
	
	EvalexprError
};

pub(crate) fn parse_radix(prefix: &str, base: u32, arg: &Value) -> Result<Value, EvalexprError> {
	let i_parse = arg.as_string()?;
	let parse = i_parse.strip_prefix(prefix).unwrap_or(i_parse.as_str());

	let i_result = i64::from_str_radix(parse, base);
	if i_result.is_err() {
		return Err(EvalexprError::CustomMessage("failed to parse integer from string".to_string()));
	}
	let result = i_result.ok();

	return Ok(result.unwrap().into());
}
