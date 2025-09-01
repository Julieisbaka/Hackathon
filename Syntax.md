
# Math Script Language Syntax

---

## Variables

Declare a variable:

```ms
x = 5
```

---

## Conditionals

Add `{}` after an expression to specify a condition:

```ms
f(x) = x^2 {x > 0}
```

---

## Operators

- **Comparison:**
  - `=` inside `{}` for equality (e.g. `{x = 2}`)
  - `!` for negation (e.g. `{x != 2}` means x ≠ 2)
  - `>`, `<`, `>=`, `<=` for inequalities
- **Arithmetic:**
  - `+`, `-`, `*`, `/`, `^` for addition, subtraction, multiplication, division, exponentiation
- **Factorial:**
  - `n!` or `x!` (no space) for factorial
- **Absolute Value:**
  - `|expr|` for absolute value

---

## Grouping

Use parentheses `()` to group expressions:

```ms
(2 + 3) * 4
```

---

## Functions

- Define a function:

 ```ms
 f(x) = x^2 + 1
 ```

- Multiple variables:

 ```ms
 g(x, y) = x + y
 ```

- Call other functions:

 ```ms
 h(x) = f(x) + 2
 ```

---

## Arrays, Lists, and Matrices

Use square brackets:

```ms
a = [1, 2, 3]
m = [[1, 2], [3, 4]]
```

---

## Other Syntax

- **Anonymous functions:**

  ```ms
  (x) => x^2 + 1
  ```

- **Assignment:**

  ```ms
  y = 2 * x + 1
  ```

- **Docstrings:**
  Use triple quotes for docstrings:

  ```ms
  """
  This is a docstring
  """
  ```

- **Comments:**
  Use `#` for single-line comments:

  ```ms
  # This is a comment
  ```

- **Import:**

  ```ms
  import "otherfile.ms"
  ```

---

## Built-in Functions

All built-in functions can be called as `name(args)`.

- **Trigonometric:**
  - `sin(x)`, `cos(x)`, `tan(x)`, `sec(x)`, `csc(x)`, `cot(x)`
  - Inverse: `asin(x)`, `acos(x)`, `atan(x)`, `asec(x)`, `acsc(x)`, `acot(x)`
  - Hyperbolic: `sinh(x)`, `cosh(x)`, `tanh(x)`, `sech(x)`, `csch(x)`, `coth(x)`
  - Inverse hyperbolic: `asinh(x)`, `acosh(x)`, `atanh(x)`, `asech(x)`, `acsch(x)`, `acoth(x)`

- **Logarithms:**
  - `ln(x)` — natural log
  - `log(x)` — base 10 log

- **Error functions:**
  - `erf(x)` — error function
  - `erfc(x)` — complementary error function

- **Other math:**
  - `abs(x)` — absolute value
  - `sqrt(x)` — square root
  - `exp(x)` — exponential
  - `min(a, b, ...)`, `max(a, b, ...)`
  - `sum([a, b, ...])`, `prod([a, b, ...])`
  - `real(z)`, `imag(z)` — real/imaginary part
  - `conj(z)` — complex conjugate
  - `arg(z)` — argument/angle of complex
  - `re(z)`, `im(z)` — aliases for real/imag

- **Printing and Logging:**
  - `print(args...)` — print to output
  - `log(level, msg)` — log with level (INFO, WARN, ERROR, DEBUG)

- **Derivatives:**
  - `d/dx f(x)` — derivative
  - `dy/dx f(x)` — partial derivative
  - Prime notation: `f'(x)`, `f''(x)`, `f'''(x)`
  - `f[x]'` — x-th derivative

- **Other:**
  - `import "file.ms"` — import another file

---

## Calculus

- Derivatives:
  - `d/dx f(x)`
  - `dy/dx f(x)`
  - Prime notation: `f'(x)`, `f''(x)`, `f'''(x)`
  - `f[x]'` for the x-th derivative

---

## Constants

- `pi` (π = 3.141592...)
- `e` (Euler's number)
- `i`, `j`, `k` (imaginary units)

---

## Examples

```ms
# Function with condition
f(x) = x^2 {x > 0}

# Array and matrix
arr = [1, 2, 3]
mat = [[1, 2], [3, 4]]

# Built-in math
z = sin(pi/2) + cos(0)

# Print and log
print("Hello, world!", z)
log("INFO", "Computation done")

# Derivative
f'(x)

g(x, y) = x^2 + y^2
# Partial derivative
dy/dx g(x, y)

# Anonymous function
square = (x) => x^2
print(square(5))

# Import
import "otherfile.ms"
```

---

Honestly if all these features work that would be a miracle.
