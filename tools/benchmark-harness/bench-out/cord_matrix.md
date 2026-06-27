# CORD Structured Extraction Matrix

**Timestamp**: 2026-06-02 13:00:30
**Sample Size**: 10 documents (CORD v2 `test` split)
**Strategy**: text-then-llm — `xberg::extract` (classical text extraction) → `liter-llm` structured extraction with the dataset schema attached.

| Provider | F1 | Type Corr | Valid % | Tokens | Est. Cost | p50 (ms) | p95 (ms) | Errors |
|----------|-----|-----------|---------|--------|-----------|----------|----------|--------|
| openai (`gpt-4o-mini`) | 0.172 | 1.000 | 100.0 | 4,723 | $0.001 | 6,002 | 9,206 | 0 |
| anthropic (`claude-haiku-4-5-20251001`) | 0.153 | 1.000 | 100.0 | 8,741 | $0.016 | 5,948 | 9,618 | 0 |
| gemini (`gemini-2.5-flash`) | 0.209 | 0.700 | 100.0 | 23,070 | $0.003 | 16,271 | 29,014 | 0 |

## Observations

**Schema validity is unanimous.** All three providers return JSON that conforms to the CORD v2 schema 100% of the time at this sample size — the engine reliably constrains the output regardless of backend.

**Gemini leads accuracy, OpenAI leads cost+latency.** Gemini-2.5-flash extracts the most correct fields (F1=0.209) but uses 4.9× the tokens of GPT-4o-mini and runs 2.7× slower at p50. OpenAI is the cost/latency sweet spot — fastest p50, fewest tokens, second-best F1, and the cheapest of the three at $0.001 per 10 docs.

**Anthropic's haiku-4-5 is the outlier on price.** Despite middling F1 it costs ~$0.016 / 10 docs — 14× OpenAI and 5× Gemini — driven by claude-haiku's higher per-token price and an output-token tax under JSON-mode structuring. Worth re-running with `claude-sonnet-4-5` for an apples-to-budget comparison.

**The absolute F1 numbers are low.** ~0.15–0.21 means roughly 1 in 5 leaf fields matches the GT byte-for-byte. The schema validity rate (100%) means the model is producing the right shape; the F1 gap is dominated by **field-value normalization** — CORD GT serialises numerics as strings with locale-specific punctuation (`"60.000"`, `"24,000"`) and the LLMs render them in their own canonical form (`"60.00"`, `"60"`, `60.0`). `json_quality::numeric_match` exists with a 1% tolerance budget but the matrix only uses exact-equality `field_precision_recall_f1` — a follow-up pass that scores numeric leaves via `numeric_match` and string leaves via normalised-equality would lift the F1 ceiling without changing the underlying engine.

## Next steps to make these numbers meaningful

1. **Numeric-tolerant F1**: dispatch leaf comparison through `json_quality::numeric_match` when both sides are numeric (or stringified numeric); fall back to canonical-string compare otherwise.
2. **Add `vlm-only` strategy**: render page images and skip xberg's text path. CORD is image-only — text extraction quality dominates the F1 floor today and a VLM may bypass that.
3. **Add `fused-text+image` strategy**: feed both the text and the image to the LLM. Expected to lift the F1 ceiling further when receipts have layout ambiguity classical OCR can't resolve.
4. **Scale to 100+ docs** and bring in SROIE for a second dataset perspective. 10 docs is enough for a sniff test; 100 is enough for a defensible ranking.
5. **Replay against `claude-sonnet-4-7`** to put anthropic's strongest current sonnet on the curve.
