from __future__ import annotations

import json
import statistics
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Literal

from kreuzberg import extract_bytes_sync
from kreuzberg._token_reduction import get_reduction_stats, reduce_tokens
from kreuzberg._types import ExtractionConfig, TokenReductionConfig


@dataclass
class CompressionResult:
    text_type: str
    mode: str
    original_length: int
    reduced_length: int
    original_tokens: int
    reduced_tokens: int
    character_reduction_ratio: float
    token_reduction_ratio: float
    processing_time_ms: float

    @property
    def character_compression_percent(self) -> float:
        return self.character_reduction_ratio * 100

    @property
    def token_compression_percent(self) -> float:
        return self.token_reduction_ratio * 100


@dataclass
class CompressionBenchmarkSuite:
    results: list[CompressionResult] = field(default_factory=list)
    total_tests: int = 0
    total_time_ms: float = 0.0

    def add_result(self, result: CompressionResult) -> None:
        self.results.append(result)
        self.total_tests += 1
        self.total_time_ms += result.processing_time_ms

    def get_summary(self) -> dict[str, Any]:
        if not self.results:
            return {}

        by_mode: dict[str, Any] = {}
        for result in self.results:
            if result.mode not in by_mode:
                by_mode[result.mode] = []
            by_mode[result.mode].append(result)

        mode_stats = {}
        for mode, results in by_mode.items():
            char_ratios = [r.character_compression_percent for r in results]
            token_ratios = [r.token_compression_percent for r in results]
            times = [r.processing_time_ms for r in results]

            mode_stats[mode] = {
                "tests": len(results),
                "character_compression": {
                    "mean": statistics.mean(char_ratios),
                    "median": statistics.median(char_ratios),
                    "stdev": statistics.stdev(char_ratios) if len(char_ratios) > 1 else 0.0,
                    "min": min(char_ratios),
                    "max": max(char_ratios),
                },
                "token_compression": {
                    "mean": statistics.mean(token_ratios),
                    "median": statistics.median(token_ratios),
                    "stdev": statistics.stdev(token_ratios) if len(token_ratios) > 1 else 0.0,
                    "min": min(token_ratios),
                    "max": max(token_ratios),
                },
                "performance": {
                    "avg_time_ms": statistics.mean(times),
                    "total_time_ms": sum(times),
                },
            }

        return {
            "summary": {
                "total_tests": self.total_tests,
                "total_time_ms": self.total_time_ms,
                "avg_time_per_test_ms": self.total_time_ms / self.total_tests if self.total_tests > 0 else 0.0,
            },
            "by_mode": mode_stats,
            "detailed_results": [
                {
                    "text_type": r.text_type,
                    "mode": r.mode,
                    "original_length": r.original_length,
                    "reduced_length": r.reduced_length,
                    "original_tokens": r.original_tokens,
                    "reduced_tokens": r.reduced_tokens,
                    "character_compression_percent": r.character_compression_percent,
                    "token_compression_percent": r.token_compression_percent,
                    "processing_time_ms": r.processing_time_ms,
                }
                for r in self.results
            ],
        }


