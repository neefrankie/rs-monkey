Rust implmentation of the [Monkey](https://interpreterbook.com/) language.

## 普拉特解析法到底在做什么？

普拉特解析法本质上在比较一个操作数两侧的操作符优先级，哪一侧的优先级更高，这个操作数就被吸附到哪一侧。以下面的表达式为例：

```text
a + b * c
```

我们先给操作符赋予优先级：

* 默认最低 0
* `+`: 1
* `*`: 2

在处理操作数 `a` 时，我们看一下 `a` 两侧的操作符优先级是什么：左侧没有，那相当于最低优先级 0，右侧是 `+`，显然右侧更高，则 `a` 被吸附到右侧。

接下来来看 `b`，左侧是 `+`，优先级为 1，右侧是 `*`，优先级为 2，显然 `b` 被吸附到右侧。

于是 `b` 和 `c` 先结合，然后作为一个整体再和 `a` 通过 `+` 结合，得到 `(a + (b * c))`。

我们可以想象每个操作符都是一块磁铁，有不同的吸引力，吸引力大的一侧就把操作数吸引过去，聚成一团。

沿着这个思路，加上括号以后怎么处理呢？例如：

```text
(5 + 5) * 2;
```

括号里的内容构成一个独立的世界，那就需要保证括号两侧没有任何磁力来吸附括号内的东西，于是我们可以手动把括号两侧的磁力清零，括号内当作一个独立的表达式，和括号外的操作数按照上述原则继续运算。

## 中文版 108 页翻译不精确

本页最下面一段的中文是：

> 这意味着需要为 `token.LPAREN` 注册一个 `infixParseFn`，这样就可以解析函数表达式了（无论它是标识符还是函数字面量）。然后检查与 `token.LPAREN` 关联的 `infixParseFn`，使用已解析的表达式作为参数调用它。最后在这个 `infixParseFn` 中解析参数列表。

这一段看中文理解起来比较困难，对照原文：

> The `add` is an identifier that's parsed by a `prefixParseFn`. And after the identifier comes a `token.LPAREN`, right between the identifier and the list of arguments, just in the middle, in infix position… Yes, we need to register an `infixParseFn` for `token.LPAREN`. This way we parse the expression that is the function (either an identifier, or a function literal), then check for an `infixParseFn` associated with `token.LPAREN` and call it with the already parsed expression as argument. And in this `infixParseFn` we can then parse the argument list.

原文核心逻辑：

1. `add` 是一个由 `prefixParseFn` 解析的标识符。
2. 在这个标识符后面紧跟着 `token.LPAREN`，它**位于函数名和参数列表之间**——这正是 **infix 位置**。
3. 因此，我们需要为 `token.LPAREN` 注册一个 `infixParseFn`。
4. 这样做之后，解析流程会是：
   - 先解析出“函数”表达式（可能是标识符或函数字面量）；
   - 然后发现下一个 token 是 `(`，就查找其对应的 `infixParseFn`；
   - 调用该函数，并把**已经解析好的函数表达式**作为参数传进去；
   - 在这个 `infixParseFn` 内部，再解析参数列表。

这是一个非常清晰的 “先左操作数，再中缀操作符，再处理右操作数”的 Pratt Parser 思维。

更准确的中文翻译建议：

> `add` 是一个由 `prefixParseFn` 解析的标识符。而在该标识符之后，紧接着是一个 `token.LPAREN` —— 它正好位于函数名和参数列表之间，处于典型的中缀（infix）位置。  
>   
> 因此，我们需要为 `token.LPAREN` 注册一个 `infixParseFn`。这样一来，解析器会先解析出表示函数的表达式（可以是标识符，也可以是函数字面量），然后发现下一个 token 是 `(`，于是查找与 `token.LPAREN` 关联的 `infixParseFn`，并将已经解析好的函数表达式作为参数传递给它。  
>   
> 最后，在这个 `infixParseFn` 内部，我们就可以解析括号内的参数列表了。

## 解析调用表达式时没有设置权重发生了什么？

在《2.8.5 调用表达式》一节中，没有设置 `token.LPAREN` 的权重前执行测试会出错，错误信息是这样：

```text
expected next token to be ), got , instea
no prefix parse function for , found
no prefix parse function for ) found
```

这个时候发生了什么？执行了哪些函数？书中没有讲，我们一步一步分析整个解析过程中的错误传播链。

我们以 `add(2, 3)` 为例。

### 1. 初始状态：

- 当前 token: `add` (Ident)
- Peek token: `(` (Lparen)

### 2. 解析 `add` 标识符
```rust
// parseExpression(LOWEST) 被调用
let left = parse_identifier(); // 返回 Identifier("add")
```

### 3. 关键的优先级检查失败
```rust
while !peek_token_is(SEMICOLON) && precedence < peek_precedence()
```

此时 `precedence = LOWEST (0)`，`peek_precedence()` 找 `(` 的优先级值，因为没有设置 Call 优先级，所以是 `LOWEST (0)`。因此 `0 < 0 = false`，循环不执行。

结果：`add` 被当作一个完整的表达式返回！

### 4. 解析器认为语句结束

- `add` 构成了一个完整的 `ExpressionStatement`
- 解析器移动到下一个语句的解析

### 5. 下一个表达式从 `(` 开始

- 现在当前 token 是 `(`
- 调用 `parseExpression(LOWEST)` 解析新的表达式

### 6. 寻找 `(` 的前缀解析函数

- `(` 的前缀解析函数是 `parse_grouped_expression()`
- 这个函数期望的模式是：`( expression )`

### 7. 解析分组表达式失败
```rust
// parse_grouped_expression():
self.next_token(); // 跳过 (
let expr = self.parse_expression(LOWEST); // 解析 2
self.next_token(); // 期望是 ), 但实际是 ,
if !self.expect_peek(RPAREN) { 
    // 报错: "expected next token to be ), got , instead"
}
```

### 8. 逗号的后续问题

- 即使前面没报错，当解析器遇到 `,` 时
- 会查找 `,` 的前缀解析函数（不存在）
- 报错: "no prefix parse function for , found"

### 为什么设置 `(` 的优先级为 `Call` 能解决问题？

当你设置：

```rust
precedences.insert(TokenType::LParen, Precedence::Call);
```

### 关键变化：

```rust
// 在解析完 add 后：
// precedence = LOWEST (0)  
// peek_precedence() for '(' = CALL (higher than LOWEST)
// 所以 0 < CALL_PRIORITY = true，进入循环体

// 执行中缀解析函数 parse_call_expression(left)
// 这个函数专门处理函数调用：identifier(args...)
```

### `parse_call_expression` 的正确行为

1. 消费 `(` token
2. 解析参数列表：`2, 3`
3. 期望并消费 `)` token
4. 返回 `CallExpression { function: add, arguments: [2, 3] }`

这个错误完美展示了 Pratt Parser 的核心思想：

- 优先级决定"粘性"：高优先级的操作符能"粘住"周围的表达式
- 没有优先级 = 没有粘性：`add` 和 `(2, 3)` 被当作两个独立的表达式
- 前缀 vs 中缀：`(` 在不同上下文中有不同含义
  - 作为前缀：分组表达式 `(expr)`
  - 作为中缀：函数调用 `func(args)`

### 伪代码模拟

遇到 `add` 时生成一条语句：

```text
parse_statement()
  parse_expression(0)
    parse_identifier()
    return
```

接下来解析新语句，指向 `(2`

```text
parse_statement()
  parse_expression(0)
    parse_grouped_expression() # now point to 2
      parse_integer_literal()
    expect_peek(RPAREN) # Error! Here we see a comma
```

循环重新开始，解析新语句 `,`

```text
parse_statement()
  parse_expression(0)
    # prefix parsing funciton for `,`? Not found. Error!
```

进入新循环，解析下一个语句 `3`

```text
parse_statement()
  parse_expression(0)
    parse_integer_literal()
    precedence 0 < peek precedence 0 ? # No
    return
```

然后就循环再次重新开始，解析新语句 `)`

```text
parse_statement()
  parse_expression(0)
    prefix funciton for `)`? Not found. Error!
```
