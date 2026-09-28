import type { LucideIcon } from "lucide-react";
import {
  Combine,
  Scissors,
  FileOutput,
  FileMinus,
  RotateCw,
  LayoutGrid,
  Images,
  Image,
  Stamp,
  Hash,
  Lock,
  Unlock,
} from "lucide-react";

export type ToolCategory = "organize" | "convert" | "edit" | "security";

export type ToolId =
  | "merge"
  | "split"
  | "extract"
  | "remove"
  | "rotate"
  | "reorder"
  | "images-to-pdf"
  | "pdf-to-images"
  | "watermark"
  | "page-numbers"
  | "protect"
  | "unlock";

export interface ToolMeta {
  id: ToolId;
  category: ToolCategory;
  icon: LucideIcon;
}

/** Single source of truth for the tool catalog and its ordering. */
export const TOOL_CATEGORIES: readonly ToolCategory[] = [
  "organize",
  "convert",
  "edit",
  "security",
] as const;

export const TOOLS: readonly ToolMeta[] = [
  { id: "merge", category: "organize", icon: Combine },
  { id: "split", category: "organize", icon: Scissors },
  { id: "extract", category: "organize", icon: FileOutput },
  { id: "remove", category: "organize", icon: FileMinus },
  { id: "rotate", category: "edit", icon: RotateCw },
  { id: "reorder", category: "organize", icon: LayoutGrid },
  { id: "images-to-pdf", category: "convert", icon: Images },
  { id: "pdf-to-images", category: "convert", icon: Image },
  { id: "watermark", category: "edit", icon: Stamp },
  { id: "page-numbers", category: "edit", icon: Hash },
  { id: "protect", category: "security", icon: Lock },
  { id: "unlock", category: "security", icon: Unlock },
] as const;

export const TOOL_IDS: readonly ToolId[] = TOOLS.map((tool) => tool.id);

export function getToolMeta(id: ToolId): ToolMeta {
  const meta = TOOLS.find((tool) => tool.id === id);
  if (!meta) {
    throw new Error(`Unknown tool id: ${id}`);
  }
  return meta;
}

export function getToolsByCategory(category: ToolCategory): ToolMeta[] {
  return TOOLS.filter((tool) => tool.category === category);
}
