// Generates a placeholder 1024x1024 app icon (PNG) with no external dependencies.
// Usage: node scripts/generate-icon.mjs [outputPath]
import { deflateSync } from "node:zlib";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const SIZE = 1024;
const outputPath = resolve(process.argv[2] ?? "assets/icon-source.png");

const CRC_TABLE = (() => {
  const table = new Uint32Array(256);
  for (let n = 0; n < 256; n += 1) {
    let c = n;
    for (let k = 0; k < 8; k += 1) {
      c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    }
    table[n] = c >>> 0;
  }
  return table;
})();

function crc32(buffer) {
  let c = 0xffffffff;
  for (const byte of buffer) {
    c = CRC_TABLE[(c ^ byte) & 0xff] ^ (c >>> 8);
  }
  return (c ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const length = Buffer.alloc(4);
  length.writeUInt32BE(data.length, 0);
  const typeBuffer = Buffer.from(type, "ascii");
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(Buffer.concat([typeBuffer, data])), 0);
  return Buffer.concat([length, typeBuffer, data, crc]);
}

// --- Canvas -----------------------------------------------------------------
const pixels = Buffer.alloc(SIZE * SIZE * 4);

function setPixel(x, y, [r, g, b, a = 255]) {
  if (x < 0 || y < 0 || x >= SIZE || y >= SIZE) return;
  const offset = (y * SIZE + x) * 4;
  pixels[offset] = r;
  pixels[offset + 1] = g;
  pixels[offset + 2] = b;
  pixels[offset + 3] = a;
}

function fillRect(x0, y0, w, h, color) {
  for (let y = y0; y < y0 + h; y += 1) {
    for (let x = x0; x < x0 + w; x += 1) {
      setPixel(x, y, color);
    }
  }
}

const INDIGO = [79, 70, 229, 255];
const INDIGO_DARK = [67, 56, 202, 255];
const WHITE = [255, 255, 255, 255];
const GREY = [203, 213, 225, 255];

// Background.
fillRect(0, 0, SIZE, SIZE, INDIGO);
// Subtle bottom band for depth.
fillRect(0, SIZE * 0.8, SIZE, SIZE * 0.2, INDIGO_DARK);

// Document shape.
const docX = 288;
const docY = 224;
const docW = 448;
const docH = 576;
fillRect(docX, docY, docW, docH, WHITE);

// Folded corner.
const fold = 128;
for (let y = 0; y < fold; y += 1) {
  const rowWidth = fold - y;
  fillRect(docX + docW - rowWidth, docY + y, rowWidth, 1, GREY);
}

// Text lines.
const lineX = docX + 72;
const lineW = docW - 144;
for (let i = 0; i < 5; i += 1) {
  const y = docY + 200 + i * 64;
  fillRect(lineX, y, i === 4 ? lineW * 0.55 : lineW, 24, GREY);
}

// --- Encode PNG -------------------------------------------------------------
const raw = Buffer.alloc(SIZE * (SIZE * 4 + 1));
for (let y = 0; y < SIZE; y += 1) {
  raw[y * (SIZE * 4 + 1)] = 0; // filter: none
  pixels.copy(raw, y * (SIZE * 4 + 1) + 1, y * SIZE * 4, (y + 1) * SIZE * 4);
}

const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(SIZE, 0);
ihdr.writeUInt32BE(SIZE, 4);
ihdr[8] = 8; // bit depth
ihdr[9] = 6; // color type RGBA
ihdr[10] = 0; // compression
ihdr[11] = 0; // filter
ihdr[12] = 0; // interlace

const png = Buffer.concat([
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
  chunk("IHDR", ihdr),
  chunk("IDAT", deflateSync(raw, { level: 9 })),
  chunk("IEND", Buffer.alloc(0)),
]);

mkdirSync(dirname(outputPath), { recursive: true });
writeFileSync(outputPath, png);
console.log(`Wrote ${outputPath} (${png.length} bytes)`);
