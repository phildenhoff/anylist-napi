#!/usr/bin/env bash
# Release anylist-napi.
#
#   ./release.sh              # tag v<package.json version>
#   ./release.sh v1.2.3       # tag an explicit version
#   ./release.sh --approve    # skip tagging and CI; approve what is staged
#   ./release.sh --otp 123456 # pass the OTP up front (no terminal needed)
#
# 1. Creates the tag (if needed) and pushes it.
# 2. Waits for the CI run for that tag. CI builds every platform and stages
#    all packages on npm (npm stage publish), which needs no 2FA.
# 3. Approves every staged package with your npm OTP, platform packages
#    first, then the root package.
# 4. Creates the GitHub release with generated notes.
#
# Needs: git, gh (logged in), jq, npm >= 11.15 (or npx), and `npm login`
# done on this machine with 2FA enabled on the account. The OTP is read from
# --otp, then $NPM_OTP, then the terminal.
set -euo pipefail

APPROVE_ONLY=0
OTP=${NPM_OTP:-}
while [[ $# -gt 0 ]]; do
  case $1 in
    --approve) APPROVE_ONLY=1; shift ;;
    --otp) OTP=${2:-}; shift 2 ;;
    --otp=*) OTP=${1#--otp=}; shift ;;
    -*) echo "release: unknown option $1" >&2; exit 2 ;;
    *) break ;;
  esac
done
ROOT_PKG=$(jq -r .name package.json)
PKG_VERSION=$(jq -r .version package.json)
TAG=${1:-v$PKG_VERSION}
VERSION=${TAG#v}

fail() { echo "release: $*" >&2; exit 1; }

# --- npm CLI with staged publishing support ---------------------------------
NPM=(npm)
if ! npm stage --help >/dev/null 2>&1; then
  echo "local npm $(npm --version) lacks 'npm stage'; using npx npm@latest"
  NPM=(npx -y npm@latest)
fi
npm_cmd() { "${NPM[@]}" "$@"; }

# --- preflight ---------------------------------------------------------------
preflight() {
  local tool
  for tool in git gh jq npm; do
    command -v "$tool" >/dev/null || fail "$tool is required"
  done
  [[ "$TAG" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.]+)?$ ]] || fail "tag must look like v1.2.3, got $TAG"
  [[ "$VERSION" == "$PKG_VERSION" ]] || fail "package.json is $PKG_VERSION but tag is $TAG; bump the version first"
  if ! (( APPROVE_ONLY )); then
    [[ -z "$(git status --porcelain)" ]] || fail "working tree is not clean"
    [[ "$(git branch --show-current)" == "main" ]] || fail "run from main"
    git fetch -q origin
    [[ "$(git rev-parse HEAD)" == "$(git rev-parse origin/main)" ]] || fail "main is not in sync with origin/main"
  fi
  gh auth status >/dev/null 2>&1 || fail "gh is not logged in"
  local user
  user=$(npm_cmd whoami 2>/dev/null) || fail "not logged in to npm; run: npm login"
  echo "releasing $ROOT_PKG $TAG as npm user $user"
}

# --- tag and CI --------------------------------------------------------------
tag_and_wait_for_ci() {
  local head_sha tag_sha run_id
  head_sha=$(git rev-parse HEAD)
  if git rev-parse -q --verify "refs/tags/$TAG^{commit}" >/dev/null; then
    tag_sha=$(git rev-parse "refs/tags/$TAG^{commit}")
    [[ "$tag_sha" == "$head_sha" ]] || fail "tag $TAG already exists at ${tag_sha:0:7}, not at HEAD ${head_sha:0:7}"
    echo "tag $TAG already exists at HEAD"
  else
    git tag -a "$TAG" -m "$TAG"
    echo "created tag $TAG"
  fi
  git push origin "refs/tags/$TAG"

  echo "waiting for the CI run for $TAG"
  run_id=""
  for _ in $(seq 1 36); do
    run_id=$(gh run list --branch "$TAG" --event push --json databaseId,headSha \
      --jq ".[] | select(.headSha == \"$head_sha\") | .databaseId" | head -n1)
    [[ -n "$run_id" ]] && break
    sleep 5
  done
  [[ -n "$run_id" ]] || fail "no CI run appeared for $TAG; check GitHub Actions"
  echo "run $run_id: $(gh run view "$run_id" --json url --jq .url)"
  if ! gh run watch "$run_id" --interval 30 --exit-status; then
    gh run view "$run_id" --json jobs \
      --jq '.jobs[] | select(.conclusion != "success" and .conclusion != "skipped") | "  \(.conclusion)\t\(.name)"' >&2
    fail "CI run $run_id did not succeed; nothing was approved"
  fi
}

