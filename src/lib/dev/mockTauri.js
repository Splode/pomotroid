// @ts-nocheck
/**
 * DEV-ONLY Tauri IPC mock.
 *
 * The real app talks to a Rust backend over Tauri's IPC bridge. When the
 * frontend is served by plain `npm run dev` (a normal browser), that bridge
 * does not exist, so every `invoke()` rejects and each window stays blank.
 *
 * This module installs a lightweight stand-in bridge so the UI can be reviewed
 * in a browser: settings and themes come back with defaults, the stats
 * commands return realistic generated history, and the timer runs for real
 * (ticks, pauses, skips) so the incremental ladder can be seen climbing.
 *
 * It is injected ONLY by the dev server (see `tauriDevMockPlugin` in
 * vite.config.js) and is never part of a production bundle. It is also a no-op
 * whenever a genuine Tauri bridge is present.
 */

const isTauri = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
const isMac = typeof navigator !== 'undefined' && /Mac/i.test(navigator.userAgent || '');

/** Present only in the `tauri dev` shell; harmless elsewhere. */
export const MOCK_ACTIVE = !isTauri();

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

const settings = {
  time_work_secs: 300, // 5 min base — makes the increment easy to watch
  time_short_break_secs: 300,
  time_long_break_secs: 900,
  incremental_work_enabled: true,
  time_work_increment_secs: 300, // +5 min per completed work round
  time_work_max_secs: 1200, // capped at 20 min
  incremental_reset_on_long_break: true,
  incremental_reset_daily: true,
  long_break_interval: 4,
  short_breaks_enabled: true,
  long_breaks_enabled: true,
  auto_start_work: false,
  auto_start_break: false,
  tray_icon_enabled: false,
  min_to_tray: false,
  min_to_tray_on_close: false,
  notifications_enabled: false,
  always_on_top: false,
  break_always_on_top: false,
  volume: 1.0,
  tick_sounds_during_work: false,
  tick_sounds_during_break: false,
  shortcut_toggle: 'Control+F1',
  shortcut_reset: 'Control+F2',
  shortcut_skip: 'Control+F3',
  shortcut_restart: 'Control+F4',
  websocket_enabled: false,
  websocket_port: 1314,
  theme_mode: 'auto',
  theme_light: 'Pomotroid Light',
  theme_dark: 'Pomotroid',
  dial_countdown: true,
  language: 'en',
  verbose_logging: false,
  check_for_updates: false,
  global_shortcuts_enabled: false,
  local_shortcut_toggle: ' ',
  local_shortcut_reset: 'ArrowLeft',
  local_shortcut_skip: 'ArrowRight',
  local_shortcut_volume_down: 'ArrowDown',
  local_shortcut_volume_up: 'ArrowUp',
  local_shortcut_mute: 'm',
  local_shortcut_fullscreen: 'F11',
};

const themes = [
  {
    name: 'Pomotroid',
    is_custom: false,
    colors: {
      '--color-background': '#2f384b',
      '--color-foreground': '#f2f4f8',
      '--color-foreground-darker': '#96a0b5',
      '--color-separator': 'rgba(242, 244, 248, 0.12)',
      '--color-hover': 'rgba(242, 244, 248, 0.08)',
      '--color-focus-round': '#e9573f',
      '--color-short-round': '#4fbf8b',
      '--color-long-round': '#5b8def',
    },
  },
  {
    name: 'Pomotroid Light',
    is_custom: false,
    colors: {
      '--color-background': '#f6f6f8',
      '--color-foreground': '#1f2733',
      '--color-foreground-darker': '#7b8798',
      '--color-separator': 'rgba(31, 39, 51, 0.12)',
      '--color-hover': 'rgba(31, 39, 51, 0.06)',
      '--color-focus-round': '#d9452c',
      '--color-short-round': '#2f9e6b',
      '--color-long-round': '#3f6fd0',
    },
  },
];

// ---------------------------------------------------------------------------
// Deterministic mock history
// ---------------------------------------------------------------------------

