/**
 * Diff tabs stay mounted while another tab is shown: the panes keep their
 * scroll offsets, so switching away and back does not jump to the top.
 */
import { describe, expect, it } from "vitest";
import { useContributions } from "@/workbench/contributions";
import { COMBINED_DIFF_KIND } from "../allInOneDiff";
import { DIFF_KIND } from "../diffInput";
import { activateDiffViewer } from "../diffViewer";

describe("diff editor kinds", () => {
  it("stays mounted while hidden, so its scroll offset survives a tab switch", () => {
    const deactivate = activateDiffViewer();
    try {
      const kinds = useContributions.getState().editorKinds;
      expect(kinds.get(DIFF_KIND)).toMatchObject({ keepAlive: true });
      expect(kinds.get(COMBINED_DIFF_KIND)).toMatchObject({ keepAlive: true });
    } finally {
      deactivate();
    }
  });
});
