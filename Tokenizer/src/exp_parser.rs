use std::collections::HashMap;

pub enum Value {
    Int(i32),
    Str(String),
}

pub struct ExprParser<'a> {
    input: Vec<char>,
    pos: usize,
    variables: &'a HashMap<String, Value>,
    last_token_was_operator: bool,
}

impl<'a> ExprParser<'a> {
    pub fn new(expression: &str, variables: &'a HashMap<String, Value>) -> Self {
        Self {
            input: expression.chars().collect(),
            pos: 0,
            variables,
            last_token_was_operator: true,
        }
    }

    pub fn parse(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        let value = if self.peek() == Some('"') {
            self.parse_string_literal()
        } else {
            self.parse_expression().map(Value::Int)
        }?;

        self.skip_whitespace();
        if self.pos < self.input.len() {
            return Err("Unexpected characters at end of input".to_string());
        }

        Ok(value)
    }

    fn parse_expression(&mut self) -> Result<i32, String> {
        let mut value = self.parse_term()?;
        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('+') => {
                    if self.last_token_was_operator {
                        return Err(
                            "Not allowed to have consecutive addition (+) operators".to_string()
                        );
                    }
                    self.next();
                    self.last_token_was_operator = true;
                    value += self.parse_term()?;
                }
                Some('-') => {
                    if self.last_token_was_operator {
                        return Err(
                            "Not allowed to have consecutive subtraction (-) operators".to_string()
                        );
                    }
                    self.next();
                    self.last_token_was_operator = true;
                    value -= self.parse_term()?;
                }
                _ => break,
            }
            self.last_token_was_operator = false;
        }
        Ok(value)
    }

    fn parse_term(&mut self) -> Result<i32, String> {
        let mut value = self.parse_factor()?;
        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('*') => {
                    if self.last_token_was_operator {
                        return Err(
                            "Not allowed to have consecutive multiplication (*) operators"
                                .to_string(),
                        );
                    }
                    self.next();
                    self.last_token_was_operator = true;
                    value *= self.parse_factor()?;
                }
                _ => break,
            }
            self.last_token_was_operator = false;
        }
        Ok(value)
    }

    fn parse_factor(&mut self) -> Result<i32, String> {
        self.skip_whitespace();
        match self.peek() {
            Some('(') => {
                self.next();
                let value = self.parse_expression()?;
                self.skip_whitespace();
                if self.next() != Some(')') {
                    return Err("Expected ')' is missing".to_string());
                }
                Ok(value)
            }
            Some('-') => {
                if self.last_token_was_operator {
                    return Err("Multiple unary (-) operators not allowed".to_string());
                }
                self.last_token_was_operator = true;
                self.next();
                Ok(-self.parse_factor()?)
            }
            Some('+') => {
                if self.last_token_was_operator {
                    return Err("Multiple unary (+) operators not allowed".to_string());
                }
                self.last_token_was_operator = true;
                self.next();
                self.parse_factor()
            }
            Some(c) if c.is_ascii_digit() => {
                self.last_token_was_operator = false;
                self.parse_integer_literal()
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                self.last_token_was_operator = false;
                self.parse_identifier()
            }
            _ => Err("Invalid token in expression".to_string()),
        }
    }

    fn parse_integer_literal(&mut self) -> Result<i32, String> {
        let start = self.pos;
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.next();
        }
        let number: String = self.input[start..self.pos].iter().collect();
        if number.len() > 1 && number.starts_with('0') {
            return Err("Invalid number: leading zeros are not allowed".to_string());
        }
        number
            .parse::<i32>()
            .map_err(|_| "Invalid number format".to_string())
    }

    fn parse_string_literal(&mut self) -> Result<Value, String> {
        self.next();
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.next();
                return Ok(Value::Str(result));
            }
            result.push(c);
            self.next();
        }
        Err("Unterminated string literal".to_string())
    }

    fn parse_identifier(&mut self) -> Result<i32, String> {
        let start = self.pos;
        while self
            .peek()
            .map_or(false, |c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.next();
        }
        let name: String = self.input[start..self.pos].iter().collect();
        match self.variables.get(&name) {
            Some(Value::Int(n)) => Ok(*n),
            Some(Value::Str(_)) => Err(format!(
                "Cannot use string variable '{}' in arithmetic",
                name
            )),
            None => Err(format!("Variable '{}' not defined", name)),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek();
        self.pos += ch.is_some() as usize;
        ch
    }

    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |c| c.is_whitespace()) {
            self.next();
        }
    }
}
