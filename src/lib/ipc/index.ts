// Typed wrappers around Tauri invoke() and listen().
// All backend communication goes through this module.

import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { TimerState, Settings, Theme } from '$lib/types';

// --- Timer commands ---

export const timerToggle = () => invoke<void>('timer_toggle');
export const timerReset = () => invoke<void>('timer_reset');
export const timerSkip = () => invoke<void>('timer_skip');
export const getTimerState = () => invoke<TimerState>('timer_get_state');

// --- Settings commands ---

export const getSettings = () => invoke<Settings>('settings_get');
/** Save a single setting key/value pair and receive the full updated settings. */
export const setSetting = (key: string, value: string) =>
  invoke<Settings>('settings_set', { key, value });
export const resetSettings = () => invoke<Settings>('settings_reset_defaults');
export const reloadShortcuts = () => invoke<void>('shortcuts_reload');

// --- Theme commands ---

export const getThemes = () => invoke<Theme[]>('themes_list');
export const applyThemeByName = (name: string) => invoke<Theme>('theme_apply', { name });

// --- Window commands ---

export const setWindowVisibility = (visible: boolean) =>
  invoke<void>('window_set_visibility', { visible });
export const setAlwaysOnTop = (onTop: boolean) =>
  invoke<void>('window_set_always_on_top', { onTop });

// --- Event listeners ---

export const onTimerTick = (
  cb: (payload: { elapsed_secs: number; total_secs: number }) => void,
): Promise<UnlistenFn> =>
  listen<{ elapsed_secs: number; total_secs: number }>('timer:tick', (e) => cb(e.payload));

export const onTimerPaused = (
  cb: (payload: { elapsed_secs: number }) => void,
): Promise<UnlistenFn> =>
  listen<{ elapsed_secs: number }>('timer:paused', (e) => cb(e.payload));

export const onTimerResumed = (
  cb: (payload: { elapsed_secs: number }) => void,
): Promise<UnlistenFn> =>
  listen<{ elapsed_secs: number }>('timer:resumed', (e) => cb(e.payload));

export const onRoundChange = (cb: (state: TimerState) => void): Promise<UnlistenFn> =>
  listen<TimerState>('timer:round-change', (e) => cb(e.payload));

export const onTimerReset = (cb: (state: TimerState) => void): Promise<UnlistenFn> =>
  listen<TimerState>('timer:reset', (e) => cb(e.payload));

export const onSettingsChanged = (cb: (settings: Settings) => void): Promise<UnlistenFn> =>
  listen<Settings>('settings:changed', (e) => cb(e.payload));

export const onThemesChanged = (cb: (themes: Theme[]) => void): Promise<UnlistenFn> =>
  listen<Theme[]>('themes:changed', (e) => cb(e.payload));
