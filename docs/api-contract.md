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
| Project categories | GET | `/project/tree/json` |
| Project articles | GET | `/project/list/{page}/json?cid={categoryId}` |
| Collection list | GET | `/lg/collect/list/{page}/json` |
| Collect article | POST form | `/lg/collect/{id}/json` |
| Uncollect article | POST form | `/lg/uncollect_originId/{id}/json` |
| Login | POST form (`username`, `password`) | `/user/login` |
| Logout | GET | `/user/logout/json` |

The remaining endpoint integration is Android binding generation and a production HTTP client; the feature repositories and state machines are now defined without putting business rules in Kotlin.

Business errors (`errorCode != 0`) are mapped to `RepositoryError::Api`. Network failures and malformed JSON are kept distinct, so Kotlin can render retryable and non-retryable error states differently.
