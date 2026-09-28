import { useEffect, useRef, useState } from "react";
import { FileWarning, Loader2 } from "lucide-react";

import { useThumbnail } from "../../hooks/useThumbnail";

interface PageThumbnailProps {
  path: string;
  page: number;
  maxWidth: number;
}

/**
 * Renders a page preview only once it is near the viewport.
 *
 * Keeps large documents responsive: off-screen pages are never rasterized.
 */
export function PageThumbnail({ path, page, maxWidth }: PageThumbnailProps) {
  const ref = useRef<HTMLDivElement>(null);
  const [visible, setVisible] = useState(() => typeof IntersectionObserver === "undefined");

  useEffect(() => {
    const node = ref.current;
    if (!node || visible) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries.some((entry) => entry.isIntersecting)) {
          setVisible(true);
          observer.disconnect();
        }
      },
      { rootMargin: "300px" },
    );
    observer.observe(node);
    return () => observer.disconnect();
  }, [visible]);

  const { thumbnail, error } = useThumbnail(visible ? path : null, page, maxWidth);

  return (
    <div
      ref={ref}
      className="flex aspect-[3/4] items-center justify-center overflow-hidden rounded-md border border-slate-200 bg-slate-50"
    >
      {thumbnail ? (
        <img
          src={thumbnail.dataUrl}
          alt=""
          draggable={false}
          className="h-full w-full object-contain"
        />
      ) : error ? (
        <FileWarning className="h-5 w-5 text-rose-400" aria-hidden="true" />
      ) : (
        <Loader2 className="h-5 w-5 animate-spin text-slate-300" aria-hidden="true" />
      )}
    </div>
  );
}
