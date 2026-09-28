import { useEffect, useState } from "react";

import { invokeCommand } from "../lib/tauri";

export interface Thumbnail {
  dataUrl: string;
  width: number;
  height: number;
}

interface ThumbnailResponse {
  data_url: string;
  width: number;
  height: number;
}

// Module-level caches shared across every thumbnail instance. Thumbnails are
// only ever rendered once per (path, page, width); scrolling back is instant.
const cache = new Map<string, Thumbnail>();
const inflight = new Map<string, Promise<Thumbnail>>();

function keyOf(path: string, page: number, maxWidth: number): string {
  return `${path}|${page}|${maxWidth}`;
}

function load(path: string, page: number, maxWidth: number): Promise<Thumbnail> {
  const key = keyOf(path, page, maxWidth);
  const cached = cache.get(key);
  if (cached) return Promise.resolve(cached);

  const pending = inflight.get(key);
  if (pending) return pending;

  const promise = invokeCommand<ThumbnailResponse>("render_thumbnail", {
    request: { input: path, page, max_width: maxWidth },
  })
    .then((response) => {
      const thumbnail: Thumbnail = {
        dataUrl: response.data_url,
        width: response.width,
        height: response.height,
      };
      cache.set(key, thumbnail);
      inflight.delete(key);
      return thumbnail;
    })
    .catch((error: unknown) => {
      inflight.delete(key);
      throw error;
    });

  inflight.set(key, promise);
  return promise;
}

/**
 * Lazily render (and cache) a single page thumbnail.
 *
 * Pass `null` as the path to stay idle — the thumbnail grid uses this to only
 * render pages near the viewport, so large PDFs never freeze the UI.
 */
export function useThumbnail(
  path: string | null,
  page: number,
  maxWidth: number,
): { thumbnail: Thumbnail | null; error: string | null } {
  const [thumbnail, setThumbnail] = useState<Thumbnail | null>(() =>
    path ? (cache.get(keyOf(path, page, maxWidth)) ?? null) : null,
  );
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!path) return;
    let active = true;

    load(path, page, maxWidth)
      .then((value) => {
        if (!active) return;
        setThumbnail(value);
        setError(null);
      })
      .catch((cause: unknown) => {
        if (active) setError(cause instanceof Error ? cause.message : String(cause));
      });

    return () => {
      active = false;
    };
  }, [path, page, maxWidth]);

  return { thumbnail, error };
}
