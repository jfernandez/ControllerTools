#!/bin/bash

VSCODE_WS="$1"
SSH_REMOTE="$2"
GDBPORT="$3"

APP="controller-tools"
TARGET_ARCH="x86_64-unknown-linux-gnu"
BUILD_BIN_FILE="${VSCODE_WS}/backend/target/${TARGET_ARCH}/debug/${APP}"
TARGET_USER="deck"
TARGET_BIN_FILE="/home/deck/homebrew/${APP}"
TARGET_CWD="/home/deck"

ssh "${TARGET_USER}@${SSH_REMOTE}" "killall gdbserver ${APP}"

if ! rsync -avz "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
    # If rsync doesn't work, it may not be available on target. Fallback to trying SSH copy.
    if ! scp "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
        exit 2
    fi
fi

ssh -f "${TARGET_USER}@${SSH_REMOTE}" "sh -c 'gdbserver *:${GDBPORT} ${TARGET_BIN_FILE}'"