class TokenReductionCompressionBenchmark:
    def __init__(self) -> None:
        self.test_texts = self._create_test_texts()
        self.modes: list[Literal["light", "moderate"]] = ["light", "moderate"]

    def _create_test_texts(self) -> dict[str, str]:
        return {
            "formal_document": """
                The quarterly financial report demonstrates significant improvements in operational efficiency and market positioning.
                Our comprehensive analysis reveals that the implementation of strategic initiatives has resulted in measurable outcomes
                across multiple key performance indicators. The organization's commitment to excellence and continuous improvement
                is evident in these results. Furthermore, the systematic approach to risk management and quality assurance has
                contributed to enhanced stakeholder confidence and sustainable growth trajectory. The board of directors acknowledges
                the exceptional efforts of the management team and all employees in achieving these remarkable results.
            """.strip(),
            "casual_conversation": """
                Hey there! I was just thinking about that amazing movie we watched last weekend. It was really incredible, wasn't it?
                The way they told the story was so compelling and the characters were just wonderful. I think it's one of the best
                films I've seen this year. What did you think about it? I'd love to hear your thoughts and maybe we could discuss
                some of the themes that really stood out to me. There were so many interesting elements that I'm still thinking about.
            """.strip(),
            "technical_manual": """
                Configure the system parameters by accessing the administrative interface through the main configuration panel.
                Navigate to Settings > Advanced > Network Configuration and verify that all connection parameters are correctly
                initialized. The TCP/IP stack must be properly configured with appropriate DNS resolution settings and gateway
                routing tables. Execute the diagnostic utilities to validate network connectivity and ensure that all protocols
                are functioning within acceptable performance thresholds. Document any configuration changes in the system log
                for future reference and troubleshooting procedures.
            """.strip(),
            "news_article": """
                Local authorities announced today that the new public transportation system will begin operations next month,
                connecting several major districts across the metropolitan area. The project, which has been in development for
                over three years, represents a significant investment in sustainable urban infrastructure. City officials expect
                the system to reduce traffic congestion and provide affordable transportation options for thousands of daily
                commuters. Environmental impact studies indicate that the implementation will contribute to reduced carbon emissions
                and improved air quality throughout the region.
            """.strip(),
            "literature_excerpt": """
                The old lighthouse stood majestically against the stormy horizon, its weathered stones bearing witness to countless
                tempests and countless ships that had sought its guiding light. Sarah approached the ancient structure with a sense
                of reverence, knowing that within its walls lay the stories of generations of lighthouse keepers who had dedicated
                their lives to the safety of maritime travelers. The wind howled through the nearby cliffs, carrying with it the
                salt spray of crashing waves and the whispered secrets of the sea itself.
            """.strip(),
            "scientific_abstract": """
                This study investigates the relationship between cognitive load and working memory performance in multilingual
                individuals under various experimental conditions. Participants (n=127) completed a series of standardized
                assessments while neural activity was monitored using electroencephalography. Results indicate significant
                correlations between language switching frequency and executive control efficiency (p<0.001). The findings suggest
                that bilingual advantages in cognitive flexibility extend to domain-general executive functions, with implications
                for educational policy and cognitive training interventions.
            """.strip(),
            "stopword_heavy": """
                And so it was that he went to the store, and then he bought some things that he needed for the house. But when
                he got back to the place where he lived, he realized that he had forgotten to get the most important thing that
                he had originally planned to purchase. So he had to go back to the store again, and this time he made sure to
                get everything that he needed. It was a bit frustrating, but in the end, everything worked out just fine.
            """.strip(),
            "technical_jargon": """
                The microservices architecture implements a distributed system pattern utilizing containerized deployments
                orchestrated through Kubernetes clusters. API gateways facilitate service discovery and load balancing across
                multiple availability zones. The event-driven messaging infrastructure leverages Apache Kafka for asynchronous
                communication between bounded contexts. Monitoring and observability are achieved through OpenTelemetry
                instrumentation with Prometheus metrics collection and Grafana visualization dashboards.
            """.strip(),
            "minimal_stopwords": """
                Python programming language offers powerful features. Machine learning algorithms require extensive datasets.
                Neural networks demonstrate remarkable performance capabilities. Developers utilize frameworks like TensorFlow.
                Data preprocessing involves cleaning, transformation, validation procedures. Model training requires computational
                resources, optimization techniques. Evaluation metrics include accuracy, precision, recall measurements.
                Production deployment considerations encompass scalability, monitoring, maintenance requirements.
            """.strip(),
        }

    def test_compression_effectiveness(
        self, text: str, text_type: str, mode: Literal["light", "moderate"]
    ) -> CompressionResult:
        config = TokenReductionConfig(mode=mode, preserve_markdown=False)

        start_time = time.perf_counter()
        reduced_text = reduce_tokens(text, config=config, language="en")
        processing_time = (time.perf_counter() - start_time) * 1000

        stats = get_reduction_stats(text, reduced_text)

        return CompressionResult(
            text_type=text_type,
            mode=mode,
            original_length=len(text),
            reduced_length=len(reduced_text),
            original_tokens=stats["original_tokens"],
            reduced_tokens=stats["reduced_tokens"],
            character_reduction_ratio=stats["character_reduction_ratio"],
            token_reduction_ratio=stats["token_reduction_ratio"],
            processing_time_ms=processing_time,
        )

    def run_comprehensive_benchmark(self) -> CompressionBenchmarkSuite:
        suite = CompressionBenchmarkSuite()

        for text_type, text in self.test_texts.items():
            for mode in self.modes:
                result = self.test_compression_effectiveness(text, text_type, mode)
                suite.add_result(result)

        return suite

    def run_pipeline_integration_test(self) -> dict[str, Any]:
        pipeline_results: dict[str, Any] = {}

        for mode in self.modes:
            config = ExtractionConfig(token_reduction=TokenReductionConfig(mode=mode))

            test_text = self.test_texts["formal_document"]

            start_time = time.perf_counter()
            result = extract_bytes_sync(test_text.encode("utf-8"), "text/plain", config)
            processing_time = (time.perf_counter() - start_time) * 1000

            reduction_stats = result.metadata.get("token_reduction", {})

            pipeline_results[mode] = {
                "original_length": len(test_text),
                "reduced_length": len(result.content),
                "processing_time_ms": processing_time,
                "reduction_stats": reduction_stats,
                "metadata_present": "token_reduction" in result.metadata,
            }

        return pipeline_results


def main() -> None:
    benchmark = TokenReductionCompressionBenchmark()

    suite = benchmark.run_comprehensive_benchmark()

    pipeline_results = benchmark.run_pipeline_integration_test()

    summary = suite.get_summary()

    for _stats in summary["by_mode"].values():
        pass

    output_dir = Path("benchmarks/results")
    output_dir.mkdir(exist_ok=True)

    full_results = {
        "compression_benchmark": summary,
        "pipeline_integration": pipeline_results,
        "timestamp": time.time(),
    }

    output_file = output_dir / "token_reduction_compression.json"
    with output_file.open("w") as f:
        json.dump(full_results, f, indent=2)


if __name__ == "__main__":
    main()
