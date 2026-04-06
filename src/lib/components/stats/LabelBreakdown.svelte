<script lang="ts">
  import type { LabelStat } from '$lib/types';

  interface Props {
    entries: LabelStat[];
    variant: 'pie' | 'list';
    onrename?: (from: string, to: string) => void;
  }

  let { entries, variant, onrename }: Props = $props();

  // Inline label editing state.
  let editingLabel = $state<string | null>(null);
  let editValue = $state('');
  let editInputEl = $state<HTMLInputElement | null>(null);

  function startEdit(label: string) {
    editingLabel = label;
    editValue = label;
    // Defer focus so the element is in the DOM.
    setTimeout(() => editInputEl?.select(), 0);
  }

  function commitEdit() {
    if (editingLabel === null) return;
    const from = editingLabel;
    editingLabel = null;
    onrename?.(from, editValue.trim().toLowerCase());
  }

  function cancelEdit() {
    editingLabel = null;
  }

  // A fixed palette of theme-aware slice colors. Unlabeled always uses the last (muted) color.
  const SLICE_COLORS = [
    'var(--color-focus-round)',
    'var(--color-short-round)',
    'var(--color-long-round)',
    'color-mix(in oklch, var(--color-focus-round) 60%, var(--color-long-round))',
    'color-mix(in oklch, var(--color-short-round) 60%, var(--color-focus-round))',
  ];
  const UNLABELED_COLOR = 'color-mix(in oklch, var(--color-foreground-darker) 35%, transparent)';

  function fmtMins(mins: number): string {
    if (mins < 60) return `${mins}m`;
    const h = Math.floor(mins / 60);
    const m = mins % 60;
    return m === 0 ? `${h}h` : `${h}h ${m}m`;
  }

  // Separate unlabeled entry from labeled entries; sort labeled by duration desc.
  const prepared = $derived.by(() => {
    const labeled = entries
      .filter((e) => e.label !== null)
      .sort((a, b) => b.duration_mins - a.duration_mins);
    const unlabeled = entries.find((e) => e.label === null) ?? null;

    // For pie: cap labeled at top 4, merge rest into "Other"
    if (variant === 'pie') {
      const top = labeled.slice(0, 4);
      const rest = labeled.slice(4);
      const other = rest.reduce((s, e) => s + e.duration_mins, 0);
      const result: Array<{ label: string; duration_mins: number; color: string }> = top.map(
        (e, i) => ({ label: e.label!, duration_mins: e.duration_mins, color: SLICE_COLORS[i % SLICE_COLORS.length] }),
      );
      if (other > 0) {
        result.push({ label: 'Other', duration_mins: other, color: SLICE_COLORS[top.length % SLICE_COLORS.length] });
      }
      if (labeled.length > 0 && unlabeled && unlabeled.duration_mins > 0) {
        result.push({ label: '(unlabeled)', duration_mins: unlabeled.duration_mins, color: UNLABELED_COLOR });
      }
      return result;
    }

    // For list: show all labeled, unlabeled last
    const result: Array<{ label: string; duration_mins: number; color: string }> = labeled.map(
      (e, i) => ({ label: e.label!, duration_mins: e.duration_mins, color: SLICE_COLORS[i % SLICE_COLORS.length] }),
    );
    if (labeled.length > 0 && unlabeled && unlabeled.duration_mins > 0) {
      result.push({ label: '(unlabeled)', duration_mins: unlabeled.duration_mins, color: UNLABELED_COLOR });
    }
    return result;
  });

  const total = $derived(prepared.reduce((s, e) => s + e.duration_mins, 0));

  // SVG pie helpers
  const CX = 70;
  const CY = 70;
  const R = 54;
  const R_INNER = 34;

  function polarToXY(cx: number, cy: number, r: number, angle: number) {
    return { x: cx + r * Math.cos(angle - Math.PI / 2), y: cy + r * Math.sin(angle - Math.PI / 2) };
  }

  function slicePath(cx: number, cy: number, outerR: number, innerR: number, start: number, end: number): string {
    const fmt = (n: number) => n.toFixed(3);
    const sweep = end - start;

    // Full circle: start and end are the same point — SVG arc collapses to nothing.
    // Draw as two half-arcs (outer clockwise, inner counterclockwise for the donut hole).
    if (sweep >= 2 * Math.PI - 0.0001) {
      const mid = start + Math.PI;
      const o0 = polarToXY(cx, cy, outerR, start);
      const oM = polarToXY(cx, cy, outerR, mid);
      const i0 = polarToXY(cx, cy, innerR, start);
      const iM = polarToXY(cx, cy, innerR, mid);
      return [
        `M ${fmt(o0.x)} ${fmt(o0.y)}`,
        `A ${outerR} ${outerR} 0 1 1 ${fmt(oM.x)} ${fmt(oM.y)}`,
        `A ${outerR} ${outerR} 0 1 1 ${fmt(o0.x)} ${fmt(o0.y)}`,
        `M ${fmt(i0.x)} ${fmt(i0.y)}`,
        `A ${innerR} ${innerR} 0 1 0 ${fmt(iM.x)} ${fmt(iM.y)}`,
        `A ${innerR} ${innerR} 0 1 0 ${fmt(i0.x)} ${fmt(i0.y)}`,
        'Z',
      ].join(' ');
    }

    const os = polarToXY(cx, cy, outerR, start);
    const oe = polarToXY(cx, cy, outerR, end);
    const ie = polarToXY(cx, cy, innerR, end);
    const is_ = polarToXY(cx, cy, innerR, start);
    const large = sweep > Math.PI ? 1 : 0;
    return [
      `M ${fmt(os.x)} ${fmt(os.y)}`,
      `A ${outerR} ${outerR} 0 ${large} 1 ${fmt(oe.x)} ${fmt(oe.y)}`,
      `L ${fmt(ie.x)} ${fmt(ie.y)}`,
      `A ${innerR} ${innerR} 0 ${large} 0 ${fmt(is_.x)} ${fmt(is_.y)}`,
      'Z',
    ].join(' ');
  }

  // Compute pie slices from prepared entries
  const slices = $derived.by(() => {
    if (total === 0) return [];
    let angle = 0;
    return prepared.map((entry) => {
      const sweep = (entry.duration_mins / total) * 2 * Math.PI;
      const path = slicePath(CX, CY, R, R_INNER, angle, angle + sweep);
      const mid = angle + sweep / 2;
      angle += sweep;
      return { ...entry, path, mid };
    });
  });

  const maxDuration = $derived(prepared.length > 0 ? prepared[0].duration_mins : 1);
