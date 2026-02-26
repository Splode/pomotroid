<script lang="ts">
  import { openUrl } from '@tauri-apps/plugin-opener';

  const VERSION = '1.0.0';
  const REPO = 'https://github.com/Splode/pomotroid';
  const RELEASE_URL = `${REPO}/releases/tag/v${VERSION}`;
</script>

<div class="section">
  <div class="hero">
    <svg
      version="1.2"
      baseProfile="tiny"
      width="64"
      height="64"
      viewBox="0 0 256 256"
      xmlns="http://www.w3.org/2000/svg"
      aria-label="Pomotroid logo"
    >
      <circle fill="var(--color-background-light)" cx="128" cy="128" r="126.81"/>
      <circle
        fill="none"
        stroke="var(--color-focus-round)"
        stroke-width="40"
        stroke-linecap="round"
        cx="128"
        cy="128"
        r="73.31"
      />
    </svg>
    <div>
      <h2 class="name">Pomotroid</h2>
      <p class="version">Version {VERSION}</p>
    </div>
  </div>

  <div class="links">
    <button class="link-row" onclick={() => openUrl(RELEASE_URL)}>
      <span>Release Notes</span>
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M2 10L10 2M10 2H4M10 2V8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <button class="link-row" onclick={() => openUrl(REPO)}>
      <span>Source Code &amp; License</span>
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M2 10L10 2M10 2H4M10 2V8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
  </div>

  <p class="credit">
    Built with Tauri, Svelte, and Rust.<br/>
    MIT License — Copyright &copy; 2017–2026 Christopher Murphy.
  </p>
</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
    padding: 32px 20px;
    gap: 24px;
  }

  .hero {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .name {
    font-size: 1.1rem;
    font-weight: 400;
    color: var(--color-short-round);
    letter-spacing: 0.04em;
    margin-bottom: 4px;
  }

  .version {
    font-size: 0.8rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    letter-spacing: 0.04em;
  }

  .links {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--color-separator);
    border-radius: 6px;
    overflow: hidden;
  }

  .link-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background: none;
    border: none;
    border-bottom: 1px solid var(--color-separator);
    cursor: pointer;
    color: var(--color-foreground);
    font-size: 0.85rem;
    letter-spacing: 0.02em;
    text-align: left;
    transition: background 0.12s;
  }

  .link-row:last-child {
    border-bottom: none;
  }

  .link-row:hover {
    background: var(--color-hover);
    color: var(--color-accent);
  }

  .credit {
    font-size: 0.72rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    opacity: 0.5;
    line-height: 1.6;
  }
</style>
