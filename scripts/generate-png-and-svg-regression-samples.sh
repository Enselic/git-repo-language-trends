#!/usr/bin/env bash
set -o errexist -o nounset -o pipefail

echo "This script generates a set of SVG and PNG files that is meant to be"
echo "used as a visual regression test. Since it is visual regression"
echo "test, it requires a human."

# Use same tag for consistentcy across runs
TAG="v0.0.4-pip"
OUTDIR=$(mktemp -d)

for format in "svg" "png"; do
    for type in "--relative" ""; do
        for watermark in "--no-watermark" ""; do
            for size in "6:4" "11.75:8.25" "23:16"; do
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

echo "Output put in $OUTDIR, trying to 'xdg-open' for you now"
xdg-open $OUTDIR
