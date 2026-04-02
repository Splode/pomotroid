// Local keyboard shortcuts — active while any Pomotroid window has focus.
// Bindings are plain KeyboardEvent.key strings (e.g. " ", "ArrowLeft", "F11").
// The handler is created via a factory so it can read reactive state by closure.

import { timerToggle, timerRestartRound, timerSkip, setSetting } from '$lib/ipc';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import type { Settings } from '$lib/types';

const MODIFIER_KEYS = new Set(['Control', 'Shift', 'Alt', 'Meta', 'CapsLock']);

/** Readable display labels for special KeyboardEvent.key values. */
export function formatLocalKey(key: string): string {
  switch (key) {
    case ' ':         return 'Space';
    case 'ArrowLeft': return '←';
    case 'ArrowRight': return '→';
    case 'ArrowUp':   return '↑';
    case 'ArrowDown': return '↓';
    default:          return key;
  }
}

export interface LocalShortcutState {
  /** Current settings (read reactively via closure). */
  getSettings: () => Settings;
  /** Current volume (may differ from settings.volume if mute is pending). */
  getVolume: () => number;
  setVolume: (v: number) => void;
  /** Pre-mute volume saved for mute-toggle restore. */
  getPreMuteVolume: () => number;
  setPreMuteVolume: (v: number) => void;
  /** Current fullscreen state. */
  getFullscreen: () => boolean;
  setFullscreen: (v: boolean) => void;
}

/**
 * Returns a keydown event handler that fires local shortcuts.
 * Mount via addEventListener('keydown', handler) in onMount, remove in onDestroy.
 */
export function createLocalShortcutHandler(state: LocalShortcutState): (e: KeyboardEvent) => void {
  return function handleKeydown(e: KeyboardEvent) {
    // Skip if a text input / shortcut capture field is focused.
    const target = e.target as HTMLElement | null;
    if (target) {
      const tag = target.tagName;
      if (tag === 'INPUT' || tag === 'TEXTAREA' || target.isContentEditable) return;
    }

    // Skip bare modifier keys.
    if (MODIFIER_KEYS.has(e.key)) return;

    const s = state.getSettings();
    const key = e.key;

    if (key === s.local_shortcut_toggle) {
      e.preventDefault();
      timerToggle();
    } else if (key === s.local_shortcut_reset) {
      e.preventDefault();
      timerRestartRound();
    } else if (key === s.local_shortcut_skip) {
      e.preventDefault();
      timerSkip();
    } else if (key === s.local_shortcut_volume_down) {
      e.preventDefault();
      const newVol = Math.max(0, Math.round((state.getVolume() - 0.05) * 100) / 100);
      state.setVolume(newVol);
      setSetting('volume', String(Math.round(newVol * 100)));
    } else if (key === s.local_shortcut_volume_up) {
      e.preventDefault();
      const newVol = Math.min(1, Math.round((state.getVolume() + 0.05) * 100) / 100);
      state.setVolume(newVol);
      setSetting('volume', String(Math.round(newVol * 100)));
    } else if (key === s.local_shortcut_mute) {
      e.preventDefault();
      const cur = state.getVolume();
      if (cur > 0) {
        state.setPreMuteVolume(cur);
        state.setVolume(0);
        setSetting('volume', '0');
      } else {
        const restore = state.getPreMuteVolume() > 0 ? state.getPreMuteVolume() : 0.5;
        state.setVolume(restore);
        setSetting('volume', String(Math.round(restore * 100)));
      }
    } else if (key === s.local_shortcut_fullscreen) {
      e.preventDefault();
      const next = !state.getFullscreen();
      state.setFullscreen(next);
      getCurrentWebviewWindow().setFullscreen(next);
    }
  };
}
