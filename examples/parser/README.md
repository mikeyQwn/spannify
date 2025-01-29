# Parser

This examples shows how `spannify` can be used in
a complex environment with lots of function calls
that are hard to track.

This example outputs the following trace:

```plaintext
┌parse_expression: current=`Number(10)`
| ┌advance_tokens: current=`Number(10)`
| └advance_tokens: current=`Number(10)`
| ┌parse_infix_expression: current=`Add`
| ¦ ┌advance_tokens: current=`Add`
| ¦ └advance_tokens: current=`Add`
| ¦ ┌parse_expression: current=`Number(13)`
| ¦ └parse_expression: current=`Number(13)`
| └parse_infix_expression: current=`Add`
| ┌advance_tokens: current=`Number(13)`
| └advance_tokens: current=`Number(13)`
| ┌parse_infix_expression: current=`Sub`
| ¦ ┌advance_tokens: current=`Sub`
| ¦ └advance_tokens: current=`Sub`
| ¦ ┌parse_expression: current=`Number(23)`
| ¦ ┆ ┌advance_tokens: current=`Number(23)`
| ¦ ┆ └advance_tokens: current=`Number(23)`
| ¦ ┆ ┌parse_infix_expression: current=`Div`
| ¦ ┆ ┊ ┌advance_tokens: current=`Div`
| ¦ ┆ ┊ └advance_tokens: current=`Div`
| ¦ ┆ ┊ ┌parse_expression: current=`LParen`
| ¦ ┆ ┊ | ┌parse_grouped_expression: current=`LParen`
| ¦ ┆ ┊ | ¦ ┌advance_tokens: current=`LParen`
| ¦ ┆ ┊ | ¦ └advance_tokens: current=`LParen`
| ¦ ┆ ┊ | ¦ ┌parse_expression: current=`Number(103)`
| ¦ ┆ ┊ | ¦ ┆ ┌advance_tokens: current=`Number(103)`
| ¦ ┆ ┊ | ¦ ┆ └advance_tokens: current=`Number(103)`
| ¦ ┆ ┊ | ¦ ┆ ┌parse_infix_expression: current=`Sub`
| ¦ ┆ ┊ | ¦ ┆ ┊ ┌advance_tokens: current=`Sub`
| ¦ ┆ ┊ | ¦ ┆ ┊ └advance_tokens: current=`Sub`
| ¦ ┆ ┊ | ¦ ┆ ┊ ┌parse_expression: current=`Number(10)`
| ¦ ┆ ┊ | ¦ ┆ ┊ └parse_expression: current=`Number(10)`
| ¦ ┆ ┊ | ¦ ┆ └parse_infix_expression: current=`Sub`
| ¦ ┆ ┊ | ¦ └parse_expression: current=`Number(103)`
| ¦ ┆ ┊ | ¦ ┌advance_tokens: current=`Number(10)`
| ¦ ┆ ┊ | ¦ └advance_tokens: current=`Number(10)`
| ¦ ┆ ┊ | └parse_grouped_expression: current=`LParen`
| ¦ ┆ ┊ └parse_expression: current=`LParen`
| ¦ ┆ └parse_infix_expression: current=`Div`
| ¦ └parse_expression: current=`Number(23)`
| └parse_infix_expression: current=`Sub`
| ┌advance_tokens: current=`RBrace`
| └advance_tokens: current=`RBrace`
| ┌parse_infix_expression: current=`Add`
| ¦ ┌advance_tokens: current=`Add`
| ¦ └advance_tokens: current=`Add`
| ¦ ┌parse_expression: current=`Number(1)`
| ¦ └parse_expression: current=`Number(1)`
| └parse_infix_expression: current=`Add`
└parse_expression: current=`Number(10)`
```
