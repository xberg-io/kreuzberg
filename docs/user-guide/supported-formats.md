# Supported Formats

Kreuzberg handles a wide range of document, image, and text formats.

## Document Formats

- PDF (`.pdf`, both searchable and scanned) - includes detailed [metadata extraction](metadata-extraction.md#pdf-specific-metadata)
- Microsoft Word (`.docx`)
- PowerPoint presentations (`.pptx`)
- OpenDocument Text (`.odt`)
- Rich Text Format (`.rtf`)
- EPUB (`.epub`)
- DocBook XML (`.dbk`, `.xml`)
- FictionBook (`.fb2`)
- LaTeX (`.tex`, `.latex`)
- Typst (`.typ`)

## Markup and Text Formats

- HTML (`.html`, `.htm`)
- Plain text (`.txt`) and Markdown (`.md`, `.markdown`)
- reStructuredText (`.rst`)
- Org-mode (`.org`)
- DokuWiki (`.txt`)
- Pod (`.pod`)
- Troff/Man (`.1`, `.2`, etc.)

## Data and Research Formats

- Spreadsheets (`.xlsx`, `.xls`, `.xlsm`, `.xlsb`, `.xlam`, `.xla`, `.ods`)
- CSV (`.csv`) and TSV (`.tsv`) files
- OPML files (`.opml`)
- Jupyter Notebooks (`.ipynb`)
- BibTeX (`.bib`) and BibLaTeX (`.bib`)
- CSL-JSON (`.json`)
- EndNote and JATS XML (`.xml`)
- RIS (`.ris`)

## Image Formats

- JPEG (`.jpg`, `.jpeg`, `.pjpeg`)
- PNG (`.png`)
- TIFF (`.tiff`, `.tif`)
- BMP (`.bmp`)
- GIF (`.gif`)
- JPEG 2000 family (`.jp2`, `.jpm`, `.jpx`, `.mj2`)
- WebP (`.webp`)
- Portable anymap formats (`.pbm`, `.pgm`, `.ppm`, `.pnm`)

**Image Extraction Support**: Kreuzberg can extract embedded images from the following document types:

- PDF documents (embedded images and graphics)
- PowerPoint presentations (PPTX - slide images, charts, shapes)
- HTML documents (inline images and base64-encoded images)
- Microsoft Word documents (DOCX - embedded images and charts)
- Email messages (EML, MSG - image attachments and inline images)

See the [Image Extraction guide](extraction-configuration.md#image-extraction) for configuration options.
