#!/bin/bash

set -eu

cargo build --release

mkdir -p tmp
rm -rf tmp/*.md
rm -rf tmp/markdown

# Render the book as Markdown to include all the code listings
MDBOOK_OUTPUT__MARKDOWN=1 mdbook build -d tmp

pwd
# ls tmp/markdown

# Get all the Markdown files
find tmp/markdown -name "${1:-\"\"}*.md" -print0 | \
# Extract just the filename so we can reuse it easily.
xargs -0 basename | \
# Remove all links followed by `<!-- ignore -->``, then
# Change all remaining links from Markdown to italicized inline text.
while IFS= read -r filename; do
  < "tmp/markdown/$filename" ./target/aarch64-apple-darwin/release/remove_links \
    | ./target/aarch64-apple-darwin/release/link2print \
    | ./target/aarch64-apple-darwin/release/remove_markup \
    | ./target/aarch64-apple-darwin/release/remove_hidden_lines > "tmp/$filename"
done
pwd

# Concatenate the files into the `nostarch` dir.
./target/aarch64-apple-darwin/release/concat_chapters tmp nostarch
