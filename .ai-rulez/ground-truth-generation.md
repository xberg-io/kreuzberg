# Ground Truth Generation

## Pandoc Commands

```bash
pandoc <source_file> -t gfm --wrap=none -o <gt_file.md>
pandoc <source_file> -t plain --wrap=none -o <gt_file.txt>
```

## Artifact Removal

```bash
sed -i '' 's/ {#[^}]*}//g' "$file"   # Remove {#id} attributes
sed -i '' 's/ {[^}]*}//g' "$file"     # Remove {.class} attributes
sed -i '' '/^:::/d' "$file"            # Remove fenced div markers
sed -i '' 's/\\\$/$/g' "$file"         # Unescape dollar signs
sed -i '' "s/\\\\'/'/g" "$file"        # Unescape quotes
```

## Cleanup Rules

1. Convert ALL HTML to markdown equivalents where possible
2. For colspan/rowspan, put content in first cell, leave others empty
3. Remove `<!-- -->` comments
4. Strip `<u>`, `<sup>`, `<sub>` tags (keep text content)
5. Convert `<img>` to `![alt](src)`
6. Collapse 3+ consecutive blank lines to 2
7. Never use our own extractor output as GT

## Fixture JSON Structure

```json
{
  "document": "relative/path/to/source.ext",
  "file_type": "docx",
  "file_size": 12345,
  "expected_frameworks": ["xberg"],
  "metadata": { "description": "...", "source": "pandoc-generated" },
  "ground_truth": {
    "text_file": "relative/path/to/gt.txt",
    "markdown_file": "relative/path/to/gt.md",
    "source": "pandoc"
  }
}
```
