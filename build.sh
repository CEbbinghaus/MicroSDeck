#!/bin/sh
exec node "--no-warnings=ExperimentalWarning" "util/build.mjs" "$@"
