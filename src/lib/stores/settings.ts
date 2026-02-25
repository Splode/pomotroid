// Reactive settings store.
// Loaded once on startup from Rust via settings_get; updated on save.

import { writable } from 'svelte/store';
import type { Settings } from '$lib/types';

const defaults: Settings = {
  time_work_secs: 1500,
  time_short_break_secs: 300,
  time_long_break_secs: 900,
  long_break_interval: 4,
  auto_start_work: false,
  auto_start_break: false,
  min_to_tray: false,
  min_to_tray_on_close: false,
  notifications_enabled: true,
  always_on_top: false,
  break_always_on_top: false,
  volume: 1.0,
  tick_sounds_during_work: false,
  tick_sounds_during_break: true,
  shortcut_toggle: 'Control+F1',
  shortcut_reset: 'Control+F2',
  shortcut_skip: 'Control+F3',
  websocket_enabled: false,
  websocket_port: 1314,
  theme: 'Pomotroid',
  dial_countdown: false,
};

export const settings = writable<Settings>(defaults);
