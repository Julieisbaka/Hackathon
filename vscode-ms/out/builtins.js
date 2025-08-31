"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.KEYWORDS = exports.BUILTINS = void 0;
exports.BUILTINS = [
    { kind: 'constant', name: 'pi', detail: 'constant: number', documentation: 'π = 3.141592653589793', returnType: 'number' },
    { kind: 'constant', name: 'e', detail: 'constant: number', documentation: 'Euler’s number e ≈ 2.718281828', returnType: 'number' },
    { kind: 'constant', name: 'i', detail: 'constant: complex', documentation: 'Imaginary unit i (√-1)', returnType: 'complex' },
    { kind: 'constant', name: 'j', detail: 'constant: complex', documentation: 'Imaginary unit j (alias of i)', returnType: 'complex' },
    { kind: 'constant', name: 'k', detail: 'constant: complex', documentation: 'Imaginary unit k (alias of i)', returnType: 'complex' },
    { kind: 'function', name: 'sin', detail: 'fn(x: number): number', documentation: 'Sine in radians', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'cos', detail: 'fn(x: number): number', documentation: 'Cosine in radians', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'tan', detail: 'fn(x: number): number', documentation: 'Tangent in radians', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'asin', detail: 'fn(x: number): number', documentation: 'Arcsine (returns radians)', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'acos', detail: 'fn(x: number): number', documentation: 'Arccosine (returns radians)', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'atan', detail: 'fn(x: number): number', documentation: 'Arctangent (returns radians)', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'sinh', detail: 'fn(x: number): number', documentation: 'Hyperbolic sine', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'cosh', detail: 'fn(x: number): number', documentation: 'Hyperbolic cosine', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'tanh', detail: 'fn(x: number): number', documentation: 'Hyperbolic tangent', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'ln', detail: 'fn(x: number): number', documentation: 'Natural logarithm', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'log', detail: 'fn(x: number): number', documentation: 'Base-10 logarithm', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'erf', detail: 'fn(x: number): number', documentation: 'Error function', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'erfc', detail: 'fn(x: number): number', documentation: 'Complementary error function', parameters: ['x: number'], returnType: 'number' },
    { kind: 'function', name: 'print', detail: 'fn(...args): unit', documentation: 'Print values to standard output', parameters: ['...args'], returnType: 'unit' },
];
exports.KEYWORDS = ['import', 'print', 'log', 'd'];
//# sourceMappingURL=builtins.js.map