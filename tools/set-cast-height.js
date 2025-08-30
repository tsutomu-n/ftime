#!/usr/bin/env node
const fs = require('fs');
const readline = require('readline');

function arg(name, dflt) {
  const i = process.argv.indexOf(name);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : dflt;
}

const inPath = arg('--in');
const outPath = arg('--out');
const rows = parseInt(arg('--rows', '40'), 10);

if (!inPath || !outPath) {
  console.error('Usage: set-cast-height.js --in <file.cast> --out <file.cast> --rows <num>');
  process.exit(1);
}

const fileStream = fs.createReadStream(inPath);
const rl = readline.createInterface({ input: fileStream, crlfDelay: Infinity });
const writer = fs.createWriteStream(outPath);

let isFirstLine = true;

rl.on('line', (line) => {
  if (isFirstLine) {
    try {
      const header = JSON.parse(line);
      header.height = rows;
      writer.write(JSON.stringify(header) + '\n');
    } catch (e) {
      // Not a valid JSON header, write as is
      writer.write(line + '\n');
    }
    isFirstLine = false;
  } else {
    writer.write(line + '\n');
  }
});

rl.on('close', () => {
  writer.end();
  console.error(`[set-height] Wrote cast with height=${rows} to ${outPath}`);
});
