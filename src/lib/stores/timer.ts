// Reactive timer state store.
// Populated by Tauri event listeners (timer:tick, timer:round-change, etc.).

import { writable } from 'svelte/store';
import type { TimerState } from '$lib/types';

const initial: TimerState = {
  round_type: 'work',
  elapsed_secs: 0,
  total_secs: 25 * 60,
  is_running: false,
  is_paused: false,
  work_round_number: 1,
  work_rounds_total: 4,
  session_work_count: 1,
};

export const timerState = writable<TimerState>(initial);
