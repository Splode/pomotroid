// Formatting helpers shared by the Better Stats components.

import type { PeriodSummary, TrendPoint } from '$lib/types';

/** Above this many hours, minutes are dropped and the value is whole hours. */
const HOURS_ONLY_THRESHOLD = 10;

/**
 * Human-readable focus duration.
 *
 * - under an hour: `45m`
 * - up to 10 hours: `1h 30m` (minutes are dropped when they are zero)
 * - 10 hours and above: whole hours only, e.g. `12h` — minute precision stops
 *   being meaningful at that scale and the longer form reads as clutter.
 */
export function fmtMins(mins: number): string {
  const total = Math.max(0, Math.round(mins));
  if (total < 60) return `${total}m`;

  const hours = total / 60;
  if (hours >= HOURS_ONLY_THRESHOLD) return `${Math.round(hours)}h`;

  const whole = Math.floor(hours);
  const rest = total % 60;
  return rest === 0 ? `${whole}h` : `${whole}h ${rest}m`;
}

/** Duration with a leading `~`, for averages and estimates. */
export function fmtMinsApprox(mins: number): string {
  return `~${fmtMins(mins)}`;
}

/** Completion rate as a percentage, or an em dash when unknown. */
export function fmtRate(rate: number | null): string {
  if (rate === null || Number.isNaN(rate)) return '—';
  return `${Math.round(rate * 100)}%`;
}

/** A one-decimal average, or an em dash when unknown. */
export function fmtAvg(value: number | null, suffix = ''): string {
  if (value === null || Number.isNaN(value)) return '—';
  const rounded = Math.round(value * 10) / 10;
  return `${Number.isInteger(rounded) ? rounded : rounded.toFixed(1)}${suffix}`;
}

/** Signed percentage change of `current` vs `previous`, or null if not comparable. */
export function deltaPct(current: number, previous: number): number | null {
  if (previous <= 0) return null;
  return (current - previous) / previous;
}

/** "+42%" / "−18%" / "—" */
export function fmtDelta(pct: number | null): string {
  if (pct === null) return '—';
  const sign = pct > 0 ? '+' : pct < 0 ? '−' : '';
  return `${sign}${Math.abs(Math.round(pct * 100))}%`;
}

/** Short weekday label from a Monday-based index (0 = Mon). */
const WEEKDAYS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
export function weekdayLabel(index: number): string {
  return WEEKDAYS[((index % 7) + 7) % 7];
}

/** Hour label such as "9a", "12p", "11p". */
export function hourLabel(hour: number): string {
  const h = ((hour % 24) + 24) % 24;
  if (h === 0) return '12a';
  if (h === 12) return '12p';
  return h < 12 ? `${h}a` : `${h - 12}p`;
}

/** "Mar 15" from a "YYYY-MM-DD" string, without timezone drift. */
const MONTHS = [
  'Jan',
  'Feb',
  'Mar',
  'Apr',
  'May',
  'Jun',
  'Jul',
  'Aug',
  'Sep',
  'Oct',
  'Nov',
  'Dec',
];
export function fmtDay(date: string): string {
  const [, m, d] = date.split('-').map((p) => parseInt(p, 10));
  if (!m || !d) return date;
  return `${MONTHS[m - 1]} ${d}`;
}

/** "Mar 11" for a week starting on that Monday. */
export function fmtWeek(date: string): string {
  return fmtDay(date);
}

/** The busiest entry in a profile list, or null when everything is zero. */
export function peakOf<T extends { rounds: number }>(entries: T[]): T | null {
  let best: T | null = null;
  for (const e of entries) {
    if (e.rounds <= 0) continue;
    if (!best || e.rounds > best.rounds) best = e;
  }
  return best;
}

/** A summary's best day is only interesting once there is activity. */
export function bestDayOf(period: PeriodSummary | null | undefined): TrendPoint | null {
  if (!period || period.rounds === 0) return null;
  return period.best_day;
}
