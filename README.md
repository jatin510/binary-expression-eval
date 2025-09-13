# Binary Expression Evaluator

A flexible and efficient Rust implementation for evaluating and serializing binary expressions with async support.

## Features

- **🧮 Expression Evaluation**: Supports binary operations (Add, Subtract) with nested expressions
- **🔄 Async API**: Future-ready async evaluation interface
- **📝 Serialization**: Clean string representation with proper formatting
- **⚡ Memory Efficient**: Uses `Rc<Expression>` for optimal memory sharing of sub-expressions
- **🧪 Comprehensive Testing**: Full test coverage with multiple complexity levels

## Expression Format

The evaluator handles expressions in the following formats:

```rust
// Constants
"15.000000"

// Binary operations  
"(12.000000 + 3.000000)"

// Nested expressions
"((10.000000 + 2.000000) + 3.000000)"

// Complex nested operations
"((10.000000 + 2.000000) + ((1.000000 + 1.000000) + (2.000000 - 1.000000)))"
```

## Usage

```rust
use expression::{ExpressionContext, Operator};

#[tokio::main]
async fn main() {
    let context = ExpressionContext::new();

    // Create a simple constant
    let expr1 = ExpressionContext::new_constant_expression(15.0);
    println!("{}", expr1.to_string()); // "15.000000"
    
    let result = context.eval(&expr1).await.unwrap();
    println!("Result: {}", result); // 15.0

    // Create a binary expression
    let expr2 = ExpressionContext::new_binary_expression(
        Operator::Add,
        ExpressionContext::new_constant_expression(12.0),
        ExpressionContext::new_constant_expression(3.0),
    );
    
    println!("{}", expr2.to_string()); // "(12.000000 + 3.000000)"
    let result = context.eval(&expr2).await.unwrap();
    println!("Result: {}", result); // 15.0
}
```

## Architecture

### Core Components

- **`Expression`**: Enum representing either constants or binary operations
- **`Operator`**: Supported operations (Add, Subtract)  
- **`ExpressionContext`**: Main interface for creating and evaluating expressions

### Design Principles

- **Separation of Concerns**: Async wrapper with synchronous evaluation core
- **Memory Efficiency**: Shared sub-expressions using `Rc<Expression>`
- **Extensibility**: Easy to add new operators and expression types
- **Performance**: Optimized for evaluation-heavy workloads

## Running Tests

```bash
cargo test
```

## Performance Considerations

- Uses `Rc<Expression>` for efficient memory sharing of reused sub-expressions
- Synchronous evaluation core avoids unnecessary async overhead
- Natural recursion handling without complex boxing patterns

## Future Extensibility

The design supports easy addition of:
- New operators (Multiply, Divide, etc.)
- Expression caching mechanisms
- I/O-based operations (database lookups, API calls)
- Different number types and precision levels

## Requirements

- Rust 2024 edition
- Tokio runtime for async functionality
