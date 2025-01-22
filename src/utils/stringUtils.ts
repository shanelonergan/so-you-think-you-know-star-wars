// Levenshtein distance calculation
export function levenshteinDistance(a: string, b: string): number {
  const matrix = Array(b.length + 1).fill(null).map(() => Array(a.length + 1).fill(null));

  for (let i = 0; i <= a.length; i++) matrix[0][i] = i;
  for (let j = 0; j <= b.length; j++) matrix[j][0] = j;

  for (let j = 1; j <= b.length; j++) {
    for (let i = 1; i <= a.length; i++) {
      const substitutionCost = a[i - 1] === b[j - 1] ? 0 : 1;
      matrix[j][i] = Math.min(
        matrix[j][i - 1] + 1,
        matrix[j - 1][i] + 1,
        matrix[j - 1][i - 1] + substitutionCost
      );
    }
  }

  return matrix[b.length][a.length];
}

export function isCloseMatch(guess: string, actual: string): boolean {
  const normalizedGuess = guess.toLowerCase().trim();
  const normalizedActual = actual.toLowerCase().trim();
  
  // Exact match
  if (normalizedGuess === normalizedActual) return false;
  
  // Length difference check
  const lengthDiff = Math.abs(normalizedGuess.length - normalizedActual.length);
  if (lengthDiff > 3) return false;
  
  // Calculate distance
  const distance = levenshteinDistance(normalizedGuess, normalizedActual);
  
  // Determine threshold based on string length
  const threshold = Math.min(3, Math.floor(normalizedActual.length * 0.3));
  
  return distance <= threshold;
}