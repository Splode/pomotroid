<script lang="ts">
  import { openUrl } from '@tauri-apps/plugin-opener';

  const VERSION = '1.0.0';
  const REPO = 'https://github.com/Splode/pomotroid';
  const RELEASE_URL = `${REPO}/releases/tag/v${VERSION}`;
</script>

<div class="panel">
  <p class="heading">About</p>

  <section class="content">
    <!-- Pomotroid logo SVG -->
    <svg
      version="1.2"
      baseProfile="tiny"
      width="72"
      height="72"
      viewBox="0 0 256 256"
      xmlns="http://www.w3.org/2000/svg"
      aria-label="Pomotroid logo"
    >
      <circle fill="var(--color-background)" cx="128" cy="128" r="126.81"/>
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

    <h2>Pomotroid</h2>
    <p class="meta">Version {VERSION}</p>
    <button class="link" onclick={() => openUrl(RELEASE_URL)}>
      Release notes
    </button>
    <button class="link" onclick={() => openUrl(REPO)}>
      License &amp; Documentation
    </button>
  </section>
</div>

<style>
  .panel {
    padding: 8px 12px;
    height: 100%;
  }

  .heading {
    font-size: 0.7rem;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--color-foreground-darker, var(--color-foreground));
    margin-bottom: 8px;
    padding-bottom: 6px;
    border-bottom: 1px solid rgba(255,255,255,0.06);
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 24px;
    gap: 8px;
  }

  h2 {
    color: var(--color-short-round);
    font-weight: 400;
    letter-spacing: 0.05em;
    font-size: 1.1rem;
  }

  .meta {
    font-size: 0.8rem;
    color: var(--color-foreground-darker, var(--color-foreground));
    letter-spacing: 0.05em;
  }

  .link {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-foreground-darker, var(--color-foreground));
    font-size: 0.8rem;
    letter-spacing: 0.04em;
    padding: 2px 0;
    transition: color 0.15s;
  }

  .link:hover {
    color: var(--color-accent);
  }
</style>
