// Utility for managing the full-screen break screen overlay window ('break').

import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { info, error as logError } from '@tauri-apps/plugin-log';

/**
 * Opens or restores the full-screen break overlay window.
 * If the window already exists, it is focused and brought to the front.
 */
export async function openBreakShield(): Promise<void> {
  try {
    const existing = await WebviewWindow.getByLabel('break');
    if (existing) {
      await existing.show();
      await existing.setFocus();
      return;
    }

    new WebviewWindow('break', {
      url: '/break',
      title: 'Pomotroid — Break',
      fullscreen: true,
      alwaysOnTop: true,
      decorations: false,
      skipTaskbar: true,
      focus: true,
      visible: false, // will show after applying theme to avoid white flash
    });
    await info('[break-shield] created break window');
  } catch (err) {
    await logError(`[break-shield] failed to open break shield: ${err}`);
  }
}

/**
 * Closes the full-screen break overlay window if it is currently open.
 */
export async function closeBreakShield(): Promise<void> {
  try {
    const existing = await WebviewWindow.getByLabel('break');
    if (existing) {
      await existing.close();
      await info('[break-shield] closed break window');
    }
  } catch (err) {
    await logError(`[break-shield] failed to close break shield: ${err}`);
  }
}
