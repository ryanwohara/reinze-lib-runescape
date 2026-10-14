// Refreshes TAX_EXEMPT_IDS in src/tax.rs from the OSRS wiki's list of items
// exempt from the Grand Exchange convenience fee.
//
// https://oldschool.runescape.wiki/w/Grand_Exchange#Exempt_from_tax
//
//   node scripts/gen-tax-exempt.js
//
// The list is wiki links, not item IDs, so each linked page's {{Infobox Item}}
// supplies the IDs. A link whose text names one version - "Games necklace(8)"
// - takes only that version; a bare "Energy potion" takes every dose. IDs the
// Grand Exchange cannot trade (noted, untradeable) are dropped by checking
// them against the prices mapping, which lists only tradeable items.
//
// Fails loudly rather than writing a bad list.

const fs = require('fs');
const path = require('path');

const API = 'https://oldschool.runescape.wiki/api.php';
const MAPPING = 'https://prices.runescape.wiki/api/v1/osrs/mapping';
const UA = { headers: { 'User-Agent': 'Reinze.com' } };
const TARGET = path.join(__dirname, '..', 'src', 'tax.rs');

async function json(url) {
  const res = await fetch(url, UA);
  if (!res.ok) throw new Error(`${url}: HTTP ${res.status}`);
  return res.json();
}

async function wikitext(titles) {
  const url = `${API}?action=query&prop=revisions&rvprop=content&rvslots=main` +
    `&redirects=1&format=json&formatversion=2` +
    `&titles=${titles.map(encodeURIComponent).join('|')}`;
  const body = await json(url);

  // Map each requested title through any normalisation and redirect, so the
  // caller can look pages up by the title it asked for.
  const alias = new Map();
  for (const step of [...(body.query.normalized || []), ...(body.query.redirects || [])]) {
    alias.set(step.from, step.to);
  }
  const resolve = (t) => { while (alias.has(t)) t = alias.get(t); return t; };

  const pages = new Map();
  for (const page of body.query.pages) {
    if (page.missing) continue;
    pages.set(page.title, page.revisions[0].slots.main.content);
  }

  return new Map(titles.map((t) => [t, pages.get(resolve(t))]));
}

// The bullet links under "===Exempt from tax===", up to the next heading.
function exemptLinks(text) {
  const start = text.indexOf('===Exempt from tax===');
  if (start < 0) throw new Error('Grand Exchange page has no "Exempt from tax" section');

  const rest = text.slice(start + 3);
  const end = rest.search(/\n==/);
  const section = end < 0 ? rest : rest.slice(0, end);

  const links = [...section.matchAll(/^\*+\s*\[\[([^\]|]+)(?:\|([^\]]+))?\]\]/gm)]
    .map(([, target, label]) => ({ target: target.trim(), label: (label || target).trim() }));

  if (links.length < 20) {
    throw new Error(`only ${links.length} exempt links found - the section changed shape`);
  }
  return links;
}

// [{ name, ids }] for each version in a page's {{Infobox Item}}.
function infoboxVersions(text) {
  const start = text.indexOf('{{Infobox Item');
  if (start < 0) return [];
  const box = text.slice(start, text.indexOf('\n}}', start));

  const versions = new Map();
  for (const [, key, n, value] of box.matchAll(/^\|\s*(name|id)(\d*)\s*=\s*(.*)$/gm)) {
    const v = versions.get(n) || { name: null, ids: [] };
    if (key === 'name') v.name = value.trim();
    else v.ids = (value.match(/\d+/g) || []).map(Number);
    versions.set(n, v);
  }

  // An unnumbered field is the default for every numbered version lacking it.
  const base = versions.get('') || { name: null, ids: [] };
  versions.delete('');
  if (versions.size === 0) return [base];

  return [...versions.values()].map((v) => ({
    name: v.name || base.name,
    ids: v.ids.length ? v.ids : base.ids,
  }));
}

(async () => {
  const ge = await wikitext(['Grand Exchange']);
  const links = exemptLinks(ge.get('Grand Exchange'));

  const mapping = await json(MAPPING);
  const tradeable = new Map(mapping.map((item) => [item.id, item.name]));

  const pages = new Map();
  for (let i = 0; i < links.length; i += 50) {
    const batch = await wikitext(links.slice(i, i + 50).map((l) => l.target));
    for (const [title, text] of batch) pages.set(title, text);
  }

  const exempt = new Map();

  for (const { target, label } of links) {
    const text = pages.get(target);
    if (!text) throw new Error(`${target}: page not found`);

    const versions = infoboxVersions(text);
    const named = versions.filter((v) => v.name === label);
    const chosen = named.length ? named : versions;

    const ids = chosen.flatMap((v) => v.ids).filter((id) => tradeable.has(id));
    if (ids.length === 0) {
      throw new Error(`${label} (${target}): no tradeable item ID in its infobox`);
    }

    for (const id of ids) exempt.set(id, tradeable.get(id));
  }

  const ids = [...exempt.keys()].sort((a, b) => a - b);
  const body = ids.map((id) => `    ${id}, // ${exempt.get(id)}`).join('\n');
  const declaration = `pub const TAX_EXEMPT_IDS: &[u32] = &[\n${body}\n];`;

  const source = fs.readFileSync(TARGET, 'utf8');

  // Check the declaration is there exactly once, so "already correct" cannot
  // be confused with "matched nothing".
  const pattern = /pub const TAX_EXEMPT_IDS: &\[u32\] = &\[[^\]]*\];/g;
  const hits = source.match(pattern) || [];
  if (hits.length !== 1) {
    throw new Error(
      `TAX_EXEMPT_IDS: expected exactly 1 declaration in ${TARGET}, found ${hits.length}. ` +
      `It was renamed or reformatted - fix this script before trusting it.`);
  }

  const updated = source.replace(pattern, declaration);

  if (updated === source) {
    console.log(`no change: ${ids.length} exempt items`);
    return;
  }

  fs.writeFileSync(TARGET, updated);
  console.log(`updated: ${ids.length} exempt items from ${links.length} links`);
})().catch((e) => {
  console.error(e.message);
  process.exit(1);
});
