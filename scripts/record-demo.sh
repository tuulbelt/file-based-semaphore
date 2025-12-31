#!/bin/bash
# Record File-Based Semaphore (Rust) demo
source "$(dirname "$0")/lib/demo-framework.sh"

TOOL_NAME="file-based-semaphore"
SHORT_NAME="sema"
LANGUAGE="rust"

# GIF parameters
GIF_COLS=100
GIF_ROWS=30
GIF_SPEED=1.0
GIF_FONT_SIZE=14

demo_commands() {
  # ═══════════════════════════════════════════
  # File-Based Semaphore / sema - Tuulbelt
  # ═══════════════════════════════════════════

  # Step 1: Installation
  echo "# Step 1: Install globally"
  sleep 0.5
  echo "$ cargo install --path ."
  sleep 1

  # Step 2: View help
  echo ""
  echo "# Step 2: View available commands"
  sleep 0.5
  echo "$ sema --help"
  sleep 0.5
  "$BIN" --help | head -30
  sleep 3

  # Step 3: Try to acquire lock (non-blocking)
  echo ""
  echo "# Step 3: Try to acquire lock (non-blocking)"
  sleep 0.5
  echo "$ sema try /tmp/demo.lock --tag \"demo process\""
  sleep 0.5
  "$BIN" try /tmp/demo.lock --tag "demo process"
  echo "✓ Lock acquired!"
  sleep 2

  # Step 4: Check lock status
  echo ""
  echo "# Step 4: Check lock status"
  sleep 0.5
  echo "$ sema status /tmp/demo.lock"
  sleep 0.5
  "$BIN" status /tmp/demo.lock
  sleep 2

  # Step 5: Try to acquire again (should fail)
  echo ""
  echo "# Step 5: Try to acquire again (should fail)"
  sleep 0.5
  echo "$ sema try /tmp/demo.lock --tag \"second process\""
  sleep 0.5
  "$BIN" try /tmp/demo.lock --tag "second process" || echo "✓ Lock held by first process"
  sleep 2

  # Step 6: Release lock
  echo ""
  echo "# Step 6: Release lock"
  sleep 0.5
  echo "$ sema release /tmp/demo.lock"
  sleep 0.5
  "$BIN" release /tmp/demo.lock
  echo "✓ Lock released"
  sleep 2

  # Step 7: Acquire with timeout and JSON status
  echo ""
  echo "# Step 7: Acquire with timeout, check JSON status"
  sleep 0.5
  echo "$ sema acquire /tmp/demo.lock --timeout 5 --tag \"timed lock\""
  "$BIN" acquire /tmp/demo.lock --timeout 5 --tag "timed lock"
  sleep 0.5
  echo "$ sema status /tmp/demo.lock --json"
  "$BIN" status /tmp/demo.lock --json
  sleep 2
  "$BIN" release /tmp/demo.lock

  # Cleanup
  rm -f /tmp/demo.lock

  echo ""
  echo "# Done! Coordinate processes with: sema acquire <path>"
  sleep 1
}

run_demo

# Demo regenerated 2025-12-30