</script>

{#if prepared.length > 0}
  <div class="breakdown">
    <span class="section-title">TIME BY ACTIVITY</span>

    {#if variant === 'pie'}
      <div class="pie-area">
        <svg width={CX * 2} height={CY * 2} viewBox="0 0 {CX * 2} {CY * 2}" class="pie-svg">
          {#each slices as slice}
            <path d={slice.path} fill={slice.color} class="slice" />
          {/each}
        </svg>

        <div class="legend">
          {#each prepared as entry, i}
            <div class="legend-row">
              <span class="legend-dot" style="background: {entry.color}"></span>
              <span class="legend-label">{entry.label}</span>
              <span class="legend-dur">{fmtMins(entry.duration_mins)}</span>
            </div>
          {/each}
        </div>
      </div>

    {:else}
      <div class="list">
        {#each prepared as entry}
          {@const isUnlabeled = entry.label === '(unlabeled)'}
          {@const isEditing = editingLabel === entry.label}
          <div class="list-row">
            <div class="list-header">
              {#if isEditing}
                <input
                  bind:this={editInputEl}
                  class="list-label-edit"
                  type="text"
                  maxlength="48"
                  bind:value={editValue}
                  onkeydown={(e) => {
                    if (e.key === 'Enter') { e.preventDefault(); commitEdit(); }
                    if (e.key === 'Escape') cancelEdit();
                  }}
                  onblur={commitEdit}
                />
              {:else if !isUnlabeled && onrename}
                <button
                  class="list-label list-label-editable"
                  onclick={() => startEdit(entry.label)}
                >{entry.label}</button>
              {:else}
                <span class="list-label">{entry.label}</span>
              {/if}
              <span class="list-dur">{fmtMins(entry.duration_mins)}</span>
            </div>
            <div class="list-bar-track">
              <div
                class="list-bar-fill"
                style="width: {(entry.duration_mins / maxDuration) * 100}%; background: {entry.color}"
              ></div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .breakdown {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 16px 24px 20px;
    border-top: 1px solid var(--color-separator);
    animation: app-fade-in 0.2s ease;
  }

  .section-title {
    font-size: 0.68rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    color: var(--color-foreground-darker);
  }

  /* ── Pie ──────────────────────────────────────────────────── */
  .pie-area {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .pie-svg {
    flex-shrink: 0;
  }

  .slice {
    opacity: 0.9;
    transition: opacity 0.15s;
  }

  .slice:hover { opacity: 1; }

  .legend {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .legend-row {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.7rem;
  }

  .legend-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .legend-label {
    flex: 1;
    color: var(--color-foreground);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 120px;
  }

  .legend-dur {
    color: var(--color-foreground-darker);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }

  /* ── List ─────────────────────────────────────────────────── */
  .list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .list-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .list-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    font-size: 0.72rem;
  }

  .list-label {
    color: var(--color-foreground);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 70%;
  }

  .list-dur {
    color: var(--color-foreground-darker);
    font-variant-numeric: tabular-nums;
    font-size: 0.68rem;
  }

  button.list-label-editable {
    background: none;
    border: none;
    padding: 0;
    font: inherit;
    text-align: left;
    cursor: pointer;
    border-radius: 2px;
    transition: color 0.12s;
    max-width: 70%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  button.list-label-editable:hover {
    color: var(--color-focus-round);
    text-decoration: underline;
    text-underline-offset: 2px;
  }

  .list-label-edit {
    background: none;
    border: none;
    border-bottom: 1px solid var(--color-focus-round);
    outline: none;
    color: var(--color-foreground);
    font-size: 0.72rem;
    font-family: inherit;
    padding: 0 2px;
    width: 70%;
    caret-color: var(--color-focus-round);
  }

  .list-bar-track {
    height: 4px;
    background: color-mix(in oklch, var(--color-foreground) 8%, transparent);
    border-radius: 2px;
    overflow: hidden;
  }

  .list-bar-fill {
    height: 100%;
    border-radius: 2px;
    opacity: 0.85;
    transition: width 0.35s cubic-bezier(0.22, 1, 0.36, 1);
  }
</style>
