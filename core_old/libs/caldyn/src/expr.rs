#[allow(clippy::all)]
use std::iter::Peekable;
use std::str::Chars;

use crate::context::Context;
use crate::error::Error;

/// Get the math function associated with the given `name`
fn math_function(name: &str) -> Option<fn(f64) -> f64> {
    match name {
        "sqrt" => Some(f64::sqrt),
        "cbrt" => Some(f64::cbrt),
        "sin" => Some(f64::sin),
        "cos" => Some(f64::cos),
        "tan" => Some(f64::tan),
        "asin" => Some(f64::asin),
        "acos" => Some(f64::acos),
        "atan" => Some(f64::atan),
        "sinh" => Some(f64::sinh),
        "cosh" => Some(f64::cosh),
        "tanh" => Some(f64::tanh),
        "asinh" => Some(f64::asinh),
        "acosh" => Some(f64::acosh),
        "atanh" => Some(f64::atanh),
        "floor" => Some(f64::floor),
        "ceil" => Some(f64::ceil),
        "abs" => Some(f64::abs),
        "exp" => Some(f64::exp),
        "ln" => Some(f64::ln),
        "log2" => Some(f64::log2),
        "log10" => Some(f64::log10),
        _ => None,
    }
}

/// Ast nodes for the expressions
#[derive(Debug, Clone, PartialEq)]
enum Ast {
    /// A variable, to be resolved later
    Variable(String),
    /// A constant value
    Value(f64),
    /// <left> + <right>
    Add(Box<Ast>, Box<Ast>),
    /// <left> - <right>
    Sub(Box<Ast>, Box<Ast>),
    /// <left> * <right>
    Mul(Box<Ast>, Box<Ast>),
    /// <left> / <right>
    Div(Box<Ast>, Box<Ast>),
    /// <left> ^ <right>
    Exp(Box<Ast>, Box<Ast>),
    /// fn(<arg>)
    Function(fn(f64) -> f64, Box<Ast>),
}

impl Ast {
    /// Construct the AST for a vector of tokens in reverse polish notation.
    /// This function eats the tokens as it uses them
    fn from_tokens(tokens: &mut Vec<Token>, context: &str) -> Result<Ast, Error> {
        if let Some(token) = tokens.pop() {
            match token {
                Token::Value(value) => {
                    if let Some(func) = math_function(&value) {
                        let args = Box::new(Ast::from_tokens(tokens, " in function call")?);
                        Ok(Ast::Function(func, args))
                    } else if let Ok(number) = value.parse() {
                        Ok(Ast::Value(number))
                    } else if is_variable(&value) {
                        Ok(Ast::Variable(value))
                    } else {
                        Err(Error::ParseError(format!("invalid value {}", value)))
                    }
                }
                Token::Op(op) => {
                    let right = Box::new(Ast::from_tokens(tokens, " after operator")?);
                    let left = Box::new(Ast::from_tokens(tokens, " befor operator")?);
                    match op {
                        Op::Plus => Ok(Ast::Add(left, right)),
                        Op::Minus => Ok(Ast::Sub(left, right)),
                        Op::Mul => Ok(Ast::Mul(left, right)),
                        Op::Div => Ok(Ast::Div(left, right)),
                        Op::Exp => Ok(Ast::Exp(left, right)),
                    }
                }
                other => panic!("Internal error: got {:?} token after shunting yard", other),
            }
        } else {
            Err(Error::ParseError(format!("empty expression{}", context)))
        }
    }

    /// Recursively evaluate the AST in a given context
    #[allow(clippy::all)]
    fn eval(&self, context: Option<&Context>) -> Result<f64, Error> {
        match *self {
            Ast::Variable(ref name) => context
                .and_then(|context| context.get(name))
                .ok_or(Error::NameError(format!("name '{}' is not defined", name))),
            Ast::Value(number) => Ok(number),
            Ast::Add(ref left, ref right) => Ok(left.eval(context)? + right.eval(context)?),
            Ast::Sub(ref left, ref right) => Ok(left.eval(context)? - right.eval(context)?),
            Ast::Mul(ref left, ref right) => Ok(left.eval(context)? * right.eval(context)?),
            Ast::Div(ref left, ref right) => Ok(left.eval(context)? / right.eval(context)?),
            Ast::Exp(ref left, ref right) => Ok(left.eval(context)?.powf(right.eval(context)?)),
            Ast::Function(ref func, ref arg) => Ok(func(arg.eval(context)?)),
        }
    }