# --- approve staged packages -------------------------------------------------
read_otp() {
  if [[ -r /dev/tty && -w /dev/tty ]]; then
    read -r -p "npm one-time password: " OTP </dev/tty
  elif [[ -t 0 ]]; then
    read -r -p "npm one-time password: " OTP
  else
    fail "no terminal to ask for the OTP; rerun with --otp CODE (or NPM_OTP=CODE)"
  fi
}

approve_one() {
  local id=$1 name=$2 attempt out
  for attempt in 1 2 3; do
    if [[ -z "$OTP" ]]; then
      read_otp
    fi
    if out=$(npm_cmd stage approve "$id" --otp "$OTP" 2>&1); then
      echo "  approved $name"
      return 0
    fi
    if grep -q "EOTP\|one-time password" <<<"$out"; then
      echo "  OTP rejected for $name (attempt $attempt), enter a fresh code"
      OTP=""
      continue
    fi
    if grep -qi "already" <<<"$out"; then
      echo "  $name was already approved"
      return 0
    fi
    echo "$out" >&2
    fail "could not approve $name ($id)"
  done
  fail "gave up approving $name after 3 OTP attempts"
}

list_staged() {
  # Platform packages first, the root package (which depends on them) last.
  npm_cmd stage list --json 2>/dev/null \
    | jq -c --arg v "$VERSION" --arg root "$ROOT_PKG" \
        '[.[] | select(.version == $v and (.packageName | startswith($root)))]
         | sort_by(.packageName == $root) | .[]'
}

approve_all() {
  local staged item pending
  echo "looking up staged packages for version $VERSION"
  staged=$(list_staged)
  [[ -n "$staged" ]] || fail "no staged packages found for $VERSION; run: npm stage list"
  # npm scans each upload first ("validating"); approval needs "staged".
  for _ in $(seq 1 60); do
    pending=$(jq -r 'select(.status != "staged") | .status' <<<"$staged" | sort -u | tr '\n' ' ')
    [[ -z "$pending" ]] && break
    echo "  waiting for npm validation (${pending% })"
    sleep 10
    staged=$(list_staged)
  done
  echo "$staged" | jq -r '"  \(.status)\t\(.packageName)@\(.version)\t\(.id)"'
  [[ -z "$(jq -r 'select(.status != "staged") | .id' <<<"$staged")" ]] \
    || fail "some packages are still not in the staged state; check: npm stage list"
  while IFS= read -r item; do
    approve_one "$(jq -r .id <<<"$item")" "$(jq -r '"\(.packageName)@\(.version)"' <<<"$item")"
  done <<<"$staged"
}

# --- GitHub release and verification ----------------------------------------
github_release() {
  if gh release view "$TAG" >/dev/null 2>&1; then
    echo "GitHub release $TAG already exists"
  else
    gh release create "$TAG" --title "$TAG" --generate-notes
  fi
}

verify() {
  for _ in $(seq 1 12); do
    if [[ "$(npm view "$ROOT_PKG@$VERSION" version 2>/dev/null)" == "$VERSION" ]]; then
      echo "published: $ROOT_PKG@$VERSION"
      return 0
    fi
    sleep 5
  done
  echo "approved, but $ROOT_PKG@$VERSION is not visible on the registry yet; check npm view in a minute" >&2
}

preflight
if (( APPROVE_ONLY )); then
  echo "skipping tag and CI (--approve)"
else
  tag_and_wait_for_ci
fi
approve_all
github_release
verify
