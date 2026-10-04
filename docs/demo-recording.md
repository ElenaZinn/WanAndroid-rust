# Recording a demo against real data

`wanandroid.com`'s TLS certificate expired on 2026-10-03, and the server redirects all plain HTTP
to HTTPS. A client that validates certificates (the Rust core uses rustls) therefore cannot reach
the real API. The application code is unchanged by this; only the endpoint it talks to is
redirected while recording.

## 1. Start the local proxy

```bash
python3 scripts/wanandroid-demo-proxy.py --host 0.0.0.0 --port 8080
```

It forwards to the upstream site over TLS without verifying that certificate and relays the
response verbatim, including `Set-Cookie`, so login and session behaviour still work. It is a
recording aid and never ships in the app.

## 2. Build with the endpoint override

```bash
MACIP=$(ipconfig getifaddr en0)
./android/gradlew -p android -PwanandroidBaseUrl="http://$MACIP:8080" assembleDebug
adb install -r android/app/build/outputs/apk/debug/app-debug.apk
```

`BuildConfig.WANANDROID_BASE_URL` is empty by default, so a normal build talks to production. The
phone must be on the same network as the machine running the proxy.

Cleartext is permitted only in the `debug` variant (`android/app/src/debug/`); the release build
keeps Android's default.

Banner images and article links come back as absolute upstream URLs. `DemoEndpoint` rewrites only
their scheme and authority onto the configured endpoint so they load through the proxy too; it
names no host, keeping API addresses out of the Kotlin layer.

## 3. Record

```bash
scrcpy --no-window --record=demo.mp4 --no-audio
adb shell svc power stayon true
```

`--no-window` records without opening a window, which is what works in a headless/agent shell. Stop
it with `SIGTERM` so the mp4 index is written; a killed process leaves an unplayable file.

## Alternative

`adb reverse tcp:8080 tcp:8080` also works, but the rule is dropped whenever the adb session is
re-established, so it must be set up again in the same step that triggers the request.
