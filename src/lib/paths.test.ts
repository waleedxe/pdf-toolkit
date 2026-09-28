import { describe, expect, it } from "vitest";

import {
  baseName,
  dirName,
  extension,
  fileNameWithoutExtension,
  isImagePath,
  isPdfPath,
  joinPath,
  replaceExtension,
} from "./paths";

describe("path helpers", () => {
  it("splits Windows and POSIX paths", () => {
    expect(baseName("C:\\Users\\walee\\report.pdf")).toBe("report.pdf");
    expect(baseName("/home/walee/report.pdf")).toBe("report.pdf");
    expect(dirName("C:\\Users\\walee\\report.pdf")).toBe("C:\\Users\\walee");
    expect(dirName("/home/walee/report.pdf")).toBe("/home/walee");
  });

  it("reads extensions case-insensitively", () => {
    expect(extension("Scan.PDF")).toBe(".pdf");
    expect(extension("photo.JPEG")).toBe(".jpeg");
    expect(extension("no-extension")).toBe("");
  });

  it("handles names with multiple dots", () => {
    expect(fileNameWithoutExtension("report.final.pdf")).toBe("report.final");
  });

  it("joins with the separator style of the first segment", () => {
    expect(joinPath("C:\\a", "b", "c.pdf")).toBe("C:\\a\\b\\c.pdf");
    expect(joinPath("/a/", "/b/", "c.pdf")).toBe("/a/b/c.pdf");
  });

  it("replaces extensions without losing the directory", () => {
    expect(replaceExtension("C:\\a\\report.pdf", "jpg")).toBe("C:\\a\\report.jpg");
    expect(replaceExtension("report.pdf", ".png")).toBe("report.png");
  });

  it("identifies supported input types", () => {
    expect(isPdfPath("a.PDF")).toBe(true);
    expect(isPdfPath("a.docx")).toBe(false);
    expect(isImagePath("a.webp")).toBe(true);
    expect(isImagePath("a.gif")).toBe(false);
  });
});
