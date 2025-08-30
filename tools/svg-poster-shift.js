#!/usr/bin/env node
/**
 * This script now handles THREE tasks:
 * 1. Wraps the terminal SVG in a macOS-style window frame.
 * 2. Adds a poster text overlay that fades out.
 * 3. Injects CSS to handle the initial frame and animation looping.
 */
const fs = require('fs');
const path = require('path');

// --- Utils ---
function arg(name, dflt) {
  const i = process.argv.indexOf(name);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : dflt;
}

function escapeXml(s) {
  return s.replace(/[&<>]/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;'}[c]));
}

// --- Args & Setup ---
const inPath = arg('--in', null) || arg('-i', null);
if (!inPath) {
  console.error('Usage: svg-poster-shift.js --in <file.svg>');
  process.exit(2);
}

const delayRaw = arg('--delay', process.env.POSTER_DELAY || '1.5');
const delay = Math.max(0, parseFloat(delayRaw || '1.5')) || 0;

let termSvg;
try {
  termSvg = fs.readFileSync(inPath, 'utf8');
} catch (e) {
  console.error('[script] read failed', e.message);
  process.exit(1);
}

// --- Poster Text Content ---
const base = path.basename(inPath, '.svg');
const linesMap = {
  basic: ['ftime: File Time Viewer', '- Lists files by modification time'],
  pattern: ['ftime: Filtering by Pattern', '- Focus on specific extensions or strings'],
  dir: ['ftime: Directory & Extension Scoping', '- No subcommands, just arguments'],
  tz: ['ftime: Timezone Switching', '- Change display timezone with FTL_TZ env var'],
};
const lines = linesMap[base];
if (!lines) {
  console.error('[script] no known heading for', base);
  process.exit(0);
}

// --- Find Animation Details from the Terminal SVG ---
const firstStepMatch = termSvg.match(/translateX\(-([0-9]+)px\)/);
const firstStepPx = firstStepMatch ? firstStepMatch[1] : '0';

const keyframesMatch = termSvg.match(/@keyframes\s+([a-zA-Z0-9_-]+)/);
const keyframesName = keyframesMatch ? keyframesMatch[1] : null;

const kf = termSvg.match(/@keyframes[^}]+}/s);
const pcts = kf ? (kf[0].match(/([0-9.]+)%/g) || []).map(s => parseFloat(s)) : [100];
const maxPct = Math.max(...pcts);
const estDuration = (maxPct > 0 && keyframesName) ? (100 / maxPct) * 8 : 8; // Base duration 8s

// --- Build the Final SVG ---

// Get dimensions from terminal SVG
const termWidthMatch = termSvg.match(/width="(\d+)"/);
const termHeightMatch = termSvg.match(/height="(\d+(?:\.\d+)?)"/);
const termViewBoxMatch = termSvg.match(/viewBox="([\d\s.]+)"/);

const termWidth = termWidthMatch ? parseInt(termWidthMatch[1], 10) : 580;
const termHeight = termHeightMatch ? parseFloat(termHeightMatch[1]) : 564.46;
const termViewBox = termViewBoxMatch ? termViewBoxMatch[1] : `0 0 ${termWidth} ${termHeight}`;

// Define outer frame dimensions
const framePadding = { x: 15, top: 50, bottom: 10 };
const outerWidth = termWidth + framePadding.x * 2;
const outerHeight = termHeight + framePadding.top + framePadding.bottom;

// Position the terminal SVG inside the frame
const positionedTermSvg = termSvg.replace('<svg', `<svg id="term-content" x="${framePadding.x}" y="${framePadding.top}"`);

// Create poster text
const posterTexts = lines.map((t, i) => `<text x="40" y="${80 + i * 22}" class="poster-text">${escapeXml(t)}</text>`).join('');
const posterOverlay = `\n<g class="poster-overlay">${posterTexts}</g>`;

// Define all styles
const styles = `
  <style>
    .a { fill: #282d35; }
    .poster-text { fill: #b9c0cb; font-family: Monaco, Consolas, Menlo, 'Bitstream Vera Sans Mono', 'Powerline Symbols', monospace; font-size: 16px; }
    @keyframes posterFade { to { opacity: 0; } }
    .poster-overlay { animation: posterFade 0.001s linear both; animation-delay: ${delay}s; }
    ${keyframesName ? `#term-content { animation: ${keyframesName} ${estDuration.toFixed(2)}s steps(1, end) infinite; transform: translateX(-${firstStepPx}px); }` : ''}
  </style>
`;

// Assemble the final SVG
const finalSvg = [
  `<svg xmlns="http://www.w3.org/2000/svg" width="${outerWidth}" height="${outerHeight}" viewBox="0 0 ${outerWidth} ${outerHeight}">`,
  styles,
  `<rect width="${outerWidth}" height="${outerHeight}" rx="5" ry="5" class="a"/>`,
  `<svg y="0" x="0"><circle cx="20" cy="20" r="6" fill="#ff5f58"/><circle cx="40" cy="20" r="6" fill="#ffbd2e"/><circle cx="60" cy="20" r="6" fill="#18c132"/></svg>`,
  posterOverlay,
  positionedTermSvg,
  `</svg>`
].join('');

// --- Write Output ---
try {
  fs.writeFileSync(inPath, finalSvg);
  console.log(`[script] Successfully generated ${path.basename(inPath)}`);
} catch (e) {
  console.error('[script] write failed', e.message);
  process.exit(1);
}
