import { beforeEach, describe, expect, it } from "vitest";
import { gitRevisionOf } from "@/platform/virtualDocs";
import { sideDocumentUri } from "../diffInput";
import { useVcs } from "../store";

beforeEach(() => {
  useVcs.setState({ roots: ["/p", "/p/sub"] });
});

describe("sideDocumentUri", () => {
  it("opens a revision side as a read-only document of the file's repository version", () => {
    const uri = sideDocumentUri({ type: "rev", rev: "HEAD" }, "/p/src/a.ts")!;
    expect(gitRevisionOf(uri)).toEqual({ repo: "/p", revision: "HEAD", path: "src/a.ts" });
  });

  it("uses the side's old path for a rename, in its innermost repository", () => {
    const uri = sideDocumentUri({ type: "rev", rev: "HEAD", path: "/p/sub/old.ts" }, "/p/src/a.ts")!;
    expect(gitRevisionOf(uri)).toEqual({ repo: "/p/sub", revision: "HEAD", path: "old.ts" });
  });

  it("keeps file-backed sides and leaves the index, absent files and untracked paths alone", () => {
    expect(sideDocumentUri({ type: "worktree" }, "/p/a.ts")).toBe("/p/a.ts");
    expect(sideDocumentUri({ type: "file", path: "/p/b.ts" }, "/p/a.ts")).toBe("/p/b.ts");
    expect(sideDocumentUri({ type: "rev", rev: ":0" }, "/p/a.ts")).toBeNull();
    expect(sideDocumentUri({ type: "text", text: "", label: "" }, "/p/a.ts")).toBeNull();
    expect(sideDocumentUri({ type: "rev", rev: "HEAD" }, "/elsewhere/a.ts")).toBeNull();
  });
});