    /// If the AST node correspond to a constant, get `Some(constant)`. Else,
    /// get `None`
    fn value(&self) -> Option<f64> {
        if let Ast::Value(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    /// Optimize the AST by doing constants propagation
    fn optimize(self) -> Ast {
        match self {
            Ast::Variable(_) | Ast::Value(_) => self,
            Ast::Function(func, arg) => {
                let arg = arg.optimize();
                if let Some(arg) = arg.value() {
                    return Ast::Value(func(arg));
                }
                return Ast::Function(func, Box::new(arg));
            }
            Ast::Add(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                if let Some(left) = left.value() {
                    if let Some(right) = right.value() {
                        return Ast::Value(left + right);
                    }
                }
                return Ast::Add(Box::new(left), Box::new(right));
            }
            Ast::Sub(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                if let Some(left) = left.value() {
                    if let Some(right) = right.value() {
                        return Ast::Value(left - right);
                    }
                }
                return Ast::Sub(Box::new(left), Box::new(right));
            }
            Ast::Mul(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                if let Some(left) = left.value() {
                    if let Some(right) = right.value() {
                        return Ast::Value(left * right);
                    }
                }
                return Ast::Mul(Box::new(left), Box::new(right));
            }
            Ast::Div(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                if let Some(left) = left.value() {
                    if let Some(right) = right.value() {
                        return Ast::Value(left / right);
                    }
                }
                return Ast::Div(Box::new(left), Box::new(right));
            }
            Ast::Exp(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                if let Some(left) = left.value() {
                    if let Some(right) = right.value() {
                        return Ast::Value(left.powf(right));
                    }
                }
                return Ast::Exp(Box::new(left), Box::new(right));
            }
        }
    }
}

/// A parsed and optimized mathematical expression.
///
/// # Examples
/// ```
/// # use caldyn::{Expr, Context};
/// let expr = Expr::parse("3 + 5 * 2").unwrap();
/// assert_eq!(expr.eval(None), Ok(13.0));
///
/// let mut context = Context::new();
/// context.set("a", 42.0);
/// let expr = Expr::parse("-2 * a").unwrap();
/// assert_eq!(expr.eval(&context), Ok(-84.0));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    ast: Ast,
}

impl Expr {
    /// Parse the given mathematical `expression` into an `Expr`.
    ///
    /// # Examples
    /// ```
    /// # use caldyn::Expr;
    /// // A valid expression
    /// assert!(Expr::parse("3 + 5 * 2").is_ok());
    /// // an invalid expression
    /// assert!(Expr::parse("3eff + 5 * 2").is_err());
    /// ```
    pub fn parse(expression: &str) -> Result<Expr, Error> {
        let mut lexer = Lexer::new(expression);
        let mut output = Vec::new();
        let mut operators = Vec::new();

        'tokens: while let Some(token) = lexer.next_token()? {
            match token {
                Token::Value(ref name) if math_function(name).is_some() => {
                    operators.push(token.clone());
                }
                Token::Value(_) => output.push(token),
                Token::Op(o1) => {
                    'operators: while let Some(token) = operators.last().cloned() {
                        match token {
                            Token::Op(o2) => {
                                let pop_me =
                                    o1.is_left_associative() && o1.precedence() <= o2.precedence();
                                let pop_me = pop_me
                                    || o1.is_right_associative()
                                        && o1.precedence() < o2.precedence();
                                if pop_me {
                                    operators.pop();
                                    output.push(Token::Op(o2));
                                } else {
                                    break 'operators;
                                }
                            }
                            _ => break 'operators,
                        }
                    }
                    operators.push(token)
                }
                Token::LParen => operators.push(token),
                Token::RParen => {
                    while let Some(token) = operators.pop() {
                        match token {
                            Token::LParen => {
                                let next_is_fn =
                                    if let Some(&Token::Value(ref name)) = operators.last() {
                                        math_function(name).is_some()
                                    } else {
                                        false
                                    };

                                if next_is_fn {
                                    output.push(operators.pop().expect("emtpy operator stack"));
                                }
                                continue 'tokens;
                            }
                            Token::Op(_) => output.push(token),
                            other => panic!("Internal bug: found {:?} in operators stack", other),
                        }
                    }
                    return Err(Error::ParseError("mismatched parenthesis".into()));
                }
            }
        }

        while let Some(token) = operators.pop() {
            match token {
                Token::LParen => return Err(Error::ParseError("mismatched parenthesis".into())),
                Token::Op(_) => output.push(token),
                other => panic!("Internal bug: found {:?} in operators stack", other),
            }
        }

        let ast = Ast::from_tokens(&mut output, "")?;
        if output.is_empty() {
            Ok(Expr {
                ast: ast.optimize(),
            })
        } else {
            Err(Error::ParseError(
                "extra data at the end of the expression".into(),
            ))
        }
    }

    /// Evaluate the expression in a given optional `context`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caldyn::{Expr, Context};
    /// let expr = Expr::parse("3 + 5 * 2").unwrap();
    /// assert_eq!(expr.eval(None), Ok(13.0));
    ///
    /// let expr = Expr::parse("3 + a").unwrap();
    ///
    /// let mut context = Context::new();
    /// context.set("a", -5.0);
    /// assert_eq!(expr.eval(&context), Ok(-2.0));
    /// context.set("a", 2.0);
    /// assert_eq!(expr.eval(&context), Ok(5.0));
    /// ```
    pub fn eval<'a, C>(&self, context: C) -> Result<f64, Error>
    where
        C: Into<Option<&'a Context<'a>>>,
    {
        self.ast.eval(context.into())
    }
}

