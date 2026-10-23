// Reactive timer state store.
// Populated by Tauri event listeners (timer:tick, timer:round-change, etc.).

import { writable } from 'svelte/store';
import type { TimerState } from '$lib/types';

const initial: TimerState = {
  round_type: 'work',
  previous_round_type: '',
  elapsed_secs: 0,
  total_secs: 25 * 60,
  is_running: false,
  is_paused: false,
  work_round_number: 1,
  work_rounds_total: 4,
  session_work_count: 1,
  incremental_work_enabled: false,
  base_work_secs: 25 * 60,
  work_increment_secs: 5 * 60,
  work_max_secs: 90 * 60,
  increment_steps: 0,
  at_increment_cap: false,
};

export const timerState = writable<TimerState>(initial);
