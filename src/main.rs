mod expression;

#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests {

    use crate::expression::{ExpressionContext, Operator};

    #[tokio::test]
    async fn serialization_and_eval_works() {
        let context = ExpressionContext::new();

        let e1 = ExpressionContext::new_constant_expression(15.0);

        assert_eq!(e1.to_string(), String::from("15.000000"));
        assert_eq!(context.eval(&e1).await.unwrap(), 15.000000_f64);

        let e2 = ExpressionContext::new_binary_expression(
            Operator::Add,
            ExpressionContext::new_constant_expression(12.0),
            ExpressionContext::new_constant_expression(3.0),
        );

        assert_eq!(e2.to_string(), String::from("(12.000000 + 3.000000)"));
        assert_eq!(context.eval(&e2).await.unwrap(), 15.000000_f64);

        let ce = ExpressionContext::new_binary_expression(
            Operator::Add,
            ExpressionContext::new_constant_expression(10.0),
            ExpressionContext::new_constant_expression(2.0),
        );

        let e3 = ExpressionContext::new_binary_expression(
            Operator::Add,
            ce.clone(),
            ExpressionContext::new_constant_expression(3.0),
        );

        assert_eq!(
            e3.to_string(),
            String::from("((10.000000 + 2.000000) + 3.000000)")
        );
        assert_eq!(context.eval(&e3).await.unwrap(), 15.000000_f64);

        let e4 = ExpressionContext::new_binary_expression(
            Operator::Add,
            ce,
            ExpressionContext::new_binary_expression(
                Operator::Add,
                ExpressionContext::new_binary_expression(
                    Operator::Add,
                    ExpressionContext::new_constant_expression(1.0),
                    ExpressionContext::new_constant_expression(1.0),
                ),
                ExpressionContext::new_binary_expression(
                    Operator::Subtract,
                    ExpressionContext::new_constant_expression(2.0),
                    ExpressionContext::new_constant_expression(1.0),
                ),
            ),
        );
        assert_eq!(
            e4.to_string(),
            String::from(
                "((10.000000 + 2.000000) + ((1.000000 + 1.000000) + (2.000000 - 1.000000)))"
            )
        );
        assert_eq!(context.eval(&e4).await.unwrap(), 15.000000_f64);
    }
}