/// Allowed operators in the algorithm
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Plus,
    Minus,
    Mul,
    Div,
    Exp,
}

impl Op {
    /// Get the operator precedence. Operators with higher precedence should be
    /// evaluated first.
    #[allow(clippy::all)]
    fn precedence(&self) -> u8 {
        match *self {
            Op::Plus | Op::Minus => 1,
            Op::Mul | Op::Div => 2,
            Op::Exp => 3,
        }
    }

    /// Check if the operator is left associative
    #[allow(clippy::all)]
    fn is_left_associative(&self) -> bool {
        match *self {
            Op::Plus | Op::Minus | Op::Mul | Op::Div => true,
            Op::Exp => false,
        }
    }

    /// Check if the operator is right associative
    #[allow(clippy::all)]
    fn is_right_associative(&self) -> bool {
        !self.is_left_associative()
    }
}

/// Possible tokens to find in the input string
#[derive(Debug, Clone, PartialEq)]
enum Token {
    /// Any literal: value or variables
    Value(String),
    /// A boolean operator
    Op(Op),
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
}

/// An helper struct for lexing the input
struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(string: &str) -> Lexer {
        Lexer {
            input: string.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        if let Some(c) = self.input.next() {
            let token = match c {
                ' ' | '\t' | '\n' | '\r' => return self.next_token(),
                c if is_value_start(c) => {
                    let mut ident = String::new();
                    ident.push(c);
                    'value: while let Some(&c) = self.input.peek() {
                        if is_value_part(c) {
                            self.input.next();
                            ident.push(c);
                        } else {
                            break 'value;
                        }
                    }
                    // Special case to handle numbers starting with + or -
                    if ident == "+" {
                        Token::Op(Op::Plus)
                    } else if ident == "-" {
                        Token::Op(Op::Minus)
                    } else {
                        Token::Value(ident)
                    }
                }
                '*' => Token::Op(Op::Mul),
                '/' => Token::Op(Op::Div),
                '^' => Token::Op(Op::Exp),
                '(' => Token::LParen,
                ')' => Token::RParen,
                other => {
                    return Err(Error::ParseError(format!(
                        "unexpected characted in input: {}",
                        other
                    )))
                }
            };
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }
}

/// Check if `c` can appear at the first character of a value
fn is_value_start(c: char) -> bool {
    c == '+' || c == '-' || c.is_digit(10) || is_variable_start(c)
}

/// Check if `c` can appear inside a value
fn is_value_part(c: char) -> bool {
    c == '+' || c == '-' || c == '.' || is_variable_part(c)
}

/// Check if `c` can appear at the first character of a variable
fn is_variable_start(c: char) -> bool {
    c == '_' || (c.is_ascii() && c.is_alphabetic())
}

/// Check if `c` can appear inside a variable
fn is_variable_part(c: char) -> bool {
    c == '.' || c == '_' || c == '[' || c == ']' || (c.is_ascii() && c.is_alphanumeric())
}

/// Check if `ident` is a valid variable name
///
/// # Examples
///
/// ```
/// # use caldyn::is_variable;
///
/// assert_eq!(is_variable("__abc3"), true);
/// assert_eq!(is_variable("34zb"), false);
/// ```
pub fn is_variable(ident: &str) -> bool {
    let mut chars = ident.chars();
    // Check first char
    if !chars.next().map_or(false, is_variable_start) {
        return false;
    }
    // Check all others
    for c in chars {
        if !is_variable_part(c) {
            return false;
        }
    }
    return true;
}

