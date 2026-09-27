# API contract

Base URL: `https://www.wanandroid.com/`

The server response envelope uses camelCase JSON keys:

```json
{
  "errorCode": 0,
  "errorMsg": "",
  "data": {}
}
```

The current Rust slice implements:

| Feature | Method | Path |
|---|---|---|
| Home banners | GET | `/banner/json` |
| Home articles | GET | `/article/list/{page}/json` |

The remaining endpoints are deliberately reserved for the next slices:

- Search: `/article/query/{page}/json`, `/hotkey/json`
- Project: `/project/tree/json`, `/project/list/{page}/json`
- Auth: `/user/login`, `/user/logout/json`
- Collection: `/lg/collect/list/{page}/json`, `/lg/collect/{id}/json`, `/lg/uncollect_originId/{id}/json`

Business errors (`errorCode != 0`) are mapped to `RepositoryError::Api`. Network failures and malformed JSON are kept distinct, so Kotlin can render retryable and non-retryable error states differently.
