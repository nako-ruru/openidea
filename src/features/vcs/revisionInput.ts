import { useVcs } from "./store";
import { gitRevisionUri } from "@/platform/virtualDocs";
import { editorService } from "@/workbench/contributions";

export interface RevisionData {
  repo: string;
  revision: string;
  path: string;
  external?: boolean;
}

/** Opens one file at a Git revision in the read-only revision editor. */
export function openRevisionFile(repo: string, revision: string, path: string, external = false) {
  if (useVcs.getState().root) {
    void editorService().openFile(gitRevisionUri(repo, revision, path), { focus: true });
  } else
    void editorService().openInput({
      kind: "git-revision",
      key: `git-revision:${repo}\0${revision}\0${path}`,
      data: { repo, revision, path, external } satisfies RevisionData,
    });
}
