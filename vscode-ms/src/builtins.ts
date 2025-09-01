export type Builtin = {
  kind: 'function' | 'constant';
  name: string;
  detail: string;
  documentation: string;
  parameters?: string[];
  returnType?: string;
};

export const BUILTINS: Builtin[] = [
  { kind: 'constant', name: 'pi', detail: 'constant: number', documentation: 'π = 3.141592653589793', returnType: 'number' },
  { kind: 'constant', name: 'e', detail: 'constant: number', documentation: 'Euler’s number e ≈ 2.718281828', returnType: 'number' },
  { kind: 'constant', name: 'i', detail: 'constant: complex', documentation: 'Imaginary unit i (√-1), 1D complex', returnType: 'complex' },
  { kind: 'constant', name: 'j', detail: 'constant: complex', documentation: 'Imaginary unit j (2D imaginary)', returnType: 'complex' },
  { kind: 'constant', name: 'k', detail: 'constant: complex', documentation: 'Imaginary unit k (3D imaginary)', returnType: 'complex' },

  // Additional math built-ins
  { kind: 'function', name: 'abs', detail: 'fn(x: number|complex): number', documentation: 'Absolute value or modulus', parameters: ['x: number|complex'], returnType: 'number' },
  { kind: 'function', name: 'sqrt', detail: 'fn(x: number|complex): number|complex', documentation: 'Square root', parameters: ['x: number|complex'], returnType: 'number|complex' },
  { kind: 'function', name: 'exp', detail: 'fn(x: number|complex): number|complex', documentation: 'Exponential function e^x', parameters: ['x: number|complex'], returnType: 'number|complex' },
  { kind: 'function', name: 'min', detail: 'fn(...args: number[]): number', documentation: 'Minimum of arguments', parameters: ['...args: number[]'], returnType: 'number' },
  { kind: 'function', name: 'max', detail: 'fn(...args: number[]): number', documentation: 'Maximum of arguments', parameters: ['...args: number[]'], returnType: 'number' },
  { kind: 'function', name: 'sum', detail: 'fn(arr: number[]): number', documentation: 'Sum of array elements', parameters: ['arr: number[]'], returnType: 'number' },
  { kind: 'function', name: 'prod', detail: 'fn(arr: number[]): number', documentation: 'Product of array elements', parameters: ['arr: number[]'], returnType: 'number' },
  { kind: 'function', name: 'real', detail: 'fn(z: complex): number', documentation: 'Real part of a complex number', parameters: ['z: complex'], returnType: 'number' },
  { kind: 'function', name: 'imag', detail: 'fn(z: complex): number', documentation: 'Imaginary part of a complex number', parameters: ['z: complex'], returnType: 'number' },
  { kind: 'function', name: 'conj', detail: 'fn(z: complex): complex', documentation: 'Complex conjugate', parameters: ['z: complex'], returnType: 'complex' },
  { kind: 'function', name: 'arg', detail: 'fn(z: complex): number', documentation: 'Argument (angle) of a complex number', parameters: ['z: complex'], returnType: 'number' },
  { kind: 'function', name: 're', detail: 'fn(z: complex): number', documentation: 'Alias for real(z)', parameters: ['z: complex'], returnType: 'number' },
  { kind: 'function', name: 'im', detail: 'fn(z: complex): number', documentation: 'Alias for imag(z)', parameters: ['z: complex'], returnType: 'number' },

  { kind: 'function', name: 'sin', detail: 'fn(x: number): number', documentation: 'Sine in radians', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'cos', detail: 'fn(x: number): number', documentation: 'Cosine in radians', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'tan', detail: 'fn(x: number): number', documentation: 'Tangent in radians', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'sec', detail: 'fn(x: number): number', documentation: 'Secant in radians (1/cos)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'csc', detail: 'fn(x: number): number', documentation: 'Cosecant in radians (1/sin)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'cot', detail: 'fn(x: number): number', documentation: 'Cotangent in radians (cos/sin)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'asin', detail: 'fn(x: number): number', documentation: 'Arcsine (inverse sine, returns radians)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'acos', detail: 'fn(x: number): number', documentation: 'Arccosine (inverse cosine, returns radians)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'atan', detail: 'fn(x: number): number', documentation: 'Arctangent (inverse tangent, returns radians)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'asec', detail: 'fn(x: number): number', documentation: 'Arcsecant (inverse secant, returns radians)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'acsc', detail: 'fn(x: number): number', documentation: 'Arccosecant (inverse cosecant, returns radians)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'acot', detail: 'fn(x: number): number', documentation: 'Arccotangent (inverse cotangent, returns radians)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'sinh', detail: 'fn(x: number): number', documentation: 'Hyperbolic sine', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'cosh', detail: 'fn(x: number): number', documentation: 'Hyperbolic cosine', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'tanh', detail: 'fn(x: number): number', documentation: 'Hyperbolic tangent', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'sech', detail: 'fn(x: number): number', documentation: 'Hyperbolic secant (1/cosh)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'csch', detail: 'fn(x: number): number', documentation: 'Hyperbolic cosecant (1/sinh)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'coth', detail: 'fn(x: number): number', documentation: 'Hyperbolic cotangent (cosh/sinh)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'asinh', detail: 'fn(x: number): number', documentation: 'Inverse hyperbolic sine (arc hyperbolic sine)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'acosh', detail: 'fn(x: number): number', documentation: 'Inverse hyperbolic cosine (arc hyperbolic cosine)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'atanh', detail: 'fn(x: number): number', documentation: 'Inverse hyperbolic tangent (arc hyperbolic tangent)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'asech', detail: 'fn(x: number): number', documentation: 'Inverse hyperbolic secant (arc hyperbolic secant)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'acsch', detail: 'fn(x: number): number', documentation: 'Inverse hyperbolic cosecant (arc hyperbolic cosecant)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'acoth', detail: 'fn(x: number): number', documentation: 'Inverse hyperbolic cotangent (arc hyperbolic cotangent)', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'ln', detail: 'fn(x: number): number', documentation: 'Natural logarithm', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'log', detail: 'fn(level: string, ...args): unit', documentation: 'Log a message with a level: INFO, WARN, ERROR, DEBUG. Example: log("INFO", "message")', parameters: ['level: string', '...args'], returnType: 'unit' },
  { kind: 'function', name: 'erf', detail: 'fn(x: number): number', documentation: 'Error function', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'erfc', detail: 'fn(x: number): number', documentation: 'Complementary error function', parameters: ['x: number'], returnType: 'number' },
  { kind: 'function', name: 'print', detail: 'fn(...args): unit', documentation: 'Print values to standard output', parameters: ['...args'], returnType: 'unit' },
];

export const KEYWORDS = ['import', 'print', 'log', 'd'];