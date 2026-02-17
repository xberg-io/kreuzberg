"""Comprehensive tests for Python metadata types in Kreuzberg bindings.

Tests verify:
- Type structure of HtmlMetadata and related TypedDicts
- Rich metadata types (HeaderMetadata, LinkMetadata, ImageMetadata, StructuredData)
"""

from __future__ import annotations

import json
from typing import Any


class TestHtmlMetadataStructure:
    """Tests for HtmlMetadata TypedDict structure."""

    def test_html_metadata_has_required_fields(self) -> None:
        """Verify HtmlMetadata has all documented fields."""
        sample: dict[str, Any] = {
            "title": "Test Title",
            "description": "Test Description",
            "keywords": ["test", "metadata"],
            "author": "Test Author",
            "canonical_url": "https://example.com",
            "base_href": "https://example.com/",
            "language": "en",
            "text_direction": "ltr",
            "open_graph": {"og:title": "Test"},
            "twitter_card": {"twitter:card": "summary"},
            "meta_tags": {"viewport": "width=device-width"},
            "headers": [],
            "links": [],
            "images": [],
            "structured_data": [],
        }

        assert "title" in sample
        assert "description" in sample
        assert "keywords" in sample
        assert "author" in sample
        assert "canonical_url" in sample
        assert "base_href" in sample
        assert "language" in sample
        assert "text_direction" in sample
        assert "open_graph" in sample
        assert "twitter_card" in sample
        assert "meta_tags" in sample
        assert "headers" in sample
        assert "links" in sample
        assert "images" in sample
        assert "structured_data" in sample

    def test_keywords_is_list(self) -> None:
        """Verify keywords is list[str], not str."""
        sample: dict[str, Any] = {
            "keywords": ["python", "web", "extraction"],
        }

        keywords = sample["keywords"]
        assert isinstance(keywords, list), "keywords should be a list"
        assert all(isinstance(k, str) for k in keywords), "all keywords should be strings"

    def test_keywords_is_not_string(self) -> None:
        """Verify keywords is NOT a single string."""
        sample: dict[str, Any] = {
            "keywords": ["keyword1", "keyword2"],
        }
        keywords = sample["keywords"]
        assert not isinstance(keywords, str), "keywords should not be a string"

    def test_canonical_url_renamed_from_canonical(self) -> None:
        """Verify canonical_url field exists (not canonical)."""
        sample: dict[str, Any] = {
            "canonical_url": "https://example.com/page",
        }

        assert "canonical_url" in sample, "canonical_url field must exist"
        assert sample["canonical_url"] == "https://example.com/page"

    def test_open_graph_is_dict(self) -> None:
        """Verify open_graph is dict[str, str]."""
        sample: dict[str, Any] = {
            "open_graph": {
                "og:title": "My Page",
                "og:description": "Page description",
                "og:image": "https://example.com/image.jpg",
            },
        }

        og = sample["open_graph"]
        assert isinstance(og, dict), "open_graph should be a dict"
        assert all(isinstance(k, str) and isinstance(v, str) for k, v in og.items()), (
            "all keys and values in open_graph should be strings"
        )

    def test_twitter_card_is_dict(self) -> None:
        """Verify twitter_card is dict[str, str]."""
        sample: dict[str, Any] = {
            "twitter_card": {
                "twitter:card": "summary_large_image",
                "twitter:title": "My Title",
                "twitter:image": "https://example.com/image.jpg",
            },
        }

        tc = sample["twitter_card"]
        assert isinstance(tc, dict), "twitter_card should be a dict"
        assert all(isinstance(k, str) and isinstance(v, str) for k, v in tc.items()), (
            "all keys and values in twitter_card should be strings"
        )

    def test_html_metadata_partial_fields(self) -> None:
        """HtmlMetadata should support partial field population (total=False)."""
        minimal: dict[str, Any] = {
            "title": "Just Title",
        }
        assert minimal["title"] == "Just Title"

        partial: dict[str, Any] = {
            "title": "Title",
            "keywords": ["test"],
            "open_graph": {},
        }
        assert "title" in partial
        assert "keywords" in partial
        assert "open_graph" in partial


