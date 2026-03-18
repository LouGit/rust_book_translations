#!/usr/bin/env bash

PO=translations/book/fr.po
BOOKDIR=translations/book/html

TMP_PO=$(mktemp)
TMP_HTML=$(mktemp)

# extraire msgid du po
grep '^msgid "' "$PO" \
| sed 's/^msgid "//' \
| sed 's/"$//' \
| sed '/^$/d' \
| sort -u > "$TMP_PO"

# extraire texte du html
grep -hoP '(?<=>)[^<>]+(?=<)' $BOOKDIR/*.html \
| sed 's/^[[:space:]]*//' \
| sed 's/[[:space:]]*$//' \
| sed '/^$/d' \
| sort -u > "$TMP_HTML"

echo "=== chaînes présentes dans HTML mais absentes du PO ==="
comm -23 "$TMP_HTML" "$TMP_PO"

rm "$TMP_PO" "$TMP_HTML"