/// Evaluate a single expression from `input`.
///
/// Returns `Ok(result)` if the evaluation is successful, or `Err(cause)` if
/// parsing or evaluating the expression failed.
///
/// # Example
///
/// ```
/// use caldyn::{eval, Context};
///
/// assert_eq!(eval("45 - 2^3", None), Ok(37.0));
///
/// let mut context = Context::new();
/// context.set("a", -5.0);
/// assert_eq!(eval("3 * a", &context), Ok(-15.0));
/// ```
pub fn eval<'a, C>(input: &str, context: C) -> Result<f64, Error>
where
    C: Into<Option<&'a Context<'a>>>,
{
    Expr::parse(input).and_then(|expr| expr.eval(context))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn idents() {
        assert!(is_variable_start('c'));
        assert!(is_variable_start('Z'));
        assert!(is_variable_start('_'));
        assert!(is_variable_start('f'));

        assert!(!is_variable_start('3'));
        assert!(!is_variable_start('à'));
        assert!(!is_variable_start('@'));
        assert!(!is_variable_start(']'));
        assert!(!is_variable_start('['));
        assert!(!is_variable_start('.'));

        assert!(is_variable_part('c'));
        assert!(is_variable_part('Z'));
        assert!(is_variable_part('_'));
        assert!(is_variable_part('f'));
        assert!(is_variable_part('3'));
        assert!(is_variable_part('['));
        assert!(is_variable_part(']'));
        assert!(is_variable_part('.'));

        assert!(!is_variable_part('à'));
        assert!(!is_variable_part('@'));
        assert!(!is_variable_part('{'));
        assert!(!is_variable_part('}'));
        assert!(!is_variable_part('('));
        assert!(!is_variable_part(')'));
        assert!(!is_variable_part('?'));
        assert!(!is_variable_part('&'));
        assert!(!is_variable_part('$'));

        assert!(is_variable("_______"));
        assert!(is_variable("abc"));
        assert!(is_variable("ab.gd_hc"));
        assert!(is_variable("abc[2as]"));
        assert!(is_variable("abc["));
        assert!(is_variable("abc]"));
        assert!(is_variable("abc[90]"));
        assert!(is_variable("ab.c[90]"));
        assert!(is_variable("a__45__bc"));
        assert!(!is_variable("a-bc"));
        assert!(!is_variable("@bc"));
        assert!(!is_variable("6bc"));
    }

    #[test]
    fn values() {
        assert!(is_value_start('c'));
        assert!(is_value_start('Z'));
        assert!(is_value_start('_'));
        assert!(is_value_start('f'));
        assert!(is_value_start('3'));
        assert!(is_value_start('+'));
        assert!(is_value_start('-'));

        assert!(!is_value_start('à'));
        assert!(!is_value_start('@'));

        assert!(is_value_part('c'));
        assert!(is_value_part('Z'));
        assert!(is_value_part('_'));
        assert!(is_value_part('f'));
        assert!(is_value_part('3'));
        assert!(is_value_part('-'));
        assert!(is_value_part('+'));
        assert!(is_value_part('.'));

        assert!(!is_value_part('à'));
        assert!(!is_value_part('@'));
    }

    #[test]
    fn parse() {
        assert!(Expr::parse("3 + +5e67").is_ok());
        assert!(Expr::parse("(3 + -5)*45").is_ok());
        assert!(Expr::parse("(3. + 5.0)*\t\n45").is_ok());
        assert!(Expr::parse("(3 + 5^5e-6)*45").is_ok());
        assert!(Expr::parse("sin(34.0) ^ sqrt(28.0)").is_ok());
        assert!(Expr::parse("abc[ty8789]").is_ok());
    }

    #[test]
    fn eval() {
        assert_eq!(super::eval("3 + 5", None), Ok(8.0));
        assert_eq!(super::eval("2 - 5", None), Ok(-3.0));
        assert_eq!(super::eval("2 * 5", None), Ok(10.0));
        assert_eq!(super::eval("10 / 5", None), Ok(2.0));
        assert_eq!(super::eval("2 ^ 3", None), Ok(8.0));
        assert_eq!(super::eval("-3", None), Ok(-3.0));
        assert_eq!(super::eval("25 + -3", None), Ok(22.0));
        assert_eq!(super::eval("25 - -3", None), Ok(28.0));
        assert_eq!(super::eval("25 - -3", None), Ok(28.0));
        assert_eq!(super::eval("3 + 5 * 2", None), Ok(13.0));
        assert_eq!(super::eval("sqrt(9)", None), Ok(3.0));
        assert_eq!(super::eval("sin(18.0) * 3", None), Ok(3.0 * f64::sin(18.0)));

        let mut context = Context::new();
        context.set("a", 1.0);
        context.set("b", 2.0);
        assert_eq!(super::eval("2 * a", &context), Ok(2.0));
        assert_eq!(super::eval("(a + b)^2", &context), Ok(9.0));

        let result = super::eval("2 * z", &context);
        assert_eq!(
            result.err().unwrap().description(),
            "name 'z' is not defined"
        );
        let result = super::eval("2 * a", None);
        assert_eq!(
            result.err().unwrap().description(),
            "name 'a' is not defined"
        );
    }

    #[test]
    fn optimize() {
        let Expr { ast } = Expr::parse("3 + 5").unwrap();
        assert_eq!(ast.value(), Some(8.0));

        let Expr { ast } = Expr::parse("(3 + 5^2)*45").unwrap();
        assert_eq!(ast.value(), Some(1260.0));

        let Expr { ast } = Expr::parse("sqrt(9)").unwrap();
        assert_eq!(ast.value(), Some(3.0));
    }
}