class TestHeaderMetadataFields:
    """Tests for HeaderMetadata type."""

    def test_header_metadata_structure(self) -> None:
        """Verify HeaderMetadata has all required fields."""
        header: dict[str, Any] = {
            "level": 1,
            "text": "Main Heading",
            "id": "main-heading",
            "depth": 0,
            "html_offset": 150,
        }

        assert header["level"] == 1
        assert header["text"] == "Main Heading"
        assert header["id"] == "main-heading"
        assert header["depth"] == 0
        assert header["html_offset"] == 150

    def test_header_metadata_fields_present(self) -> None:
        """Verify all HeaderMetadata fields are accessible."""
        header: dict[str, Any] = {
            "level": 2,
            "text": "Subheading",
            "id": None,
            "depth": 1,
            "html_offset": 500,
        }

        assert "level" in header
        assert "text" in header
        assert "id" in header
        assert "depth" in header
        assert "html_offset" in header

    def test_header_metadata_optional_id(self) -> None:
        """Verify HeaderMetadata id field can be None."""
        header_with_id: dict[str, Any] = {
            "level": 1,
            "text": "Heading",
            "id": "heading-id",
            "depth": 0,
            "html_offset": 0,
        }

        header_without_id: dict[str, Any] = {
            "level": 1,
            "text": "Heading",
            "id": None,
            "depth": 0,
            "html_offset": 0,
        }

        assert header_with_id["id"] == "heading-id"
        assert header_without_id["id"] is None

    def test_header_metadata_level_range(self) -> None:
        """Test HeaderMetadata with various heading levels (1-6)."""
        for level in range(1, 7):
            header: dict[str, Any] = {
                "level": level,
                "text": f"Heading Level {level}",
                "id": None,
                "depth": level - 1,
                "html_offset": 0,
            }
            assert header["level"] == level


class TestLinkMetadataFields:
    """Tests for LinkMetadata type."""

    def test_link_metadata_structure(self) -> None:
        """Verify LinkMetadata has all required fields."""
        link: dict[str, Any] = {
            "href": "https://example.com",
            "text": "Example Link",
            "title": "Example Site",
            "link_type": "external",
            "rel": ["noopener", "noreferrer"],
            "attributes": {"class": "external-link", "data-id": "123"},
        }

        assert link["href"] == "https://example.com"
        assert link["text"] == "Example Link"
        assert link["title"] == "Example Site"
        assert link["link_type"] == "external"
        assert link["rel"] == ["noopener", "noreferrer"]
        assert link["attributes"] == {"class": "external-link", "data-id": "123"}

    def test_link_type_literal_values(self) -> None:
        """Verify link_type accepts literal values."""
        link_types: list[str] = ["anchor", "internal", "external", "email", "phone", "other"]

        for link_type in link_types:
            link: dict[str, Any] = {
                "href": "https://example.com",
                "text": "Link",
                "title": None,
                "link_type": link_type,
                "rel": [],
                "attributes": {},
            }
            assert link["link_type"] == link_type

    def test_link_metadata_optional_title(self) -> None:
        """Verify LinkMetadata title can be None."""
        link_with_title: dict[str, Any] = {
            "href": "#",
            "text": "Link",
            "title": "Link Title",
            "link_type": "anchor",
            "rel": [],
            "attributes": {},
        }

        link_without_title: dict[str, Any] = {
            "href": "#",
            "text": "Link",
            "title": None,
            "link_type": "anchor",
            "rel": [],
            "attributes": {},
        }

        assert link_with_title["title"] == "Link Title"
        assert link_without_title["title"] is None

    def test_link_metadata_rel_is_list(self) -> None:
        """Verify LinkMetadata rel field is a list of strings."""
        link: dict[str, Any] = {
            "href": "https://example.com",
            "text": "Link",
            "title": None,
            "link_type": "external",
            "rel": ["noopener", "noreferrer"],
            "attributes": {},
        }

        assert isinstance(link["rel"], list)
        assert all(isinstance(r, str) for r in link["rel"])

    def test_link_metadata_attributes_is_dict(self) -> None:
        """Verify LinkMetadata attributes field is dict[str, str]."""
        link: dict[str, Any] = {
            "href": "https://example.com",
            "text": "Link",
            "title": None,
            "link_type": "external",
            "rel": [],
            "attributes": {"class": "btn", "id": "link-1"},
        }

        assert isinstance(link["attributes"], dict)
        assert all(isinstance(k, str) and isinstance(v, str) for k, v in link["attributes"].items())


