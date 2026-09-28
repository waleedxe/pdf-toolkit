import { describe, expect, it } from "vitest";

import en from "./en.json";
import ar from "./ar.json";

function flattenKeys(value: unknown, prefix = ""): string[] {
  if (typeof value !== "object" || value === null) {
    return [prefix];
  }
  return Object.entries(value as Record<string, unknown>).flatMap(([key, child]) =>
    flattenKeys(child, prefix ? `${prefix}.${key}` : key),
  );
}

describe("translations", () => {
  it("exposes the same key set in English and Arabic", () => {
    const enKeys = flattenKeys(en).sort();
    const arKeys = flattenKeys(ar).sort();
    expect(arKeys).toEqual(enKeys);
  });

  it("does not ship empty strings", () => {
    const empty = flattenKeys(ar).filter(
      (key) => (ar as Record<string, unknown>)[key] === "" || key === "",
    );
    expect(empty).toEqual([]);
  });
});
