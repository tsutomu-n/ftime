#!/usr/bin/env node
// Scale the timing in an asciinema v2 cast file.
// Usage: scale-cast.js --in in.cast --out out.cast --factor 2

const fs = require('fs');

const args = process.argv.slice(2);
function arg(name, dflt) {
  const i = args.indexOf(name);
  return i >= 0 && i + 1 < args.length ? args[i + 1] : dflt;
}

const inPath = arg('--in');
const outPath = arg('--out');
const factorRaw = arg('--factor', process.env.SLOW || '2');
const factor = Math.max(0.01, parseFloat(factorRaw || '2'));
const prerollRaw = arg('--preroll', process.env.PREROLL || '0');
const preroll = Math.max(0, parseFloat(prerollRaw || '0'));

if (!inPath || !outPath) {
  console.error('Usage: scale-cast.js --in in.cast --out out.cast [--factor 2]');
  process.exit(2);
}

const text = fs.readFileSync(inPath, 'utf8');
const lines = text.split(/\r?\n/);
const out = [];

// header (v2 JSON)
if (lines.length > 0 && lines[0].trim().length) {
  try {
    const header = JSON.parse(lines[0]);
    if (header && typeof header === 'object') {
      if (typeof header.duration === 'number' && isFinite(header.duration)) {
        header.duration = +(header.duration * factor + preroll).toFixed(6);
      }
      out.push(JSON.stringify(header));
    } else {
      out.push(lines[0]);
    }
  } catch {
    out.push(lines[0]);
  }
}

for (let i = 1; i < lines.length; i++) {
  const L = lines[i];
  if (!L) continue;
  try {
    const ev = JSON.parse(L);
    if (Array.isArray(ev) && typeof ev[0] === 'number') {
      ev[0] = + (ev[0] * factor + preroll).toFixed(6);
    }
    out.push(JSON.stringify(ev));
  } catch {
    // keep line as-is if parsing fails
    out.push(L);
  }
}

fs.mkdirSync(require('path').dirname(outPath), { recursive: true });
fs.writeFileSync(outPath, out.join('\n') + '\n');
console.error(`[scale-cast] factor=${factor} preroll=${preroll} in=${inPath} out=${outPath}`);
