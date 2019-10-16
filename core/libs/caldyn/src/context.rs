#[allow(clippy::all)]
use std::collections::BTreeMap;
use std::fmt;

/// A context hold values for variables, that can be used to evaluate
/// expressions.
///
/// # Examples
///
/// ```
/// # use caldyn::Context;
/// let mut context = Context::new();
/// context.set("a", 3.0);
/// context.set("b", -2.5);
///
/// assert_eq!(context.get("a"), Some(3.0));
/// assert_eq!(context.get("d"), None);
/// ```
pub struct Context<'a> {
    values: BTreeMap<String, f64>,
    query: Option<Box<dyn Fn(&str) -> Option<f64> + 'a>>,
}

impl<'a> fmt::Debug for Context<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let query = match self.query {
            Some(_) => "Some(<boxed closure>)",
            None => "None",
        };
        fmt.debug_struct("Context")
            .field("value", &self.values)
            .field("query", &query)
            .finish()
    }
}

impl<'a> Context<'a> {
    /// Create a new empty context
    pub fn new() -> Context<'a> {
        Context {
            values: BTreeMap::new(),
            query: None,
        }
    }

    /// Set a variable with the given `name` and `value`. If the variable
    /// already exists, the value is updated.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caldyn::Context;
    /// let mut context = Context::new();
    /// context.set("a", 3.0);
    /// assert_eq!(context.get("a"), Some(3.0));
    /// assert_eq!(context.get("d"), None);
    ///
    /// context.set("d", 5.0);
    /// context.set("a", 8.0);
    /// assert_eq!(context.get("a"), Some(8.0));
    /// assert_eq!(context.get("d"), Some(5.0));
    /// ```
    pub fn set<S: Into<String>>(&mut self, name: S, value: f64) {
        self.values.insert(name.into(), value);
    }

    /// Get the value of the variable with `name`, or `None` if the variable
    /// is not defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caldyn::Context;
    /// let mut context = Context::new();
    /// context.set("a", 3.0);
    /// assert_eq!(context.get("a"), Some(3.0));
    /// assert_eq!(context.get("d"), None);
    /// ```
    pub fn get(&self, name: &str) -> Option<f64> {
        self.values
            .get(name)
            .cloned()
            .or_else(|| self.query.as_ref().and_then(|function| function(name)))
    }

    /// Set a `function` to be called when a variable is not found in the
    /// context.
    ///
    /// # Examples
    ///
    /// ```
    /// # use caldyn::Context;
    /// let mut context = Context::new();
    /// context.set("a", 3.0);
    /// assert_eq!(context.get("a"), Some(3.0));
    /// assert_eq!(context.get("b"), None);
    ///
    /// context.set_query(|name| {
    ///     if name == "a" || name == "b" {
    ///         Some(28.0)
    ///     } else {
    ///         None
    ///     }
    /// });
    ///
    /// // The function is only called for "b", because "a" is already present
    /// // in the context.
    /// assert_eq!(context.get("a"), Some(3.0));
    /// assert_eq!(context.get("b"), Some(28.0));
    ///
    /// // If we set the variable "b", the function will not be called
    /// context.set("b", 1.0);
    /// assert_eq!(context.get("b"), Some(1.0));
    /// ```
    pub fn set_query<F>(&mut self, function: F)
    where
        F: Fn(&str) -> Option<f64> + 'a,
    {
        self.query = Some(Box::new(function));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn variables() {
        let mut context = Context::new();
        context.set("a", 2.0);
        assert_eq!(context.get("a"), Some(2.0));
        assert_eq!(context.get("b"), None);

        context.set("a", 5.0);
        context.set("b", 1.0);
        assert_eq!(context.get("a"), Some(5.0));
        assert_eq!(context.get("b"), Some(1.0));
    }

    #[test]
    fn debug() {
        let mut context = Context::new();
        context.set("a", 2.4);

        let mut string = String::new();
        let _ = write!(string, "{:?}", context);
        assert_eq!(string, "Context { value: {\"a\": 2.4}, query: \"None\" }");

        context.set_query(|_| None);
        let mut string = String::new();
        let _ = write!(string, "{:?}", context);
        assert_eq!(
            string,
            "Context { value: {\"a\": 2.4}, query: \"Some(<boxed closure>)\" }"
        );
    }
}
