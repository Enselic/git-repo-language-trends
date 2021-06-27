#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

echo "
This script generates a set of SVG and PNG files that is meant to be
used as a visual regression test. Since it is visual regression
test, it requires a human.
"

# Use same tag for consistentcy across runs
TAG="v0.0.4-pip"
if [ -z "${OUTDIR-}" ]; then
    OUTDIR=$(mktemp -d)
else
    echo "Using pre-set ${OUTDIR}"
fi

for format in "svg" "png"; do
    for type in "--relative" ""; do
        for watermark in "--no-watermark" ""; do
            for size in "6,4" "11.75,8.25" "23,16"; do
                git-repo-language-trends \
                    --first-commit ${TAG} \
                    ${type} \
                    ${watermark} \
                    --size-inches ${size} \
                    -o ${OUTDIR}/visual-regression-test--size-inches-${size}${type}${watermark}.${format}
            done
        done
    done
done

echo "
Output put in
${OUTDIR}
"

# Automatically open the dir with the result if not in CI
if [ -z "${CI-}" ]; then
    echo "Not in CI, opening for you"
    xdg-open "${OUTDIR}"
else
    echo "In CI, not opening anything"
fi
