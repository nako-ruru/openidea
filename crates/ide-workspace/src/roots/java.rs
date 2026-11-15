use super::{ProjectRoots, RootKind};
use ide_core::settings::ProjectStructureSettings;
use std::path::{Component, Path};

/// Whether this looks like an Android project (has an AndroidManifest.xml or a
/// build.gradle(.kts) that applies the Android plugin).
fn is_android_project(server_root: &Path) -> bool {
    if server_root.join("AndroidManifest.xml").is_file() {
        return true;
    }
    for name in ["build.gradle", "build.gradle.kts"] {
        if let Ok(text) = std::fs::read_to_string(server_root.join(name))
            && (text.contains("com.android") || text.contains("android {") || text.contains("android{"))
        {
            return true;
        }
    }
    // Check subdirectories (multi-module): if any child has AndroidManifest or android plugin
    if let Ok(entries) = std::fs::read_dir(server_root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if p.join("AndroidManifest.xml").is_file() {
                    return true;
                }
                for name in ["build.gradle", "build.gradle.kts"] {
                    if let Ok(text) = std::fs::read_to_string(p.join(name))
                        && (text.contains("com.android") || text.contains("android {") || text.contains("android{"))
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Source paths for jdtls's unmanaged project, relative to its workspace folder.
/// Maven, Gradle and Eclipse own their source model; other layouts retain jdtls inference.
/// Android projects are treated as unmanaged so jdtls doesn't run the heavy Gradle
/// import (which freezes the UI); their source roots and the SDK jar are configured
/// separately.
pub fn unmanaged_java_source_paths(
    project: &Path,
    server_root: &Path,
    settings: &ProjectStructureSettings,
) -> Option<Vec<String>> {
    // Android projects: bypass the Gradle marker check.
    if is_android_project(server_root) {
        // Collect all src/main/java (and src/main/kotlin) directories under server_root.
        let mut paths: Vec<String> = Vec::new();
        let mut stack: Vec<PathBuf> = vec![server_root.to_path_buf()];
        // Simple recursive walk (bounded depth).
        for _ in 0..6 {
            let mut next: Vec<PathBuf> = Vec::new();
            for dir in &stack {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if p.is_dir() {
                            // Check if this dir is a source root.
                            if p.ends_with("src/main/java") || p.ends_with("src\\main\\java")
                                || p.ends_with("src/main/kotlin") || p.ends_with("src\\main\\kotlin")
                            {
                                if let Ok(rel) = p.strip_prefix(server_root) {
                                    let s = rel.to_string_lossy().replace('\\', "/");
                                    if !paths.contains(&s) {
                                        paths.push(s);
                                    }
                                }
                            } else {
                                // Don't descend into build/gradle/.git
                                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                                if name != "build" && name != ".gradle" && name != ".git"
                                    && name != "node_modules" && !name.starts_with('.')
                                {
                                    next.push(p);
                                }
                            }
                        }
                    }
                }
            }
            stack = next;
            if stack.is_empty() {
                break;
            }
        }
        if paths.is_empty() {
            // Fallback: conventional single-module layout.
            if server_root.join("src/main/java").is_dir() {
                paths.push("src/main/java".into());
            }
            if server_root.join("src/main/kotlin").is_dir() {
                paths.push("src/main/kotlin".into());
            }
        }
        return if paths.is_empty() { None } else { Some(paths) };
    }

    if ["pom.xml", "build.gradle", "build.gradle.kts", "settings.gradle", "settings.gradle.kts", ".project"]
        .iter()
        .any(|name| server_root.join(name).is_file())
    {
        return None;
    }
    let roots = ProjectRoots::compute(Vec::new(), settings);
    let mut paths: Vec<String> = roots
        .roots
        .iter()
        .filter(|r| matches!(r.kind, RootKind::Source | RootKind::Test))
        .filter(|r| Path::new(&r.path).components().all(|c| matches!(c, Component::Normal(_))))
        .filter_map(|r| {
            let absolute = project.join(&r.path);
            let relative = absolute.strip_prefix(server_root).ok()?;
            let path = relative.to_string_lossy().replace('\\', "/");
            Some(if path.is_empty() { ".".into() } else { path })
        })
        .collect();
    if paths.is_empty() {
        // jdtls can infer the package directory itself as a source root. Give
        // conventional layouts an unambiguous base without guessing other layouts.
        paths = ["src/main/java", "src/test/java"]
            .into_iter()
            .filter(|p| server_root.join(p).is_dir())
            .map(str::to_owned)
            .collect();
        if paths.is_empty() && server_root.join("src").is_dir() {
            paths.push("src".into());
        }
    }
    (!paths.is_empty()).then_some(paths)
}
""", style="xml">
<content">
use super::{ProjectRoots, RootKind};
use ide_core::settings::ProjectStructureSettings;
use std::path::{Component, Path, PathBuf};

/// Whether this looks like an Android project.
fn is_android_project(server_root: &Path) -> bool {
    if server_root.join("AndroidManifest.xml").is_file() {
        return true;
    }
    for name in ["build.gradle", "build.gradle.kts"] {
        if let Ok(text) = std::fs::read_to_string(server_root.join(name))
            && (text.contains("com.android") || text.contains("android {") || text.contains("android{"))
        {
            return true;
        }
    }
    if let Ok(entries) = std::fs::read_dir(server_root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if p.join("AndroidManifest.xml").is_file() {
                    return true;
                }
                for name in ["build.gradle", "build.gradle.kts"] {
                    if let Ok(text) = std::fs::read_to_string(p.join(name))
                        && (text.contains("com.android") || text.contains("android {") || text.contains("android{"))
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Source paths for jdtls's unmanaged project, relative to its workspace folder.
/// Maven, Gradle and Eclipse own their source model; other layouts retain jdtls inference.
/// Android projects are treated as unmanaged so jdtls doesn't run the heavy Gradle
/// import (which freezes the UI); their source roots are collected manually.
pub fn unmanaged_java_source_paths(
    project: &Path,
    server_root: &Path,
    settings: &ProjectStructureSettings,
) -> Option<Vec<String>> {
    if is_android_project(server_root) {
        let mut paths: Vec<String> = Vec::new();
        let mut stack: Vec<PathBuf> = vec![server_root.to_path_buf()];
        for _ in 0..8 {
            let mut next: Vec<PathBuf> = Vec::new();
            for dir in &stack {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if p.is_dir() {
                            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                            if p.ends_with("src/main/java") || p.ends_with("src\\main\\java")
                                || p.ends_with("src/main/kotlin") || p.ends_with("src\\main\\kotlin")
                            {
                                if let Ok(rel) = p.strip_prefix(server_root) {
                                    let s = rel.to_string_lossy().replace('\\', "/");
                                    if !paths.contains(&s) {
                                        paths.push(s);
                                    }
                                }
                            } else if name != "build" && name != ".gradle" && name != ".git"
                                && name != "node_modules" && !name.starts_with('.')
                            {
                                next.push(p);
                            }
                        }
                    }
                }
            }
            stack = next;
            if stack.is_empty() {
                break;
            }
        }
        if paths.is_empty() {
            if server_root.join("src/main/java").is_dir() {
                paths.push("src/main/java".into());
            }
            if server_root.join("src/main/kotlin").is_dir() {
                paths.push("src/main/kotlin".into());
            }
        }
        return if paths.is_empty() { None } else { Some(paths) };
    }

    if ["pom.xml", "build.gradle", "build.gradle.kts", "settings.gradle", "settings.gradle.kts", ".project"]
        .iter()
        .any(|name| server_root.join(name).is_file())
    {
        return None;
    }
    let roots = ProjectRoots::compute(Vec::new(), settings);
    let mut paths: Vec<String> = roots
        .roots
        .iter()
        .filter(|r| matches!(r.kind, RootKind::Source | RootKind::Test))
        .filter(|r| Path::new(&r.path).components().all(|c| matches!(c, Component::Normal(_))))
        .filter_map(|r| {
            let absolute = project.join(&r.path);
            let relative = absolute.strip_prefix(server_root).ok()?;
            let path = relative.to_string_lossy().replace('\\', "/");
            Some(if path.is_empty() { ".".into() } else { path })
        })
        .collect();
    if paths.is_empty() {
        paths = ["src/main/java", "src/test/java"]
            .into_iter()
            .filter(|p| server_root.join(p).is_dir())
            .map(str::to_owned)
            .collect();
        if paths.is_empty() && server_root.join("src").is_dir() {
            paths.push("src".into());
        }
    }
    (!paths.is_empty()).then_some(paths)
}