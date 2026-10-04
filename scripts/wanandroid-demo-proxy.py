#!/usr/bin/env python3
"""Local demo proxy for the WanAndroid Rust client.

`wanandroid.com` had its TLS certificate expire on 2026-10-03, so a client that
validates certificates (the Rust core uses rustls) cannot reach the real API.
This proxy terminates plain HTTP on the machine running it, forwards to the
upstream site over TLS without verifying that certificate, and relays the
response verbatim, including `Set-Cookie`, so login/session behaviour still
works.

This exists only to record a demo against real data. It is not part of the
application: the shipped app talks to the upstream host directly.

Usage:
    python3 scripts/wanandroid-demo-proxy.py --port 8080
    adb reverse tcp:8080 tcp:8080
"""

from __future__ import annotations

import argparse
import ssl
import sys
import urllib.error
import urllib.request
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse

DEFAULT_UPSTREAM = "https://wanandroid.com"

# Connections are managed per request; these must not be forwarded verbatim.
HOP_BY_HOP = {
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailers",
    "transfer-encoding",
    "upgrade",
    "content-length",
    "host",
}

TLS_CONTEXT = ssl.create_default_context()
TLS_CONTEXT.check_hostname = False
TLS_CONTEXT.verify_mode = ssl.CERT_NONE

UPSTREAM = DEFAULT_UPSTREAM


class ProxyHandler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"
    server_version = "WanAndroidDemoProxy/1.0"

    def do_GET(self) -> None:  # noqa: N802
        self._forward("GET")

    def do_POST(self) -> None:  # noqa: N802
        self._forward("POST")

    def do_PUT(self) -> None:  # noqa: N802
        self._forward("PUT")

    def do_DELETE(self) -> None:  # noqa: N802
        self._forward("DELETE")

    def _forward(self, method: str) -> None:
        declared = self.headers.get("Content-Length")
        body = self.rfile.read(int(declared)) if declared else None

        headers = {
            name: value
            for name, value in self.headers.items()
            if name.lower() not in HOP_BY_HOP
        }
        headers["Host"] = urlparse(UPSTREAM).netloc

        request = urllib.request.Request(
            UPSTREAM + self.path,
            data=body,
            headers=headers,
            method=method,
        )

        try:
            with urllib.request.urlopen(request, context=TLS_CONTEXT, timeout=30) as response:
                self._relay(response.status, response.headers, response.read())
        except urllib.error.HTTPError as error:
            payload = error.read()
            print(f"  upstream {error.code} {method} {self.path}", file=sys.stderr)
            self._relay(error.code, error.headers, payload)
        except Exception as error:  # noqa: BLE001 - surface any transport failure to the client
            print(f"  upstream failure {method} {self.path}: {error}", file=sys.stderr)
            self.send_error(502, f"upstream failure: {error}")

    def _relay(self, status: int, headers, payload: bytes) -> None:
        print(f"  {status} {self.command} {self.path}", file=sys.stderr)

        self.send_response(status)
        for name, value in headers.items():
            if name.lower() in HOP_BY_HOP:
                continue
            self.send_header(name, value)
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        if payload:
            self.wfile.write(payload)

    def log_message(self, fmt: str, *args) -> None:
        # Keep stdout quiet; _relay already reports one concise line per request.
        return


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--port", type=int, default=8080)
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--upstream", default=DEFAULT_UPSTREAM)
    options = parser.parse_args()

    global UPSTREAM
    UPSTREAM = options.upstream.rstrip("/")

    server = ThreadingHTTPServer((options.host, options.port), ProxyHandler)
    print(f"WanAndroid demo proxy on http://{options.host}:{options.port} -> {UPSTREAM}")
    print("Press Ctrl+C to stop.")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nstopped")
    finally:
        server.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
