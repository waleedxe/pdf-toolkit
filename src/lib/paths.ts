/**
 * Cross-platform path helpers.
 *
 * The app runs on Windows (the primary target) but is developed and tested on
 * Linux, so these helpers must not assume a single separator. They only do
 * string manipulation — no filesystem access — so they are safe to call from
 * the UI thread.
 */

const SEPARATORS = /[\\/]/;

/** The path separator that matches the given path (defaults to `/`). */
export function separatorFor(path: string): "/" | "\\" {
  return path.includes("\\") ? "\\" : "/";
}

/** The final component of a path, e.g. `C:\a\b.pdf` → `b.pdf`. */
export function baseName(path: string): string {
  const normalized = path.replace(/[\\/]+$/, "");
  const index = Math.max(normalized.lastIndexOf("/"), normalized.lastIndexOf("\\"));
  return index === -1 ? normalized : normalized.slice(index + 1);
}

/** Everything before the final component, e.g. `C:\a\b.pdf` → `C:\a`. */
export function dirName(path: string): string {
  const normalized = path.replace(/[\\/]+$/, "");
  const index = Math.max(normalized.lastIndexOf("/"), normalized.lastIndexOf("\\"));
  return index <= 0 ? "" : normalized.slice(0, index);
}

/** The extension including the dot, lower-cased, e.g. `B.PDF` → `.pdf`. */
export function extension(path: string): string {
  const name = baseName(path);
  const dot = name.lastIndexOf(".");
  if (dot <= 0) return "";
  return name.slice(dot).toLowerCase();
}

/** The file name without its extension, e.g. `report.final.pdf` → `report.final`. */
export function fileNameWithoutExtension(path: string): string {
  const name = baseName(path);
  const ext = extension(name);
  return ext ? name.slice(0, -ext.length) : name;
}

/** Join path segments using the separator style of the first segment. */
export function joinPath(...segments: string[]): string {
  const parts = segments.filter((segment) => segment.length > 0);
  if (parts.length === 0) return "";
  const separator = separatorFor(parts[0] ?? "");
  return parts
    .map((segment, index) => {
      const trimmed =
        index === 0 ? segment.replace(/[\\/]+$/, "") : segment.replace(/^[\\/]+|[\\/]+$/g, "");
      return trimmed;
    })
    .filter((segment) => segment.length > 0)
    .join(separator);
}

/** Return `path` with its extension replaced by `ext` (with or without a dot). */
export function replaceExtension(path: string, ext: string): string {
  const dotExt = ext.startsWith(".") ? ext : `.${ext}`;
  const stem = dirName(path)
    ? joinPath(dirName(path), fileNameWithoutExtension(path))
    : fileNameWithoutExtension(path);
  return `${stem}${dotExt}`;
}

/** True when the path has a `.pdf` extension (case-insensitive). */
export function isPdfPath(path: string): boolean {
  return extension(path) === ".pdf";
}

const IMAGE_EXTENSIONS = new Set([".jpg", ".jpeg", ".png", ".webp"]);

/** True when the path is an image type supported by Images to PDF. */
export function isImagePath(path: string): boolean {
  return IMAGE_EXTENSIONS.has(extension(path));
}

/** True when any two paths share the same final component. */
export function shareBaseName(a: string, b: string): boolean {
  return SEPARATORS.test(a) || SEPARATORS.test(b)
    ? baseName(a).toLowerCase() === baseName(b).toLowerCase()
    : a.toLowerCase() === b.toLowerCase();
}
