/**
 * Keeps every locale file in sync with the base locale.
 *
 * `src/messages/en.json` is the source of truth for the key set. Any key that
 * exists there but is missing from another locale is added, falling back to the
 * English string so the UI degrades gracefully until a translation lands.
 *
 * Existing translations are never overwritten, and `en.json` itself is left
 * untouched apart from key ordering it already defines.
 *
 * Usage: node scripts/sync-messages.mjs
 */
import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const messagesDir = join(here, '..', 'src', 'messages');

const BASE = 'en';
const base = JSON.parse(readFileSync(join(messagesDir, `${BASE}.json`), 'utf8'));
const baseKeys = Object.keys(base);

const locales = readdirSync(messagesDir)
  .filter((f) => f.endsWith('.json'))
  .map((f) => f.replace(/\.json$/, ''));

let totalAdded = 0;

for (const locale of locales.sort()) {
  const path = join(messagesDir, `${locale}.json`);
  const raw = JSON.parse(readFileSync(path, 'utf8'));

  // Preserve the locale's existing key order, then append anything new.
  const out = {};
  for (const key of Object.keys(raw)) out[key] = raw[key];

  let added = 0;
  for (const key of baseKeys) {
    if (!(key in out)) {
      out[key] = base[key];
      added += 1;
    }
  }

  // Report keys the locale has that the base no longer defines (stale entries
  // are kept so translations are not silently destroyed, but they are surfaced).
  const stale = Object.keys(out).filter((k) => !(k in base));

  if (added > 0 || stale.length > 0) {
    writeFileSync(path, `${JSON.stringify(out, null, 2)}\n`, 'utf8');
  }

  totalAdded += added;
  const notes = [];
  if (added) notes.push(`+${added} added`);
  if (stale.length) notes.push(`${stale.length} stale: ${stale.join(', ')}`);
  console.log(`${locale}: ${notes.length ? notes.join('; ') : 'already in sync'}`);
}

console.log(`\nDone — ${totalAdded} key(s) added across ${locales.length} locale(s).`);