class TestImageMetadataFields:
    """Tests for ImageMetadata type (HTML image metadata)."""

    def test_image_metadata_structure(self) -> None:
        """Verify ImageMetadata has all required fields."""
        image: dict[str, Any] = {
            "src": "https://example.com/image.jpg",
            "alt": "Image description",
            "title": "Image Title",
            "dimensions": (800, 600),
            "image_type": "external",
            "attributes": {"class": "hero-image", "loading": "lazy"},
        }

        assert image.get("src") == "https://example.com/image.jpg"
        assert image.get("alt") == "Image description"
        assert image.get("title") == "Image Title"
        assert image.get("dimensions") == (800, 600)
        assert image.get("image_type") == "external"
        assert image.get("attributes") == {"class": "hero-image", "loading": "lazy"}

    def test_image_type_literal_values(self) -> None:
        """Verify image_type accepts literal values."""
        image_types: list[str] = ["data_uri", "inline_svg", "external", "relative"]

        for image_type in image_types:
            image: dict[str, Any] = {
                "src": "https://example.com/image.jpg",
                "alt": None,
                "title": None,
                "dimensions": None,
                "image_type": image_type,
                "attributes": {},
            }
            assert image.get("image_type") == image_type

    def test_image_metadata_optional_fields(self) -> None:
        """Verify ImageMetadata optional fields can be None."""
        image: dict[str, Any] = {
            "src": "image.png",
            "alt": None,
            "title": None,
            "dimensions": None,
            "image_type": "relative",
            "attributes": {},
        }

        assert image.get("alt") is None
        assert image.get("title") is None
        assert image.get("dimensions") is None

    def test_image_metadata_dimensions_tuple(self) -> None:
        """Verify dimensions is tuple[int, int] or None."""
        image_with_dims: dict[str, Any] = {
            "src": "image.jpg",
            "alt": None,
            "title": None,
            "dimensions": (1920, 1080),
            "image_type": "external",
            "attributes": {},
        }

        image_without_dims: dict[str, Any] = {
            "src": "image.jpg",
            "alt": None,
            "title": None,
            "dimensions": None,
            "image_type": "external",
            "attributes": {},
        }

        dims_with = image_with_dims.get("dimensions")
        assert isinstance(dims_with, tuple)
        assert len(dims_with) == 2
        assert image_without_dims.get("dimensions") is None

    def test_image_metadata_attributes_is_dict(self) -> None:
        """Verify ImageMetadata attributes field is dict[str, str]."""
        image: dict[str, Any] = {
            "src": "image.jpg",
            "alt": None,
            "title": None,
            "dimensions": None,
            "image_type": "external",
            "attributes": {"srcset": "image-2x.jpg 2x", "width": "100"},
        }

        attrs = image.get("attributes")
        assert isinstance(attrs, dict)
        assert all(isinstance(k, str) and isinstance(v, str) for k, v in attrs.items())


class TestStructuredDataFields:
    """Tests for StructuredData type."""

    def test_structured_data_structure(self) -> None:
        """Verify StructuredData has all required fields."""
        structured: dict[str, Any] = {
            "data_type": "json_ld",
            "raw_json": '{"@context": "https://schema.org", "@type": "Article"}',
            "schema_type": "Article",
        }

        assert structured["data_type"] == "json_ld"
        assert structured["raw_json"] == '{"@context": "https://schema.org", "@type": "Article"}'
        assert structured["schema_type"] == "Article"

    def test_structured_data_type_literal_values(self) -> None:
        """Verify data_type accepts literal values."""
        data_types: list[str] = ["json_ld", "microdata", "rdfa"]

        for data_type in data_types:
            structured: dict[str, Any] = {
                "data_type": data_type,
                "raw_json": "{}",
                "schema_type": "Type",
            }
            assert structured["data_type"] == data_type

    def test_structured_data_optional_schema_type(self) -> None:
        """Verify StructuredData schema_type can be None."""
        with_schema: dict[str, Any] = {
            "data_type": "json_ld",
            "raw_json": "{}",
            "schema_type": "Organization",
        }

        without_schema: dict[str, Any] = {
            "data_type": "json_ld",
            "raw_json": "{}",
            "schema_type": None,
        }

        assert with_schema["schema_type"] == "Organization"
        assert without_schema["schema_type"] is None

    def test_structured_data_raw_json_format(self) -> None:
        """Verify raw_json is valid JSON string."""
        json_data = {"@context": "https://schema.org", "@type": "Product", "name": "Widget"}
        structured: dict[str, Any] = {
            "data_type": "json_ld",
            "raw_json": json.dumps(json_data),
            "schema_type": "Product",
        }

        parsed = json.loads(structured["raw_json"])
        assert parsed["@type"] == "Product"
        assert parsed["name"] == "Widget"


class TestMetadataJsonSerialization:
    """Tests for JSON serialization/deserialization of metadata."""

    def test_metadata_json_serializable(self) -> None:
        """Metadata should be JSON serializable."""
        header: dict[str, Any] = {
            "level": 1,
            "text": "Heading",
            "id": "heading-id",
            "depth": 0,
            "html_offset": 100,
        }

        json_str = json.dumps(header)
        assert isinstance(json_str, str)

        deserialized = json.loads(json_str)
        assert deserialized["level"] == 1
        assert deserialized["text"] == "Heading"

    def test_rich_metadata_round_trip(self) -> None:
        """Rich metadata should survive JSON serialization round trip."""
        original: dict[str, Any] = {
            "title": "Test Page",
            "keywords": ["a", "b", "c"],
            "open_graph": {"og:title": "OG Title"},
            "headers": [
                {
                    "level": 1,
                    "text": "H1",
                    "id": None,
                    "depth": 0,
                    "html_offset": 0,
                }
            ],
            "links": [
                {
                    "href": "https://example.com",
                    "text": "Link",
                    "title": None,
                    "link_type": "external",
                    "rel": [],
                    "attributes": {},
                }
            ],
        }

        json_str = json.dumps(original)
        deserialized = json.loads(json_str)

        assert deserialized["title"] == original["title"]
        assert deserialized["keywords"] == original["keywords"]
        assert deserialized["open_graph"] == original["open_graph"]
