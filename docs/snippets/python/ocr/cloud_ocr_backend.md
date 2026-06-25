```python title="Python"
from xberg import register_ocr_backend, OcrBackendType, OcrConfig
import httpx

class CloudOcrBackend:
    def __init__(self, api_key: str):
        self.api_key: str = api_key
        self.langs: list[str] = ["eng", "deu", "fra"]

    def name(self) -> str:
        return "cloud-ocr"

    def version(self) -> str:
        return "1.0.0"

    def supported_languages(self) -> list[str]:
        return self.langs

    def supports_language(self, language: str) -> bool:
        return language in self.langs

    def backend_type(self) -> OcrBackendType:
        return OcrBackendType.CUSTOM

    def process_image(self, image_bytes: bytes, config: OcrConfig) -> dict:
        # `config` is an OcrConfig; `config.language` is a list of language codes.
        language = config.language[0] if config.language else "eng"
        with httpx.Client() as client:
            response = client.post(
                "https://api.example.com/ocr",
                files={"image": image_bytes},
                json={"language": language},
            )
            text: str = response.json()["text"]
        # The return is deserialized into an ExtractionResult; the required
        # fields are content, mime_type, metadata, and tables. Everything else
        # (chunks, images, detected_languages, …) is optional.
        return {
            "content": text,
            "mime_type": "text/plain",
            "metadata": {},
            "tables": [],
        }

    def initialize(self) -> None:
        pass

    def shutdown(self) -> None:
        pass

backend: CloudOcrBackend = CloudOcrBackend(api_key="your-api-key")
register_ocr_backend(backend)
```
