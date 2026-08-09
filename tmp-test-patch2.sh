#!/bin/bash
b64() { printf '%s' "$1" | base64 -w0 | tr '+/' '-_' | tr -d '='; }
H=$(b64 '{"alg":"none","typ":"JWT"}')
P=$(b64 '{"token_type":"access","tenant_id":"100001","user_id":"system","app_id":"app_100001","login_scope":"TENANT","token_version":1}')
BT="$H.$P.test-signature"
RESP=$(curl --noproxy '*' -s -X POST http://127.0.0.1:18079/app/v3/api/auth/sessions \
  -H 'Content-Type: application/json' -H "access-token: $BT" \
  -d '{"grantType":"password","username":"admin","password":"sdkwork-im-admin-dev-2026"}')
AT=$(printf '%s' "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); print(d.get('data',{}).get('accessToken',''))" 2>/dev/null)
AUTH=$(printf '%s' "$RESP" | python3 -c "import json,sys; d=json.load(sys.stdin); print(d.get('data',{}).get('authToken',''))" 2>/dev/null)
echo "== PATCH displayName=TestAdmin =="
curl --noproxy '*' -s -o /dev/null -w "patch status: %{http_code}\n" -X PATCH http://127.0.0.1:18079/app/v3/api/iam/users/current \
  -H "Authorization: Bearer $AUTH" -H "access-token: $AT" -H 'Content-Type: application/json' \
  -d '{"displayName":"TestAdmin"}'
echo "== GET after patch =="
curl --noproxy '*' -s http://127.0.0.1:18079/app/v3/api/iam/users/current \
  -H "Authorization: Bearer $AUTH" -H "access-token: $AT" | python3 -c "import json,sys; d=json.load(sys.stdin); print('displayName:', d['data']['displayName'])"
echo "== restore =="
curl --noproxy '*' -s -o /dev/null -w "restore status: %{http_code}\n" -X PATCH http://127.0.0.1:18079/app/v3/api/iam/users/current \
  -H "Authorization: Bearer $AUTH" -H "access-token: $AT" -H 'Content-Type: application/json' \
  -d '{"displayName":"Administrator"}'
