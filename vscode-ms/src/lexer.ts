export type TokenType = 'keyword' | 'string' | 'number' | 'operator' | 'function' | 'variable' | 'comment';

export interface Token {
  type: TokenType;
  start: number;
  end: number;
}

const keywords = new Set<string>([
  'import','print','log','d',
  'sin','cos','tan','sec','csc','cot',
  'asin','acos','atan','asec','acsc','acot',
  'sinh','cosh','tanh','sech','csch','coth',
  'asinh','acosh','atanh','asech','acsch','acoth',
  'erf','erfc','ln','log','pi','e','i','j','k'
]);

export function tokenize(input: string): Token[] {
  const tokens: Token[] = [];
  let i = 0;
  const n = input.length;

  while (i < n) {
    const ch = input[i];
    const start = i;

    // comments
    if (ch === '#') {
      const lineEnd = input.indexOf('\n', i);
      const end = lineEnd === -1 ? n : lineEnd;
      tokens.push({ type: 'comment', start, end });
      i = end;
      continue;
    }

    // whitespace
    if (/\s/.test(ch)) { i++; continue; }

    // strings "..." and docstrings """..."""
    if (ch === '"') {
      if (input.substr(i, 3) === '"""') {
        i += 3;
        while (i < n && input.substr(i, 3) !== '"""') { i++; }
        i = Math.min(n, i + 3);
        tokens.push({ type: 'string', start, end: i });
        continue;
      } else {
        i++;
        while (i < n && input[i] !== '"') {
          if (input[i] === '\\' && i + 1 < n) i += 2; else i++;
        }
        i = Math.min(n, i + 1);
        tokens.push({ type: 'string', start, end: i });
        continue;
      }
    }

    // numbers
    if (/[0-9]/.test(ch) || (ch === '.' && /[0-9]/.test(input[i+1] || ''))) {
      i++;
      while (i < n && /[0-9_]/.test(input[i])) i++;
      if (i < n && input[i] === '.') {
        i++;
        while (i < n && /[0-9_]/.test(input[i])) i++;
      }
      tokens.push({ type: 'number', start, end: i });
      continue;
    }

    // identifiers / functions / keywords
    if (/[A-Za-z_]/.test(ch)) {
      i++;
      while (i < n && /[A-Za-z0-9_]/.test(input[i])) i++;
      const word = input.slice(start, i);
      const type: TokenType = keywords.has(word) ? 'keyword' : (/[A-Za-z_]/.test(word[0]) ? 'variable' : 'function');
      tokens.push({ type, start, end: i });
      continue;
    }

    // operators and punctuation
    const two = input.substr(i, 2);
    if (["==","!=",">=","<=","->"].includes(two)) {
      i += 2;
      tokens.push({ type: 'operator', start, end: i });
      continue;
    }

    if (/[+\-*/^=()\[\]{},!<>|:]/.test(ch)) {
      i++;
      tokens.push({ type: 'operator', start, end: i });
      continue;
    }

    // fallback
    i++;
  }

  return tokens;
}
