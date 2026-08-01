#!/usr/bin/env bash
# Usage: converge-repo.sh <repo-dir> <repo-root-abs> [max-rounds]
REPO="$1"
ROOT="$2"
MAX="${3:-15}"
LOG="C:/Users/admin/AppData/Local/Temp/converge-$REPO.log"
cd "$ROOT" || exit 2
for i in $(seq 1 "$MAX"); do
  python "E:/sdkwork-space/sdkwork-im/tools/fix-sqlx09-sites.py" "$LOG" "$ROOT" > /tmp/conv-fix.log 2>&1
  cargo check --workspace --lib --bins > "$LOG" 2>&1
  n=$(python -c "
lines = open(r'$LOG', encoding='utf-8', errors='replace').readlines()
errs = [l for l in lines if 'could not compile' in l]
print(len(errs))")
  echo "round $i: fail-crates=$n"
  if [ "$n" = "0" ]; then echo "GREEN"; exit 0; fi
done
echo "NOT-GREEN"
exit 1
