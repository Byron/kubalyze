#!/bin/bash

set -eu -o pipefail
exe=${1:?First argument is the executable under test}

root="$(cd "${0%/*}" && pwd)"
# shellcheck source=./tests/utilities.sh
source "$root/utilities.sh"

SUCCESSFULLY=0

title "adherence to best practices: regular deployments"
fixture="$root/fixtures/kube-system"
snapshot="$root/fixtures/snapshots/deployments"

(with "a listing of deployments that have not been updated for a long time"
  it "succeeds and lists all offending deployments, as well as what should be done" && {
    WITH_SNAPSHOT="$snapshot/deployments-with-a-lack-of-updates" \
    expect_run $SUCCESSFULLY "$exe" "$fixture/deployments.json"
  }
)
