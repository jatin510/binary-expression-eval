use std::fmt;
use std::rc::Rc;

/// Supported binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
        }
    }
}

/// Core expression types - either a constant value or a binary operation
#[derive(Debug, Clone)]
pub enum Expression {
    Constant(f64),
    Binary {
        operator: Operator,
        left: Rc<Expression>,
        right: Rc<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Constant(value) => write!(f, "{:.6}", value),
            Expression::Binary {
                operator,
                left,
                right,
            } => {
                write!(f, "({} {} {})", left, operator, right)
            }
        }
    }
}

/// Expression context that handles creation and evaluation
pub struct ExpressionContext;

impl ExpressionContext {
    /// Create a new expression context
    pub fn new() -> Self {
        Self
    }

    /// Create a constant expression
    pub fn new_constant_expression(value: f64) -> Expression {
        Expression::Constant(value)
    }

    /// Create a binary expression with the given operator and operands
    pub fn new_binary_expression(
        operator: Operator,
        left: Expression,
        right: Expression,
    ) -> Expression {
        Expression::Binary {
            operator,
            left: Rc::new(left),
            right: Rc::new(right),
        }
    }

    /// Evaluate an expression asynchronously
    pub async fn eval(&self, expression: &Expression) -> Result<f64, String> {
        // Wrap synchronous evaluation in async for API compatibility and future extensibility
        self.eval_sync(expression)
    }

    /// Synchronous evaluation - handles recursion naturally
    fn eval_sync(&self, expression: &Expression) -> Result<f64, String> {
        match expression {
            Expression::Constant(value) => Ok(*value),
            Expression::Binary {
                operator,
                left,
                right,
            } => {
                let left_val = self.eval_sync(left)?;
                let right_val = self.eval_sync(right)?;

                match operator {
                    Operator::Add => Ok(left_val + right_val),
                    Operator::Subtract => Ok(left_val - right_val),
                    Operator::Multiply => Ok(left_val * right_val),
                    Operator::Divide => {
                        if right_val == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                }
            }
        }
    }
}

impl Default for ExpressionContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_evaluation() {
        let context = ExpressionContext::new();
        let expr = ExpressionContext::new_constant_expression(42.0);

        let result = context.eval(&expr).await.unwrap();
        assert_eq!(result, 42.0);
    }

    #[tokio::test]
    async fn test_binary_expression_evaluation() {
        let context = ExpressionContext::new();

        let sub_expr = ExpressionContext::new_binary_expression(
            Operator::Add,
            ExpressionContext::new_constant_expression(10.0),
            ExpressionContext::new_constant_expression(5.0),
        );

        let result = context.eval(&sub_expr).await.unwrap();
        assert_eq!(result, 15.0);

        let main_expr = ExpressionContext::new_binary_expression(
            Operator::Subtract,
            sub_expr,
            ExpressionContext::new_constant_expression(3.0),
        );

        let result = context.eval(&main_expr).await.unwrap();
        assert_eq!(result, 12.0);
    }
}
