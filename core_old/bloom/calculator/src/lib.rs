use bloom_error::BloomError;
use bloom_messages::{calculator, Message};

pub fn evaluate(input: calculator::GuiExpression) -> Result<Message, BloomError> {
    let result = caldyn::eval(&input.expression, None)?;

    let result = calculator::GuiResult {
        result: result.to_string(),
    }
    .into();

    return Ok(result);
}