/** Local "YYYY-MM-DD" for a Date. */
function ymd(d) {
  const p = (n) => String(n).padStart(2, '0');
  return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}`;
}

/** Days between two "YYYY-MM-DD" strings. */
function dayDiff(a, b) {
  const [ay, am, ad] = a.split('-').map(Number);
  const [by, bm, bd] = b.split('-').map(Number);
  const da = Date.UTC(ay, am - 1, ad);
  const db = Date.UTC(by, bm - 1, bd);
  return Math.round((db - da) / 86400000);
}

function addDays(dateStr, n) {
  const [y, m, d] = dateStr.split('-').map(Number);
  const dt = new Date(Date.UTC(y, m - 1, d));
  dt.setUTCDate(dt.getUTCDate() + n);
  return `${dt.getUTCFullYear()}-${String(dt.getUTCMonth() + 1).padStart(2, '0')}-${String(
    dt.getUTCDate()
  ).padStart(2, '0')}`;
}

function weekdayIndex(dateStr) {
  const [y, m, d] = dateStr.split('-').map(Number);
  const dow = new Date(Date.UTC(y, m - 1, d)).getUTCDay(); // 0 = Sunday
  return (dow + 6) % 7; // 0 = Monday
}

/** Stable pseudo-random in [0,1) derived from a date string. */
function hash01(str) {
  let h = 2166136261;
  for (let i = 0; i < str.length; i += 1) {
    h ^= str.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  return ((h >>> 0) % 10000) / 10000;
}

/** Plausible rounds for a given day: weekends are quieter, some days empty. */
function roundsForDate(date, today) {
  const age = dayDiff(date, today);
  const wd = weekdayIndex(date);
  const r = hash01(date);
  if (wd >= 5) {
    if (r < 0.55) return 0;
    return 1 + Math.floor(r * 3);
  }
  if (r < 0.12) return 0;
  const base = 3 + Math.floor(r * 7);
  // Slight upward drift as the history gets more recent.
  return Math.max(1, base + (age < 7 ? 1 : 0));
}

/** Mean of the trailing `window` values at each index (partial windows ramp up). */
function movingAverage(points, window) {
  return points.map((_, i) => {
    const start = Math.max(0, i + 1 - window);
    const slice = points.slice(start, i + 1);
    return slice.reduce((s, p) => s + p.rounds, 0) / slice.length;
  });
}

/** Histogram of round lengths, mirroring the Rust bucketing. */
const BUCKET_BOUNDS = [
  [0, 15],
  [15, 25],
  [25, 40],
  [40, 60],
  [60, null],
];

/** Build a full Insights payload mirroring the Rust `stats_get_insights` shape. */
function buildInsights() {
  const today = ymd(new Date());
  const dayMap = new Map();

  // ~200 days of history so the 120-day heatmap and 12-week rollup fill up.
  for (let age = 200; age >= 0; age -= 1) {
    const date = addDays(today, -age);
    const rounds = roundsForDate(date, today);
    if (rounds === 0) continue;
    dayMap.set(date, { rounds, focus_mins: rounds * (20 + Math.floor(hash01(date + 'f') * 12)) });
  }
  // Guarantee some activity today.
  dayMap.set(today, { rounds: 3, focus_mins: 75 });

  const perDay = [...dayMap.entries()]
    .map(([date, v]) => ({ date, ...v }))
    .sort((a, b) => (a.date < b.date ? -1 : 1));

  const todayByHour = Array(24).fill(0);
  todayByHour[9] = 1;
  todayByHour[14] = 1;
  todayByHour[16] = 1;

  const inWindow = (date, start, end) => date >= start && date <= end;

  const summarise = (label, start, days) => {
    const rows = perDay.filter((p) => inWindow(p.date, start, today));
    const rounds = rows.reduce((s, p) => s + p.rounds, 0);
    const focus_mins = rows.reduce((s, p) => s + p.focus_mins, 0);
    const active_days = rows.length;
    const best = rows.reduce((b, p) => (!b || p.rounds > b.rounds ? p : b), null);
    // Started rounds are approximated as completed plus a small abandonment rate.
    const started = Math.round(rounds * 1.14);
    return {
      label,
      days,
      rounds,
      focus_mins,
      active_days,
      avg_rounds_per_active_day: active_days ? rounds / active_days : null,
      avg_focus_mins_per_active_day: active_days ? focus_mins / active_days : null,
      completion_rate: started ? rounds / started : null,
      best_day: best ? { ...best } : null,
    };
  };

  const fillDays = (count) => {
    const out = [];
    for (let age = count - 1; age >= 0; age -= 1) {
      const date = addDays(today, -age);
      const v = dayMap.get(date);
      out.push({ date, rounds: v?.rounds ?? 0, focus_mins: v?.focus_mins ?? 0 });
    }
    return out;
  };

  const trend = fillDays(28);
  const heatmap = fillDays(120);

  const weeks = [];
  const currentMonday = addDays(today, -weekdayIndex(today));
  for (let w = 11; w >= 0; w -= 1) {
    const start = addDays(currentMonday, -w * 7);
    const end = addDays(start, 6);
    const rows = perDay.filter((p) => inWindow(p.date, start, end));
    weeks.push({
      date: start,
      rounds: rows.reduce((s, p) => s + p.rounds, 0),
      focus_mins: rows.reduce((s, p) => s + p.focus_mins, 0),
    });
  }

  // --- Rhythm ---------------------------------------------------------------
  const hour_profile = Array.from({ length: 24 }, (_, hour) => ({ hour, rounds: 0, focus_mins: 0 }));
  const grid = Array.from({ length: 7 * 24 }, (_, i) => ({
    weekday: Math.floor(i / 24),
    hour: i % 24,
    rounds: 0,
    focus_mins: 0,
  }));

  perDay.forEach((p, i) => {
    const wd = weekdayIndex(p.date);
    // Two focus blocks per active day: a morning one and an afternoon one.
    const morning = 8 + (i % 3);
    const afternoon = 13 + (i % 5);
    const morningRounds = Math.ceil(p.rounds / 2);
    const afternoonRounds = p.rounds - morningRounds;

    for (const [hour, rounds] of [
      [morning, morningRounds],
      [afternoon, afternoonRounds],
    ]) {
      if (rounds <= 0) continue;
      const mins = Math.round((p.focus_mins * rounds) / p.rounds);
      hour_profile[hour].rounds += rounds;
      hour_profile[hour].focus_mins += mins;
      const cell = grid[wd * 24 + hour];
      cell.rounds += rounds;
      cell.focus_mins += mins;
    }
  });

  const weekday_profile = Array.from({ length: 7 }, (_, weekday) => ({
    weekday,
    rounds: 0,
    focus_mins: 0,
  }));
  perDay.forEach((p) => {
    const idx = weekdayIndex(p.date);
    weekday_profile[idx].rounds += p.rounds;
    weekday_profile[idx].focus_mins += p.focus_mins;
  });

  const activeHours = hour_profile.filter((h) => h.rounds > 0).map((h) => h.hour);

  // --- Streak ---------------------------------------------------------------
  let current = 0;
  for (let age = 0; age < 400; age += 1) {
    if (!dayMap.has(addDays(today, -age))) {
      if (age === 0) continue; // today may simply not be logged yet
      break;
    }
    current += 1;
  }
  let longest = 1;
  let run = 1;
  for (let i = 1; i < perDay.length; i += 1) {
    if (dayDiff(perDay[i - 1].date, perDay[i].date) === 1) {
      run += 1;
      longest = Math.max(longest, run);
    } else {
      run = 1;
    }
  }

  // --- Consistency (trailing 30 days) --------------------------------------
  const last30 = fillDays(30);
  const active30 = last30.filter((p) => p.rounds > 0);
  const total30 = last30.reduce((s, p) => s + p.rounds, 0);
  const mean30 = total30 / last30.length;
  const variance = last30.reduce((s, p) => s + (p.rounds - mean30) ** 2, 0) / last30.length;
  const best30 = Math.max(0, ...last30.map((p) => p.rounds));
  let bestStreak30 = 0;
  let run30 = 0;
  for (const p of last30) {
    run30 = p.rounds > 0 ? run30 + 1 : 0;
    bestStreak30 = Math.max(bestStreak30, run30);
  }

  // --- Records --------------------------------------------------------------
  const total_rounds = perDay.reduce((s, p) => s + p.rounds, 0);
  const total_focus_mins = perDay.reduce((s, p) => s + p.focus_mins, 0);
  const bestDayRounds = perDay.reduce((b, p) => (!b || p.rounds > b.rounds ? p : b), null);
  const bestDayFocus = perDay.reduce((b, p) => (!b || p.focus_mins > b.focus_mins ? p : b), null);
  const bestWeek = weeks.reduce((b, p) => (!b || p.focus_mins > b.focus_mins ? p : b), null);

  // --- Session-length histogram --------------------------------------------
  const session_buckets = BUCKET_BOUNDS.map(([min, max]) => ({
    min_mins: min,
    max_mins: max,
    rounds: 0,
    focus_mins: 0,
  }));
  perDay.forEach((p, i) => {
    // Spread each day's rounds across a plausible mix of round lengths.
    const lengths = [12, 18, 24, 30, 45, 62];
    for (let r = 0; r < p.rounds; r += 1) {
      const mins = 5 + lengths[(i + r) % lengths.length];
      const bucket = session_buckets.find((b) =>
        b.max_mins === null ? mins >= b.min_mins : mins >= b.min_mins && mins < b.max_mins
      );
      if (bucket) {
        bucket.rounds += 1;
        bucket.focus_mins += mins;
      }
    }
  });

  // --- Momentum -------------------------------------------------------------
  const week = summarise('week', addDays(today, -6), 7);
  const month = summarise('month', addDays(today, -27), 28);
  const last_7_days = summarise('last7', addDays(today, -6), 7);
  const last_90_days = summarise('last90', addDays(today, -89), 90);
  const previous_7_days = summarise('prev7', addDays(today, -13), 7);

  const todayRounds = dayMap.get(today)?.rounds ?? 0;

  const pctChange = (cur, prev) => (prev > 0 ? (cur - prev) / prev : null);

  const comparisons = [
    {
      label: 'rounds_week',
      current: last_7_days.rounds,
      previous: previous_7_days.rounds,
      delta_pct: pctChange(last_7_days.rounds, previous_7_days.rounds),
    },
    {
      label: 'focus_week',
      current: last_7_days.focus_mins,
      previous: previous_7_days.focus_mins,
      delta_pct: pctChange(last_7_days.focus_mins, previous_7_days.focus_mins),
    },
    {
      label: 'active_days_week',
      current: last_7_days.active_days,
      previous: previous_7_days.active_days,
      delta_pct: pctChange(last_7_days.active_days, previous_7_days.active_days),
    },
    {
      label: 'avg_per_active_day',
      current: last_7_days.avg_rounds_per_active_day ?? 0,
      previous: previous_7_days.avg_rounds_per_active_day ?? 0,
      delta_pct: pctChange(
        last_7_days.avg_rounds_per_active_day ?? 0,
        previous_7_days.avg_rounds_per_active_day ?? 0
      ),
    },
  ];

  const topHourMins = Math.max(...hour_profile.map((h) => h.focus_mins));
  const topHourTotal = hour_profile.reduce((s, h) => s + h.focus_mins, 0);
  const topWeekdayMins = Math.max(...weekday_profile.map((w) => w.focus_mins));
  const topWeekdayTotal = weekday_profile.reduce((s, w) => s + w.focus_mins, 0);

  return {
    today,
    today_rounds: todayRounds,
    today_focus_mins: dayMap.get(today)?.focus_mins ?? 0,
    today_by_hour: todayByHour,
    rounds_to_beat_average:
      week.avg_rounds_per_active_day === null
        ? null
        : Math.round(week.avg_rounds_per_active_day) - todayRounds,
    ahead_of_average: todayRounds > (week.avg_rounds_per_active_day ?? 0),

    week,
    month,
    last_7_days,
    last_90_days,
    previous_7_days,
    comparisons,

    trend,
    heatmap,
    daily_all: perDay.map((p) => ({ ...p })),
    weeks,
    moving_avg_7: movingAverage(trend, 7),
    focus_trend: trend.map((p) => ({ ...p })),
    daily_28: trend,

    hour_profile,
    weekday_profile,
    rhythm_grid: grid,
    active_from_hour: activeHours.length ? Math.min(...activeHours) : null,
    active_to_hour: activeHours.length ? Math.max(...activeHours) : null,

    session_buckets,
    top_weekday_share: topWeekdayTotal ? topWeekdayMins / topWeekdayTotal : null,
    top_hour_share: topHourTotal ? topHourMins / topHourTotal : null,

    consistency: {
      window_days: 30,
      active_days: active30.length,
      active_ratio: active30.length / 30,
      avg_rounds_per_day: mean30,
      avg_focus_mins_per_day:
        last30.reduce((s, p) => s + p.focus_mins, 0) / last30.length,
      stddev_rounds: Math.sqrt(variance),
      strong_days: last30.filter((p) => p.rounds >= Math.ceil(best30 * 0.75) && best30 > 0).length,
      light_days: last30.filter((p) => p.rounds > 0 && p.rounds <= Math.floor(best30 * 0.25))
        .length,
      rest_days: last30.filter((p) => p.rounds === 0).length,
      best_streak_in_window: bestStreak30,
    },

    records: {
      total_rounds,
      total_focus_mins,
      tracked_days: perDay.length,
      avg_session_mins: total_rounds ? total_focus_mins / total_rounds : null,
      longest_session_mins: Math.max(...session_buckets.map((b) => b.max_mins ?? b.min_mins)),
      avg_rounds_per_active_day: perDay.length ? total_rounds / perDay.length : null,
      best_day_rounds: bestDayRounds,
      best_day_focus: bestDayFocus,
      best_week: bestWeek,
    },

    streak: {
      current,
      longest,
      last_active_date: perDay.length ? perDay[perDay.length - 1].date : null,
      at_risk: current > 0 && todayRounds === 0,
    },
  };
}

// ---------------------------------------------------------------------------
// Mock timer engine
// ---------------------------------------------------------------------------

const MOCK_ROUNDS_TOTAL = settings.long_break_interval;

/** Local calendar date as "YYYY-MM-DD" (mirrors SQLite's date('now','localtime')). */
function todayKey() {
  const d = new Date();
  const pad = (n) => String(n).padStart(2, '0');
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

const timer = {
  round_type: 'work',
  previous_round_type: '',
  elapsed_secs: 0,
  is_running: false,
  work_round_number: 1,
  session_work_count: 1,
  work_rounds_completed: 0,
  ladder_day: todayKey(),
  _interval: null,
};

/**
 * Mirror of `SequenceState::check_day_rollover`: a new calendar day restarts the
 * ladder unless the daily reset trigger is turned off.
 */
function checkDayRollover() {
  const today = todayKey();
  if (timer.ladder_day === today) return false;
  const hadDay = timer.ladder_day !== null;
  timer.ladder_day = today;
  if (!hadDay || !settings.incremental_reset_daily) return false;
  const changed = timer.work_rounds_completed !== 0;
  timer.work_rounds_completed = 0;
  return changed;
}

/**
 * Mirror of the Rust `SequenceState::work_duration_secs` ladder so the mock
 * shows the same escalating durations the real backend would compute.
 */
function currentDuration() {
  if (timer.round_type === 'short-break') return settings.time_short_break_secs;
  if (timer.round_type === 'long-break') return settings.time_long_break_secs;
  const base = settings.time_work_secs;
  if (!settings.incremental_work_enabled || settings.time_work_increment_secs === 0) return base;
  const cap = Math.max(settings.time_work_max_secs, base);
  return Math.min(base + settings.time_work_increment_secs * timer.work_rounds_completed, cap);
}

function snapshot() {
  const base = settings.time_work_secs;
  const inc = settings.time_work_increment_secs;
  const cap = Math.max(settings.time_work_max_secs, base);
  const active = settings.incremental_work_enabled && inc > 0;
  return {
    round_type: timer.round_type,
    previous_round_type: timer.previous_round_type,
    elapsed_secs: timer.elapsed_secs,
    total_secs: currentDuration(),
    is_running: timer.is_running,
    is_paused: !timer.is_running && timer.elapsed_secs > 0,
    work_round_number: timer.work_round_number,
    work_rounds_total: MOCK_ROUNDS_TOTAL,
    session_work_count: timer.session_work_count,
    incremental_work_enabled: settings.incremental_work_enabled,
    base_work_secs: base,
    work_increment_secs: inc,
    work_max_secs: settings.time_work_max_secs,
    increment_steps: active ? timer.work_rounds_completed : 0,
    at_increment_cap: active && base + inc * timer.work_rounds_completed >= cap,
  };
}

function stopTicking() {
  if (timer._interval !== null) {
    clearInterval(timer._interval);
    timer._interval = null;
  }
}

function startTicking() {
  stopTicking();
  timer.is_running = true;
  emit('timer:started', { total_secs: currentDuration() });
  timer._interval = setInterval(() => {
    timer.elapsed_secs += 1;
    const total = currentDuration();
    emit('timer:tick', { elapsed_secs: timer.elapsed_secs, total_secs: total });
    if (timer.elapsed_secs >= total) advanceRound();
  }, 1000);
}

function advanceRound() {
  stopTicking();
  const rolledOver = checkDayRollover();
  const finishedWork = timer.round_type === 'work';
  timer.previous_round_type = timer.round_type;

  if (timer.round_type === 'work') {
    timer.work_rounds_completed += 1;
    if (timer.work_round_number >= MOCK_ROUNDS_TOTAL && settings.long_breaks_enabled) {
      timer.round_type = 'long-break';
      // Long-break reset trigger: the ladder restarts with the new cycle unless
      // the user keeps it climbing across breaks.
      if (settings.incremental_reset_on_long_break) timer.work_rounds_completed = 0;
    } else {
      if (timer.work_round_number >= MOCK_ROUNDS_TOTAL) timer.work_round_number = 1;
      else timer.work_round_number += 1;
      timer.round_type = 'short-break';
    }
  } else if (timer.round_type === 'long-break') {
    timer.round_type = 'work';
    timer.work_round_number = 1;
    timer.session_work_count += 1;
  } else {
    timer.round_type = 'work';
    timer.session_work_count += 1;
  }

  // A work round that finished just after midnight belongs to the day it
  // started in, so it must not extend the new day's ladder.
  if (rolledOver && finishedWork) timer.work_rounds_completed = 0;

  timer.elapsed_secs = 0;
  timer.is_running = false;
  emit('timer:round-change', snapshot());
}

// --- Event bus -------------------------------------------------------------

let nextEventId = 1;
const eventListeners = new Map();

function emit(name, payload) {
  for (const cb of eventListeners.get(name) ?? []) {
    try {
      cb(payload);
    } catch {
      /* a listener throwing must not break the others */
    }
  }
}

// ---------------------------------------------------------------------------
// Command table
// ---------------------------------------------------------------------------

const commands = {
  settings_get: () => ({ ...settings }),
  settings_set: ({ key, value }) => {
    if (key in settings) {
      const current = settings[key];
      if (typeof current === 'boolean') settings[key] = value === 'true';
      else if (typeof current === 'number') settings[key] = Number(value);
      else settings[key] = value;
    }
    emit('settings:changed', { ...settings });
    checkDayRollover();
    emit('timer:reset', snapshot());
    return { ...settings };
  },
  settings_reset_defaults: () => ({ ...settings }),
  shortcuts_reload: () => undefined,

  themes_list: () => themes.map((t) => ({ ...t })),

  timer_get_state: () => {
    checkDayRollover();
    return snapshot();
  },
  timer_toggle: () => {
    checkDayRollover();
    if (timer.is_running) {
      stopTicking();
      timer.is_running = false;
      emit('timer:paused', { elapsed_secs: timer.elapsed_secs });
    } else if (timer.elapsed_secs > 0) {
      timer.is_running = true;
      emit('timer:resumed', { elapsed_secs: timer.elapsed_secs });
      startTicking();
    } else {
      startTicking();
    }
  },
  timer_reset: () => {
    stopTicking();
    timer.round_type = 'work';
    timer.previous_round_type = '';
    timer.elapsed_secs = 0;
    timer.is_running = false;
    timer.work_round_number = 1;
    timer.work_rounds_completed = 0;
    timer.ladder_day = todayKey();
    timer.session_work_count = 1;
    emit('timer:reset', snapshot());
  },
  timer_reset_increment: () => {
    timer.work_rounds_completed = 0;
    timer.ladder_day = todayKey();
    emit('timer:reset', snapshot());
  },
  timer_restart_round: () => {
    stopTicking();
    timer.elapsed_secs = 0;
    timer.is_running = false;
    emit('timer:reset', snapshot());
  },
  timer_skip: () => advanceRound(),

  stats_get_insights: () => buildInsights(),
  stats_get_detailed: () => {
    const i = buildInsights();
    return {
      today: {
        rounds: i.today_rounds,
        focus_mins: i.today_focus_mins,
        completion_rate: i.week.completion_rate,
        by_hour: i.today_by_hour,
      },
      week: i.trend.slice(-7).map((p) => ({ date: p.date, rounds: p.rounds })),
      streak: { current: i.streak.current, longest: i.streak.longest },
    };
  },
  stats_get_heatmap: () => {
    const i = buildInsights();
    return {
      entries: i.heatmap.map((p) => ({ date: p.date, count: p.rounds })),
      total_rounds: i.records.total_rounds,
      total_hours: Math.round(i.records.total_focus_mins / 60),
      longest_streak: i.streak.longest,
    };
  },
  sessions_clear: () => {
    emit('sessions:cleared', undefined);
    return undefined;
  },

  audio_get_custom_info: () => ({ work_alert: null, short_break_alert: null, long_break_alert: null }),
  notification_show: () => undefined,
  window_set_visibility: () => undefined,
  open_log_dir: () => undefined,
  get_log_dir: () => '/tmp/pomotroid-logs',
  app_version: () => '1.7.2-dev+mock',
  accessibility_trusted: () => true,
  tray_supported: () => false,
  check_update: () => null,

  // --- Log plugin (used by @tauri-apps/plugin-log) ---
  'plugin:log|log': () => undefined,

  // --- Event plugin bridge (used by @tauri-apps/api/event) ---
  'plugin:event|listen': ({ event, handler }) => {
    const id = nextEventId++;
    const callbacks = eventListeners.get(event) ?? [];
    callbacks.push(window.__TAURI_INTERNALS__.callbacks[handler]);
    eventListeners.set(event, callbacks);
    return id;
  },
  'plugin:event|unlisten': ({ event, eventId }) => {
    const callbacks = eventListeners.get(event) ?? [];
    if (callbacks.length) callbacks.pop();
    void eventId;
    return undefined;
  },
  'plugin:event|emit': ({ event, payload }) => {
    emit(event, payload);
    return undefined;
  },

  // --- Window plugin (used by the settings/stats windows) ---
  'plugin:window|show': () => undefined,
  'plugin:window|hide': () => undefined,
  'plugin:window|close': () => undefined,
  'plugin:window|set_focus': () => undefined,
  'plugin:window|minimize': () => undefined,
  'plugin:window|toggle_maximize': () => undefined,
  'plugin:window|is_maximized': () => false,
  'plugin:window|start_dragging': () => undefined,
  'plugin:window|start_resize_dragging': () => undefined,
  'plugin:window|set_fullscreen': () => undefined,
  'plugin:window|is_fullscreen': () => false,
  'plugin:window|internal_toggle_maximize': () => undefined,
  'plugin:window|theme': () => 'dark',
  'plugin:webview|create_webview_window': (args) => {
    const label = args?.options?.label ?? args?.label ?? 'window';
    const url = args?.options?.url ?? args?.url ?? '/';
    emit('mock:navigate', { label, url });
    return undefined;
  },
  'plugin:webview|get_all_webviews': () => [],
};

// ---------------------------------------------------------------------------
// Bridge installation
// ---------------------------------------------------------------------------

let installed = false;

export function installMockTauri() {
  if (installed || isTauri() || typeof window === 'undefined') return;
  // Never ship the mock: production builds skip installation entirely.
  if (!import.meta.env.DEV) return;

  let callbackId = 0;
  const callbacks = {};

  window.__TAURI_INTERNALS__ = {
    callbacks,
    transformCallback(cb, once = false) {
      const id = callbackId++;
      callbacks[id] = (payload) => {
        if (once) delete callbacks[id];
        if (typeof cb === 'function') cb(payload);
      };
      return id;
    },
    unregisterCallback(id) {
      delete callbacks[id];
    },
    convertFileSrc(filePath) {
      return filePath;
    },
    metadata: {
      currentWindow: { label: 'main' },
      currentWebview: { label: 'main', windowLabel: 'main' },
    },
    plugins: {},
    async invoke(cmd, args = {}) {
      const handler = commands[cmd];
      if (!handler) {
        // Unknown command: resolve quietly so optional features never break the UI.
        console.warn(`[mock-tauri] unhandled command: ${cmd}`, args);
        return undefined;
      }
      return handler(args);
    },
  };

  // jsdom-free window metadata for `getCurrentWebviewWindow()`.
  window.__TAURI_INTERNALS__.metadata.currentWindow.label = 'main';
  window.__TAURI_INTERNALS__.metadata.currentWebview.label = 'main';

  installed = true;

  const reason = isMac ? 'macOS' : 'browser';
  console.info(
    `%c[mock-tauri]%c Running with a mocked Tauri bridge (${reason}). ` +
      'UI only — no real timer persistence or notifications.',
    'background:#e9573f;color:#fff;padding:1px 5px;border-radius:3px',
    'color:inherit'
  );
}

installMockTauri();
