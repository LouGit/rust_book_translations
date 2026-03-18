#!/usr/bin/env bash

PO=translations/book/fr.po
BOOKDIR=translations/book/html

TMP_PO=$(mktemp)
TMP_HTML=$(mktemp)

# extraire tous les msgid
grep '^msgid "' "$PO" \
| sed 's/^msgid "//' \
| sed 's/"$//' \
| sed '/^$/d' \
| sort -u > "$TMP_PO"

# extraire texte visible du HTML
grep -hoP '(?<=>)[^<>]+(?=<)' $BOOKDIR/*.html \
| sed 's/^[[:space:]]*//' \
| sed 's/[[:space:]]*$//' \
| sed '/^$/d' \
| sort -u > "$TMP_HTML"

echo "=== msgid présents dans le PO mais absents du HTML ==="
comm -23 "$TMP_PO" "$TMP_HTML"

rm "$TMP_PO" "$TMP_HTML"
