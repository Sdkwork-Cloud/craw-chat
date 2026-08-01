import io, os, re, sys

GENERIC_FIXES = [
    (re.compile(r'QueryBuilder<\'static,\s*'), "QueryBuilder<"),
    (re.compile(r"QueryBuilder<'q,\s*"), "QueryBuilder<"),
    (re.compile(r"QueryBuilder<'_,\s*"), "QueryBuilder<"),
    (re.compile(r"QueryBuilder<'a,\s*"), "QueryBuilder<"),
    (re.compile(r'Separated<\'static,\s*'), "Separated<"),
    (re.compile(r"Separated<'q,\s*"), "Separated<"),
    (re.compile(r"Separated<'_,\s*"), "Separated<"),
    (re.compile(r"Separated<'a,\s*"), "Separated<"),
    (re.compile(r'RawSql<\'static,\s*'), "RawSql<"),
    (re.compile(r"RawSql<'q,\s*"), "RawSql<"),
    (re.compile(r"RawSql<'_,\s*"), "RawSql<"),
]

def walk_rs(roots):
    for root in roots:
        for dirpath, dirnames, filenames in os.walk(root):
            # skip heavy generated trees
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", ".git", "generated")]
            for fn in filenames:
                if fn.endswith(".rs"):
                    yield os.path.join(dirpath, fn)

def process(path):
    content = io.open(path, encoding="utf-8").read()
    orig = content
    for pat, repl in GENERIC_FIXES:
        content = pat.sub(repl, content)
    if content != orig:
        io.open(path, "w", encoding="utf-8", newline="\n").write(content)
        return True
    return False

if __name__ == "__main__":
    total = 0
    for p in walk_rs(sys.argv[1:]):
        if process(p):
            total += 1
            print("fixed:", p)
    print(f"TOTAL: {total}")
