use super::{from_message, Message};
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////
/// GUI
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiExpression {
    pub expression: String,
}
from_message!(GuiExpression, Message::CalculatorGuiExpression);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiResult {
    pub result: String,
}
from_message!(GuiResult, Message::CalculatorGuiResult);
