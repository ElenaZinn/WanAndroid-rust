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

The current Rust slices implement:

| Feature | Method | Path |
|---|---|---|
| Home banners | GET | `/banner/json` |
| Home articles | GET | `/article/list/{page}/json` |
| Search hot keys | GET | `/hotkey/json` |
| Search articles | POST form (`k`) | `/article/query/{page}/json` |
| Search by author | GET | `/article/list/{page}/json?author={encoded}` |
| Login | POST form (`username`, `password`) | `/user/login` |
| Logout | GET | `/user/logout/json` |

The remaining endpoints are deliberately reserved for the next slices:

- Project: `/project/tree/json`, `/project/list/{page}/json`
- Collection: `/lg/collect/list/{page}/json`, `/lg/collect/{id}/json`, `/lg/uncollect_originId/{id}/json`

Business errors (`errorCode != 0`) are mapped to `RepositoryError::Api`. Network failures and malformed JSON are kept distinct, so Kotlin can render retryable and non-retryable error states differently.
