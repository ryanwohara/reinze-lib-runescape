// Refreshes the casket value constants in src/pickpocket.rs from the OSRS
// wiki. A casket's worth comes from its reward table rather than the Grand
// Exchange, so the wiki evaluates it and we bake in the result.
//
//   node scripts/gen-clue-values.js
//
// Fails loudly rather than writing a bad value.

const fs = require('fs');
const path = require('path');

const API = 'https://oldschool.runescape.wiki/api.php';
const UA = { headers: { 'User-Agent': 'Reinze.com' } };
const TARGET = path.join(__dirname, '..', 'src', 'pickpocket.rs');

async function expand(template) {
  const url = `${API}?action=parse&text=${encodeURIComponent(template)}` +
    `&contentmodel=wikitext&prop=text&format=json`;
  const res = await fetch(url, UA);
  if (!res.ok) throw new Error(`${template}: HTTP ${res.status}`);

  const json = await res.json();
  const text = (json.parse?.text?.['*'] || '').replace(/<[^>]+>/g, '').trim();
  const value = parseFloat(text.replace(/,/g, ''));

  if (!isFinite(value) || value <= 0) {
    throw new Error(`${template} did not evaluate to a number: ${text.slice(0, 80)}`);
  }
  return Math.round(value);
}

(async () => {
  const easy = await expand('{{EasyClueValue}}');
  const master = await expand('{{MasterClueValue}}');

  // A casket is worth more than a coin and less than a bank. If either lands
  // outside that, the template changed shape and a human should look.
  for (const [name, value] of [['easy', easy], ['master', master]]) {
    if (value < 100 || value > 50_000_000) {
      throw new Error(`${name} casket value looks wrong: ${value}`);
    }
  }

  const source = fs.readFileSync(TARGET, 'utf8');
  const updated = source
    .replace(/pub const EASY_CASKET_GP: f64 = [\d_.]+;/,
             `pub const EASY_CASKET_GP: f64 = ${easy.toLocaleString('en-US').replace(/,/g, '_')}.0;`)
    .replace(/pub const MASTER_CASKET_GP: f64 = [\d_.]+;/,
             `pub const MASTER_CASKET_GP: f64 = ${master.toLocaleString('en-US').replace(/,/g, '_')}.0;`);

  if (updated === source) {
    console.log(`no change: easy ${easy}, master ${master}`);
    return;
  }

  fs.writeFileSync(TARGET, updated);
  console.log(`updated: easy ${easy}, master ${master}`);
})();
