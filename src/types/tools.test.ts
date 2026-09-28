import { describe, expect, it } from "vitest";

import { getToolMeta, getToolsByCategory, TOOLS, TOOL_CATEGORIES } from "./tools";

describe("tool catalog", () => {
  it("contains exactly the twelve MVP tools", () => {
    expect(TOOLS).toHaveLength(12);
    expect(new Set(TOOLS.map((tool) => tool.id)).size).toBe(12);
  });

  it("assigns every tool to a known category", () => {
    for (const tool of TOOLS) {
      expect(TOOL_CATEGORIES).toContain(tool.category);
    }
  });

  it("groups tools by category without losing any", () => {
    const grouped = TOOL_CATEGORIES.flatMap((category) => getToolsByCategory(category));
    expect(grouped).toHaveLength(TOOLS.length);
  });

  it("returns metadata for a known tool and throws for an unknown one", () => {
    expect(getToolMeta("merge").id).toBe("merge");
    expect(() => getToolMeta("nope" as never)).toThrow();
  });
});
