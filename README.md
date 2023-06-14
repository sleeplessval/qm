
# quickmaths

A small, pretty command line calculator.

Features expression evaluation, variable assignment, an interactive mode, pretty
output, and a variety of preset constants and functions focused on pure mathematics,
data science, and computer science.

## Constants

| Name           | Tokens     | Type  | Value               |
| -------------- | ---------- | ----- | ------------------- |
| Euler's Number | `e`        | `f64` | `2.718281828459045` |
| Golden Ratio   | `phi`, `ϕ` | `f64` | `1.618033988749895` |
| Pi             | `pi`, `π`  | `f64` | `3.141592653589793` |
| Root Two       | `√2`       | `f64` | `1.414213562373095` |


## Built-in functions

### Mathematics

- `fix`: fix a float to *n* digits.
- `log`: logarithm. Defaults to natural logarithm (ln).
- `sqrt`/`√`: square root. 

### Data Science

- `avg`: average an arbitrary set of numbers.

### Computer Science

- `bin`: convert an integer to a binary string or vice versa.
- `hex`: convert an integer to a hexadecimal string or vice versa.
- `oct`: convert an integer to an octal string or vice versa.

## Libraries

- [evalexpr](https://crates.io/crates/evalexpr)
- [termion](https://crates.io/crates/termion)

