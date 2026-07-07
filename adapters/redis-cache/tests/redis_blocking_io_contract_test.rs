use std::fs;
use std::path::Path;

#[test]
fn redis_sync_io_uses_bounded_connection_timeouts() {
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut violations = Vec::new();
    collect_unbounded_redis_connection_calls(source_root.as_path(), &mut violations);

    assert!(
        violations.is_empty(),
        "Redis IO must use bounded Redis helpers instead of unbounded connection constructors:\n{}",
        violations.join("\n")
    );
}

fn collect_unbounded_redis_connection_calls(path: &Path, violations: &mut Vec<String>) {
    for entry in fs::read_dir(path).expect("read redis-cache source directory") {
        let entry = entry.expect("read redis-cache source entry");
        let path = entry.path();
        if path.is_dir() {
            collect_unbounded_redis_connection_calls(path.as_path(), violations);
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let source = fs::read_to_string(path.as_path()).expect("read redis-cache source file");
        for (index, line) in source.lines().enumerate() {
            if line.contains(".get_connection()")
                || line.contains("Pool<redis::Client>")
                || line.contains("r2d2::Pool<redis::Client>")
                || line.contains("ConnectionManager::new(")
            {
                violations.push(format!("{}:{}", path.display(), index + 1));
            }
        }
    }
}
