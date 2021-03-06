# 自定义标量

`Async-graphql`已经内置了绝大部分常用的标量类型，同时你也能自定义标量。

实现`Async-graphql::Scalar`即可自定义一个标量，你只需要实现一个解析函数和输出函数。

下面的例子定义一个64位整数标量，但它的输入输出都是字符串。 (`Async-graphql`已经内置了对64位整数的支持，正是采用字符串作为输入输出)

```rust
use async_graphql::*;

struct StringNumber(i64);

impl Scalar for StringNumber {
    fn type_name() -> &'static str {
        // 类型名
        "StringNumber"
    }

    fn parse(value: &Value) -> Option<Self> {
        if let Value::String(value) = value {
            // 解析整数
            value.parse().ok()
        } else {
            // 类型不匹配则直接返回None
            None
        }
    }
}

// 这个宏必须调用
impl_scalar!(StringNumber);

```