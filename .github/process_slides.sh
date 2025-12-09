# loop through and only process files with the "marp" attribute that aren't in draft mode
for md in lectures/**/*.md; do
    if grep -q "draft:\s*true" $md; then
        continue
    fi
    
    if grep -q "marp:\s*true" $md; then
        doc=$(basename "$md" .md)
        npx @marp-team/marp-cli@latest --theme lectures/marp-theme.css --allow-local-files --pdf --html $md -o lectures/pdfs/$doc.pdf
        npx @marp-team/marp-cli@latest --theme lectures/marp-theme.css --allow-local-files --bespoke.progress --html $md -o lectures/slides/$doc.html
    fi
done
