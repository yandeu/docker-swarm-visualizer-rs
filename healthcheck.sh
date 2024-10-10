#!/bin/sh

set -eu

case "${VISUALIZER_TYPE:-}" in
    "manager")
        PORT=9510;;
    "agent")
        PORT=9511;;
    *)
        PORT=3500;;
esac

curl -sf http://localhost:${PORT}/healthcheck || exit 1
