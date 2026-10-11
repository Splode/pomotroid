/** Parse minutes or M:SS, with one-second precision. */
export function parseDuration(input: string): number | null {
  const value = input.trim();
  let seconds: number;
  if (/^\d+:[0-5]?\d$/.test(value)) {
    const [minutes, remainder] = value.split(':').map(Number);
    seconds = minutes * 60 + remainder;
  } else if (/^\d+(?:\.\d+)?$/.test(value)) {
    seconds = Math.round(Number(value) * 60);
  } else {
    return null;
  }
  return Number.isSafeInteger(seconds) ? seconds : null;
}

export function formatDuration(totalSeconds: number): string {
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${String(seconds).padStart(2, '0')}`;
}
