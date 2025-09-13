Rust Coding Exercise

# Objective:
- To come up with a flexible design and implementation for evaluating and serializing binary
expressions using Rust programming language. A typical binary expression would contain
two operands and an operator. An operand could either be a sub-expression or a constant
value.
Examples (Serialized binary expressions in string format):
"15.000000"
"(12.000000 + 3.000000)"
"((10.000000 + 2.000000) + 3.000000)"
"((10.000000 + 2.000000) + ((1.000000 + 1.000000) + (2.000000 - 1.000000)))"

Note all the above expressions evaluate to the same result, i.e., 15.000000

# Constraints:

● You will be provided a main.rs file. You are not expected to make any changes to the
main.rs. This file contains a set of test cases and your solution is expected to make
these test cases pass.

● Note that the provided main.rs will not compile due to the missing functionality and you
are expected to come up with the missing functionality.

● Please note that the additional outer brackets are there to eliminate the complexity of
operator precedence. In other words, brackets decide the precedence of operation. It
is important to get the design right such that serialization adds brackets at the right
places. For e.g., BODMAS etc. are not required as part of this implementation.
Expectations:

● You are expected to come up with your entire solution in a single rust file. All your
code is expected to reside within this new file. You will be sharing this file via email
once your code passes all the test cases in main.rs.

● You are expected to come up with a flexible design. Your design is expected to
accommodate new enhancements and pass corresponding additional test cases
without major design changes. You will be provided with a new main.rs file containing
the test cases for the new enhancements during the interview.

● You will be given a limited time during the interview to come up with the changes to
accommodate the new enhancements. More flexible your design is, you will require
much less time to implement the new enhancements.

● You are expected to use idiomatic rust wherever possible. Use inbuilt functionality from
the standard library and the language features as appropriate

● Consider performance as a mandatory requirement while coming up with the design.
In this exercise, evaluating an expression is a performance critical operation.