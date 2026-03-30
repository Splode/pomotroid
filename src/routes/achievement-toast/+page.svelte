<script lang="ts">
  import '../../app.css';
  import { onMount } from 'svelte';
  import AchievementBadge from '$lib/components/AchievementBadge.svelte';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { LogicalSize } from '@tauri-apps/api/dpi';

  // All display data comes from URL params — no IPC calls needed.
  const params  = new URLSearchParams(window.location.search);
  const count   = parseInt(params.get('count') ?? '1', 10);
  const name    = params.get('name') ?? 'Achievement';
  const emoji   = params.get('emoji') ?? '';
  const color   = params.get('color') ?? '#888888';
  const bg      = params.get('bg')    ?? '#0e0e12';
  const fg      = params.get('fg')    ?? '#ddd0bc';
  const isRollup = count > 1;

  let visible = $state(false);
  let exiting = $state(false);

  // --- Auto-close ---
  function scheduleClose() {
    setTimeout(() => {
      exiting = true;
      setTimeout(() => getCurrentWebviewWindow().close(), 300);
    }, 7000);
  }

  onMount(async () => {
    const win = getCurrentWebviewWindow();

    // Debug: log actual window dimensions before and after resize
    const pre = await win.innerSize();
    console.error(`[toast] pre-resize inner=${pre.width}x${pre.height} dpr=${window.devicePixelRatio}`);

    try {
      await win.setSize(new LogicalSize(320, 66));
      const post = await win.innerSize();
      console.error(`[toast] post-resize inner=${post.width}x${post.height}`);
    } catch (e) {
      console.error(`[toast] setSize failed: ${e}`);
    }

    await win.show();

    requestAnimationFrame(() => {
      requestAnimationFrame(() => { visible = true; });
    });

    scheduleClose();
  });

  const SPARKLE_ANGLES = [0, 60, 120, 180, 240, 300];
</script>

<div
  class="toast"
  class:visible
  class:exiting
  style="--achievement-color: {color}; --bg: {bg}; --fg: {fg};"
>
  <!-- Badge -->
  <div class="badge-area" class:pop={visible}>
    <div class="sparkles" aria-hidden="true">
      {#each SPARKLE_ANGLES as angle}
        <span class="sparkle" style="--angle: {angle}deg" class:pop={visible}></span>
      {/each}
    </div>
    {#if emoji && !isRollup}
      <AchievementBadge {color} {emoji} earned={true} size={30} />
    {:else}
      <div class="fallback-badge">🏆</div>
    {/if}
  </div>

  <!-- Text -->
  <div class="text">
    <span class="label">Achievement Unlocked</span>
    <span class="name">{name}</span>
  </div>
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    width: 320px;
    height: 66px;
    max-height: 66px;
    overflow: hidden;
  }

  .toast {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 14px;
    background: var(--bg);
    opacity: 0;
    transition: opacity 200ms ease-out;
  }

  .toast.visible {
    opacity: 1;
  }

  .toast.exiting {
    opacity: 0;
    transition: opacity 300ms ease-in;
  }

  /* Badge */
  .badge-area {
    position: relative;
    flex-shrink: 0;
    transform: scale(0);
    transition: transform 400ms cubic-bezier(0.34, 1.56, 0.64, 1);
    transition-delay: 80ms;
  }

  .badge-area.pop {
    transform: scale(1);
  }

  .fallback-badge {
    width: 44px;
    height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 26px;
  }

  /* Text */
  .text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .label {
    font-size: 0.6rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--achievement-color);
  }

  .name {
    font-size: 0.85rem;
    font-weight: 700;
    color: var(--fg);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Sparkles */
  .sparkles {
    position: absolute;
    left: 50%;
    top: 50%;
    width: 0;
    height: 0;
    pointer-events: none;
  }

  .sparkle {
    position: absolute;
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: var(--achievement-color);
    opacity: 0;
    transform: rotate(var(--angle)) translateY(0) scale(0);
  }

  .sparkle.pop {
    animation: sparkle-burst 600ms ease-out 120ms forwards;
  }

  @keyframes sparkle-burst {
    0%   { opacity: 1; transform: rotate(var(--angle)) translateY(0) scale(1); }
    60%  { opacity: 0.8; }
    100% { opacity: 0; transform: rotate(var(--angle)) translateY(-28px) scale(0.3); }
  }
</style>
