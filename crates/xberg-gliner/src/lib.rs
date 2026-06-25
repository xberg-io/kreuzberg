//! Span-mode GLiNER ONNX inference.
//!
//! This crate vendors the span-mode preprocessing and decoding path from the
//! `gline-rs` project and replaces its ORP pipeline wrapper with direct `ort`
//! session management.

use std::collections::HashSet;
use std::path::Path;

use ndarray::{Array2, Array3, ArrayViewD, Ix4};
use ort::session::Session;
use ort::session::builder::GraphOptimizationLevel;
use ort::value::Tensor;
use parking_lot::Mutex;
use regex::Regex;
use thiserror::Error;

/// GLiNER tensor input names expected by span-mode ONNX exports.
pub const INPUT_NAMES: [&str; 6] = [
    "input_ids",
    "attention_mask",
    "words_mask",
    "text_lengths",
    "span_idx",
    "span_mask",
];

/// GLiNER tensor output names expected by span-mode ONNX exports.
pub const OUTPUT_NAMES: [&str; 1] = ["logits"];

const TENSOR_INPUT_IDS: &str = "input_ids";
const TENSOR_ATTENTION_MASK: &str = "attention_mask";
const TENSOR_WORD_MASK: &str = "words_mask";
const TENSOR_TEXT_LENGTHS: &str = "text_lengths";
const TENSOR_SPAN_IDX: &str = "span_idx";
const TENSOR_SPAN_MASK: &str = "span_mask";
const TENSOR_LOGITS: &str = "logits";

/// Result type used by `xberg-gliner`.
pub type Result<T> = std::result::Result<T, GlinerError>;

/// Errors returned by GLiNER preprocessing, inference, and decoding.
#[derive(Debug, Error)]
pub enum GlinerError {
    /// Input text or label data is invalid.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// Tokenizer loading or encoding failed.
    #[error("tokenizer error: {0}")]
    Tokenizer(String),
    /// Regex splitter construction failed.
    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),
    /// ONNX Runtime failed.
    #[error("onnx runtime error: {0}")]
    Ort(#[from] ort::Error),
    /// An expected tensor was missing from model output.
    #[error("missing model output tensor '{0}'")]
    MissingOutput(&'static str),
    /// The loaded model does not expose the expected input or output names.
    #[error("unexpected model {kind} schema: expected {expected:?}, got {actual:?}")]
    UnexpectedModelSchema {
        /// Schema side being validated.
        kind: &'static str,
        /// Required names.
        expected: Vec<&'static str>,
        /// Actual names exposed by the model.
        actual: Vec<String>,
    },
    /// The logits tensor shape did not match the span-mode decoder contract.
    #[error("unexpected logits shape: expected {expected:?}, got {actual:?}")]
    UnexpectedLogitsShape {
        /// Expected dimensions.
        expected: Vec<usize>,
        /// Actual dimensions.
        actual: Vec<usize>,
    },
    /// Internal metadata referred to a missing item.
    #[error("index error: {target}[{index}] is missing")]
    Index {
        /// Indexed collection name.
        target: &'static str,
        /// Missing index.
        index: usize,
    },
    /// Source text offsets were not valid UTF-8 boundaries.
    #[error("invalid source text offsets {start}..{end}")]
    InvalidOffsets {
        /// Start byte offset.
        start: usize,
        /// End byte offset.
        end: usize,
    },
}

/// Processing parameters for GLiNER span-mode inference.
#[derive(Debug, Clone)]
pub struct Parameters {
    /// Probability threshold. Defaults to `0.5`.
    pub threshold: f32,
    /// No entity may overlap another entity when enabled. Defaults to `true`.
    pub flat_ner: bool,
    /// Overlapping spans may share the same label when enabled and `flat_ner` is disabled.
    pub dup_label: bool,
    /// Overlapping spans may use different labels when enabled and `flat_ner` is disabled.
    pub multi_label: bool,
    /// Maximum span width in words. Defaults to `12`.
    pub max_width: usize,
    /// Maximum number of words per input sequence. Defaults to `512`.
    pub max_length: Option<usize>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            threshold: 0.5,
            flat_ner: true,
            dup_label: false,
            multi_label: false,
            max_width: 12,
            max_length: Some(512),
        }
    }
}

impl Parameters {
    /// Set the probability threshold.
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Set the maximum span width.
    pub fn with_max_width(mut self, max_width: usize) -> Self {
        self.max_width = max_width;
        self
    }

    /// Set the maximum input length.
    pub fn with_max_length(mut self, max_length: Option<usize>) -> Self {
        self.max_length = max_length;
        self
    }

    /// Configure flat NER overlap filtering.
    pub fn with_flat_ner(mut self, flat_ner: bool) -> Self {
        self.flat_ner = flat_ner;
        self
    }

    /// Configure duplicate-label overlap filtering.
    pub fn with_dup_label(mut self, dup_label: bool) -> Self {
        self.dup_label = dup_label;
        self
    }

    /// Configure multi-label overlap filtering.
    pub fn with_multi_label(mut self, multi_label: bool) -> Self {
        self.multi_label = multi_label;
        self
    }
}

/// ONNX Runtime session configuration.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Intra-op thread count passed to ONNX Runtime.
    pub intra_threads: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self { intra_threads: 4 }
    }
}

impl RuntimeConfig {
    /// Set the ONNX Runtime intra-op thread count.
    pub fn with_intra_threads(mut self, intra_threads: usize) -> Self {
        self.intra_threads = intra_threads.max(1);
        self
    }
}

/// Raw text input and zero-shot entity labels.
#[derive(Debug, Clone)]
pub struct TextInput {
    texts: Vec<String>,
    entities: Vec<String>,
}

impl TextInput {
    /// Construct a text batch.
    pub fn new(texts: Vec<String>, entities: Vec<String>) -> Result<Self> {
        if texts.is_empty() {
            return Err(GlinerError::InvalidInput("texts must not be empty".to_string()));
        }
        if entities.is_empty() {
            return Err(GlinerError::InvalidInput("entity labels must not be empty".to_string()));
        }
        if let Some(index) = texts.iter().position(|text| text.trim().is_empty()) {
            return Err(GlinerError::InvalidInput(format!("texts[{index}] must not be empty")));
        }
        if let Some(index) = entities.iter().position(|label| label.trim().is_empty()) {
            return Err(GlinerError::InvalidInput(format!(
                "entity labels[{index}] must not be empty"
            )));
        }
        Ok(Self { texts, entities })
    }

    /// Construct a text batch from borrowed string slices.
    pub fn from_str(texts: &[&str], entities: &[&str]) -> Result<Self> {
        Self::new(
            texts.iter().map(|text| (*text).to_string()).collect(),
            entities.iter().map(|entity| (*entity).to_string()).collect(),
        )
    }
}

/// A word token with byte offsets in the source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    start: usize,
    end: usize,
    text: String,
}

impl Token {
    fn new(start: usize, end: usize, text: &str) -> Self {
        Self {
            start,
            end,
            text: text.to_string(),
        }
    }

    /// Start byte offset.
    pub fn start(&self) -> usize {
        self.start
    }

    /// End byte offset.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Token text.
    pub fn text(&self) -> &str {
        &self.text
    }
}

trait Splitter {
    fn split(&self, input: &str, limit: Option<usize>) -> Result<Vec<Token>>;
}

struct RegexSplitter {
    regex: Regex,
}

impl RegexSplitter {
    fn new(regex: &str) -> Result<Self> {
        Ok(Self {
            regex: Regex::new(regex)?,
        })
    }
}

impl Default for RegexSplitter {
    fn default() -> Self {
        const DEFAULT_REGEX: &str = r"\w+(?:[-_]\w+)*|\S";
        Self::new(DEFAULT_REGEX).expect("default GLiNER regex is valid")
    }
}

impl Splitter for RegexSplitter {
    fn split(&self, input: &str, limit: Option<usize>) -> Result<Vec<Token>> {
        let mut result = Vec::new();
        for match_ in self.regex.find_iter(input) {
            result.push(Token::new(match_.start(), match_.end(), match_.as_str()));
            if limit.is_some_and(|limit| result.len() >= limit) {
                break;
            }
        }
        Ok(result)
    }
}

trait Tokenizer {
    fn encode(&self, input: &str) -> Result<Vec<u32>>;
}

struct HFTokenizer {
    inner: tokenizers::Tokenizer,
}

impl HFTokenizer {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let inner = tokenizers::Tokenizer::from_file(path).map_err(|error| {
            GlinerError::Tokenizer(format!("failed to load tokenizer from file: {error}"))
        })?;
        Ok(Self { inner })
    }
}

impl Tokenizer for HFTokenizer {
    fn encode(&self, input: &str) -> Result<Vec<u32>> {
        let encoding = self
            .inner
            .encode(input, false)
            .map_err(|error| GlinerError::Tokenizer(format!("failed to encode '{input}': {error}")))?;
        Ok(encoding.get_ids().to_vec())
    }
}

#[derive(Debug)]
struct Prompt {
    tokens: Vec<String>,
    entities_length: usize,
}

impl Prompt {
    fn new(tokens: Vec<String>, entities_length: usize) -> Self {
        Self {
            tokens,
            entities_length,
        }
    }
}

struct TokenizedInput {
    tokens: Vec<Vec<Token>>,
    texts: Vec<String>,
    entities: Vec<String>,
}

impl TokenizedInput {
    fn from(input: TextInput, splitter: &impl Splitter, max_length: Option<usize>) -> Result<Self> {
        let tokens = input
            .texts
            .iter()
            .map(|text| splitter.split(text, max_length))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            tokens,
            texts: input.texts,
            entities: input.entities,
        })
    }
}

struct PromptInput {
    texts: Vec<String>,
    tokens: Vec<Vec<Token>>,
    entities: Vec<String>,
    text_lengths: Vec<usize>,
    num_words: usize,
    prompts: Vec<Prompt>,
}

impl PromptInput {
    fn from(input: TokenizedInput) -> Self {
        let entities_prompt = Self::entities_prompt(&input.entities);
        let mut text_lengths = Vec::with_capacity(input.tokens.len());
        let mut num_words = 0usize;
        let mut prompts = Vec::with_capacity(input.tokens.len());

        for tokens in &input.tokens {
            let mut prompt = Vec::with_capacity(entities_prompt.len() + tokens.len());
            prompt.extend(entities_prompt.clone());
            prompt.extend(tokens.iter().map(|token| token.text().to_string()));
            prompts.push(Prompt::new(prompt, entities_prompt.len()));
            text_lengths.push(tokens.len());
            num_words = num_words.max(tokens.len());
        }

        Self {
            texts: input.texts,
            tokens: input.tokens,
            entities: input.entities,
            text_lengths,
            num_words,
            prompts,
        }
    }

    fn entities_prompt(entities: &[String]) -> Vec<String> {
        const ENTITY_TOKEN: &str = "<<ENT>>";
        const SEP_TOKEN: &str = "<<SEP>>";

        let mut result = Vec::with_capacity(entities.len() * 2 + 1);
        for entity in entities {
            result.push(ENTITY_TOKEN.to_string());
            result.push(entity.clone());
        }
        result.push(SEP_TOKEN.to_string());
        result
    }
}

struct EncodedPrompt {
    encoding: Vec<Vec<u32>>,
    text_offset: usize,
}

struct EncodedInput {
    texts: Vec<String>,
    tokens: Vec<Vec<Token>>,
    entities: Vec<String>,
    num_words: usize,
    input_ids: Array2<i64>,
    attention_masks: Array2<i64>,
    word_masks: Array2<i64>,
    text_lengths: Array2<i64>,
}

impl EncodedInput {
    fn from(input: PromptInput, tokenizer: &impl Tokenizer) -> Result<Self> {
        let mut encodings = Vec::with_capacity(input.prompts.len());
        let mut max_tokens = 0usize;

        for prompt in &input.prompts {
            let mut prompt_tokens = Vec::with_capacity(prompt.tokens.len());
            let mut total_tokens = 2usize;
            let mut total_entity_tokens = 0usize;

            for (position, word) in prompt.tokens.iter().enumerate() {
                let encoding = tokenizer.encode(word)?;
                total_tokens += encoding.len();
                if position < prompt.entities_length {
                    total_entity_tokens += encoding.len();
                }
                prompt_tokens.push(encoding);
            }

            encodings.push(EncodedPrompt {
                encoding: prompt_tokens,
                text_offset: total_entity_tokens + 1,
            });
            max_tokens = max_tokens.max(total_tokens);
        }

        let batch_size = encodings.len();
        let mut input_ids = Array2::<i64>::zeros((batch_size, max_tokens));
        let mut attention_masks = Array2::<i64>::zeros((batch_size, max_tokens));
        let mut word_masks = Array2::<i64>::zeros((batch_size, max_tokens));

        for (row, encoded_prompt) in encodings.into_iter().enumerate() {
            let mut index = 0usize;
            let mut word_id = 0i64;

            input_ids[[row, index]] = 1;
            attention_masks[[row, index]] = 1;
            index += 1;

            for word in encoded_prompt.encoding {
                for (token_index, token) in word.iter().enumerate() {
                    input_ids[[row, index]] = i64::from(*token);
                    attention_masks[[row, index]] = 1;
                    if index >= encoded_prompt.text_offset && token_index == 0 {
                        word_masks[[row, index]] = word_id;
                    }
                    index += 1;
                }
                if index >= encoded_prompt.text_offset {
                    word_id += 1;
                }
            }

            input_ids[[row, index]] = 2;
            attention_masks[[row, index]] = 1;
        }

        let mut text_lengths = Array2::<i64>::zeros((input.text_lengths.len(), 1));
        for (row, text_length) in input.text_lengths.into_iter().enumerate() {
            text_lengths[[row, 0]] = text_length as i64;
        }

        Ok(Self {
            texts: input.texts,
            tokens: input.tokens,
            entities: input.entities,
            num_words: input.num_words,
            input_ids,
            attention_masks,
            word_masks,
            text_lengths,
        })
    }
}

struct EntityContext {
    texts: Vec<String>,
    tokens: Vec<Vec<Token>>,
    entities: Vec<String>,
    num_words: usize,
}

impl EntityContext {
    fn create_span(
        &self,
        sequence_id: usize,
        start_token: usize,
        end_token: usize,
        class: usize,
        probability: f32,
    ) -> Result<Span> {
        let sequence = self.tokens.get(sequence_id).ok_or(GlinerError::Index {
            target: "tokens",
            index: sequence_id,
        })?;
        let start_token = sequence.get(start_token).ok_or(GlinerError::Index {
            target: "tokens[]",
            index: start_token,
        })?;
        let end_token = sequence.get(end_token).ok_or(GlinerError::Index {
            target: "tokens[]",
            index: end_token,
        })?;
        let text = self.texts.get(sequence_id).ok_or(GlinerError::Index {
            target: "texts",
            index: sequence_id,
        })?;
        let source = text
            .get(start_token.start()..end_token.end())
            .ok_or(GlinerError::InvalidOffsets {
                start: start_token.start(),
                end: end_token.end(),
            })?
            .to_string();
        let class = self
            .entities
            .get(class)
            .ok_or(GlinerError::Index {
                target: "entities",
                index: class,
            })?
            .to_string();
        Span::new(
            sequence_id,
            start_token.start(),
            end_token.end(),
            source,
            class,
            probability,
        )
    }
}

/// A decoded entity span.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    sequence: usize,
    start: usize,
    end: usize,
    text: String,
    class: String,
    probability: f32,
}

impl Span {
    fn new(
        sequence: usize,
        start: usize,
        end: usize,
        text: String,
        class: String,
        probability: f32,
    ) -> Result<Self> {
        if end <= start {
            return Err(GlinerError::InvalidOffsets { start, end });
        }
        Ok(Self {
            sequence,
            start,
            end,
            text,
            class,
            probability,
        })
    }

    /// Input sequence index in the batch.
    pub fn sequence(&self) -> usize {
        self.sequence
    }

    /// Start and end byte offsets in the source text.
    pub fn offsets(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    /// Matched entity text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Entity label.
    pub fn class(&self) -> &str {
        &self.class
    }

    /// Entity probability.
    pub fn probability(&self) -> f32 {
        self.probability
    }

    fn overlaps(&self, other: &Span) -> bool {
        self.start < other.end && other.start < self.end
    }

    fn is_disjoint(&self, other: &Span) -> bool {
        !self.overlaps(other)
    }
}

/// Final GLiNER span-mode output.
#[derive(Debug, Clone)]
pub struct SpanOutput {
    /// Original text batch.
    pub texts: Vec<String>,
    /// Entity labels used for inference.
    pub entities: Vec<String>,
    /// Decoded spans per input sequence.
    pub spans: Vec<Vec<Span>>,
}

impl SpanOutput {
    fn new(texts: Vec<String>, entities: Vec<String>, spans: Vec<Vec<Span>>) -> Self {
        Self {
            texts,
            entities,
            spans,
        }
    }
}

impl std::fmt::Display for SpanOutput {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spans in &self.spans {
            for span in spans {
                writeln!(
                    formatter,
                    "{:3} | {:15} | {:10} | {:.1}%",
                    span.sequence(),
                    span.text(),
                    span.class(),
                    span.probability() * 100.0,
                )?;
            }
        }
        Ok(())
    }
}

struct SpanTensors {
    input_ids: Array2<i64>,
    attention_masks: Array2<i64>,
    word_masks: Array2<i64>,
    text_lengths: Array2<i64>,
    span_idx: Array3<i64>,
    span_mask: Array2<bool>,
    context: EntityContext,
}

impl SpanTensors {
    fn from(encoded: EncodedInput, max_width: usize) -> Self {
        let (span_idx, span_mask) = make_span_tensors(&encoded, max_width);
        Self {
            input_ids: encoded.input_ids,
            attention_masks: encoded.attention_masks,
            word_masks: encoded.word_masks,
            text_lengths: encoded.text_lengths,
            span_idx,
            span_mask,
            context: EntityContext {
                texts: encoded.texts,
                tokens: encoded.tokens,
                entities: encoded.entities,
                num_words: encoded.num_words,
            },
        }
    }
}

fn make_span_tensors(encoded: &EncodedInput, max_width: usize) -> (Array3<i64>, Array2<bool>) {
    let num_spans = encoded.num_words * max_width;
    let mut span_idx = Array3::<i64>::zeros((encoded.texts.len(), num_spans, 2));
    let mut span_mask = Array2::<bool>::from_elem((encoded.texts.len(), num_spans), false);

    for sequence in 0..encoded.texts.len() {
        let text_width = encoded.text_lengths[[sequence, 0]] as usize;
        for start in 0..text_width {
            let actual_max_width = max_width.min(text_width - start);
            for width in 0..actual_max_width {
                let dimension = start * max_width + width;
                span_idx[[sequence, dimension, 0]] = start as i64;
                span_idx[[sequence, dimension, 1]] = (start + width) as i64;
                span_mask[[sequence, dimension]] = true;
            }
        }
    }

    (span_idx, span_mask)
}

/// GLiNER span-mode inference engine.
pub struct Gliner {
    params: Parameters,
    splitter: RegexSplitter,
    tokenizer: HFTokenizer,
    session: Mutex<Session>,
}

impl Gliner {
    /// Load a GLiNER span-mode ONNX model and tokenizer from local files.
    pub fn new<P: AsRef<Path>>(params: Parameters, tokenizer_path: P, model_path: P) -> Result<Self> {
        Self::with_runtime(params, RuntimeConfig::default(), tokenizer_path, model_path)
    }

    /// Load a GLiNER span-mode ONNX model and tokenizer from local files with runtime options.
    pub fn with_runtime<P: AsRef<Path>>(
        params: Parameters,
        runtime: RuntimeConfig,
        tokenizer_path: P,
        model_path: P,
    ) -> Result<Self> {
        let tokenizer = HFTokenizer::from_file(tokenizer_path)?;
        let session = build_session(model_path, &runtime)?;
        validate_session_schema(&session)?;
        Ok(Self {
            params,
            splitter: RegexSplitter::default(),
            tokenizer,
            session: Mutex::new(session),
        })
    }

    /// Run span-mode inference.
    pub fn inference(&self, input: TextInput) -> Result<SpanOutput> {
        let tokenized = TokenizedInput::from(input, &self.splitter, self.params.max_length)?;
        let prompt = PromptInput::from(tokenized);
        let encoded = EncodedInput::from(prompt, &self.tokenizer)?;
        let tensors = SpanTensors::from(encoded, self.params.max_width);
        let context = tensors.context;

        let input_ids = Tensor::from_array(tensors.input_ids)?;
        let attention_masks = Tensor::from_array(tensors.attention_masks)?;
        let word_masks = Tensor::from_array(tensors.word_masks)?;
        let text_lengths = Tensor::from_array(tensors.text_lengths)?;
        let span_idx = Tensor::from_array(tensors.span_idx)?;
        let span_mask = Tensor::from_array(tensors.span_mask)?;

        let mut session = self.session.lock();
        let outputs = session.run(ort::inputs![
            TENSOR_INPUT_IDS => input_ids,
            TENSOR_ATTENTION_MASK => attention_masks,
            TENSOR_WORD_MASK => word_masks,
            TENSOR_TEXT_LENGTHS => text_lengths,
            TENSOR_SPAN_IDX => span_idx,
            TENSOR_SPAN_MASK => span_mask,
        ])?;
        let logits = outputs
            .get(TENSOR_LOGITS)
            .ok_or(GlinerError::MissingOutput(TENSOR_LOGITS))?
            .try_extract_array::<f32>()?;

        decode_logits(
            logits.view(),
            context,
            self.params.threshold,
            self.params.max_width,
            self.params.flat_ner,
            self.params.dup_label,
            self.params.multi_label,
        )
    }
}

fn build_session<P: AsRef<Path>>(model_path: P, runtime: &RuntimeConfig) -> Result<Session> {
    // ort 2.0.0-rc.12 builder option methods return `ort::Error<SessionBuilder>`; flatten to the
    // plain `ort::Error` the `Ort` variant accepts (matches the other ORT session sites in xberg).
    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::All)
        .map_err(|e| ort::Error::new(e.message()))?
        .with_intra_threads(runtime.intra_threads)
        .map_err(|e| ort::Error::new(e.message()))?
        .commit_from_file(model_path)?;
    Ok(session)
}

fn validate_session_schema(session: &Session) -> Result<()> {
    let inputs = session
        .inputs()
        .iter()
        .map(|input| input.name().to_string())
        .collect::<Vec<_>>();
    validate_schema_names("input", &INPUT_NAMES, &inputs)?;

    let outputs = session
        .outputs()
        .iter()
        .map(|output| output.name().to_string())
        .collect::<Vec<_>>();
    validate_schema_names("output", &OUTPUT_NAMES, &outputs)
}

fn validate_schema_names(kind: &'static str, expected: &[&'static str], actual: &[String]) -> Result<()> {
    let actual = actual.iter().map(String::as_str).collect::<HashSet<_>>();
    if expected.iter().all(|name| actual.contains(name)) {
        return Ok(());
    }
    Err(GlinerError::UnexpectedModelSchema {
        kind,
        expected: expected.to_vec(),
        actual: actual.into_iter().map(str::to_string).collect(),
    })
}

fn decode_logits(
    logits: ArrayViewD<'_, f32>,
    context: EntityContext,
    threshold: f32,
    max_width: usize,
    flat_ner: bool,
    dup_label: bool,
    multi_label: bool,
) -> Result<SpanOutput> {
    let expected_shape = vec![
        context.texts.len(),
        context.num_words,
        max_width,
        context.entities.len(),
    ];
    let actual_shape = logits.shape().to_vec();
    if actual_shape != expected_shape {
        return Err(GlinerError::UnexpectedLogitsShape {
            expected: expected_shape,
            actual: actual_shape,
        });
    }

    let logits = logits
        .into_dimensionality::<Ix4>()
        .expect("shape was already checked as four-dimensional");
    let mut decoded = Vec::with_capacity(context.texts.len());

    for sequence_id in 0..context.texts.len() {
        let sequence = logits.slice(ndarray::s![sequence_id, .., .., ..]);
        let num_tokens = context
            .tokens
            .get(sequence_id)
            .ok_or(GlinerError::Index {
                target: "tokens",
                index: sequence_id,
            })?
            .len();
        let mut spans = Vec::new();

        for ((start, width, class), score) in sequence.indexed_iter() {
            if start >= num_tokens || start + width >= num_tokens {
                continue;
            }
            let probability = sigmoid(*score);
            if probability >= threshold {
                spans.push(context.create_span(sequence_id, start, start + width, class, probability)?);
            }
        }

        spans.sort_unstable_by_key(Span::offsets);
        decoded.push(greedy_search(&spans, flat_ner, dup_label, multi_label));
    }

    Ok(SpanOutput::new(context.texts, context.entities, decoded))
}

fn greedy_search(spans: &[Span], flat_ner: bool, dup_label: bool, multi_label: bool) -> Vec<Span> {
    if spans.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(spans.len());
    let mut previous = 0usize;
    let mut next = 1usize;

    while next < spans.len() {
        let previous_span = &spans[previous];
        let next_span = &spans[next];
        if accept_span(previous_span, next_span, flat_ner, dup_label, multi_label) {
            result.push(previous_span.clone());
            previous = next;
        } else if previous_span.probability() < next_span.probability() {
            previous = next;
        }
        next += 1;
    }

    result.push(spans[previous].clone());
    result
}

fn accept_span(first: &Span, second: &Span, flat_ner: bool, dup_label: bool, multi_label: bool) -> bool {
    if first.is_disjoint(second) {
        true
    } else if flat_ner {
        false
    } else if first.class() == second.class() {
        dup_label
    } else {
        multi_label
    }
}

fn sigmoid(value: f32) -> f32 {
    1.0 / (1.0 + (-value).exp())
}

#[cfg(test)]
mod tests {
    use ndarray::Array4;

    use super::*;

    struct FakeTokenizer;

    impl Tokenizer for FakeTokenizer {
        fn encode(&self, input: &str) -> Result<Vec<u32>> {
            let tokens = match input {
                "<<ENT>>" => vec![128_002],
                "<<SEP>>" => vec![128_003],
                "movie character" => vec![1421, 1470],
                "vehicle" => vec![1508],
                "1a" => vec![16, 64],
                other => vec![stable_token_id(other)],
            };
            Ok(tokens)
        }
    }

    fn stable_token_id(input: &str) -> u32 {
        input.bytes().fold(100u32, |accumulator, byte| {
            accumulator.wrapping_add(u32::from(byte))
        })
    }

    fn fake_encoded(texts: &[&str], entities: &[&str]) -> EncodedInput {
        let input = TextInput::from_str(texts, entities).expect("valid input");
        let tokenized = TokenizedInput::from(input, &RegexSplitter::default(), None).expect("tokenized");
        let prompt = PromptInput::from(tokenized);
        EncodedInput::from(prompt, &FakeTokenizer).expect("encoded")
    }

    #[test]
    fn rejects_empty_input() {
        assert!(TextInput::new(Vec::new(), vec!["person".to_string()]).is_err());
        assert!(TextInput::new(vec!["Ada".to_string()], Vec::new()).is_err());
        assert!(TextInput::new(vec!["  ".to_string()], vec!["person".to_string()]).is_err());
        assert!(TextInput::new(vec!["Ada".to_string()], vec!["".to_string()]).is_err());
    }

    #[test]
    fn regex_splitter_handles_default_unicode_and_limit() {
        let splitter = RegexSplitter::default();
        let tokens = splitter.split("This is an oh-yeah test", None).expect("split");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[3].text(), "oh-yeah");
        assert_eq!(tokens[3].start(), 11);
        assert_eq!(tokens[3].end(), 18);

        let unicode = splitter
            .split("Word with accents: éàèèçîù foo bar", None)
            .expect("unicode split");
        assert_eq!(unicode.len(), 7);

        let limited = splitter
            .split("w1 w2 w3 w4 w5 w6 w7", Some(5))
            .expect("limited split");
        assert_eq!(limited.len(), 5);
        assert_eq!(limited[4].text(), "w5");
    }

    #[test]
    fn prompt_builder_uses_gliner_marker_layout() {
        let input = TextInput::from_str(&["This is a text !", "This is a longer one."], &["Person", "Place"])
            .expect("valid input");
        let tokenized = TokenizedInput::from(input, &RegexSplitter::default(), None).expect("tokenized");
        let prepared = PromptInput::from(tokenized);

        assert_eq!(prepared.prompts.len(), 2);
        assert_eq!(prepared.prompts[0].tokens.len(), 10);
        assert_eq!(prepared.prompts[1].tokens.len(), 11);
        assert_eq!(prepared.text_lengths[0], 5);
        assert_eq!(prepared.text_lengths[1], 6);
        assert_eq!(prepared.prompts[0].tokens[0], "<<ENT>>");
        assert_eq!(prepared.prompts[0].tokens[1], "Person");
        assert_eq!(prepared.prompts[0].tokens[2], "<<ENT>>");
        assert_eq!(prepared.prompts[0].tokens[3], "Place");
        assert_eq!(prepared.prompts[0].tokens[4], "<<SEP>>");
        assert_eq!(prepared.prompts[1].tokens[5], "This");
        assert_eq!(prepared.num_words, 6);
    }

    #[test]
    fn build_session_surfaces_missing_model_as_error() {
        // Exercises the ort 2.0.0-rc.12 `SessionBuilder` option chain (the part the
        // rc.12 bump broke) and confirms a missing model file surfaces as a
        // `GlinerError` instead of panicking.
        let err = build_session("/nonexistent/gliner-model.onnx", &RuntimeConfig::default())
            .expect_err("missing model file must error");
        assert!(matches!(err, GlinerError::Ort(_)), "expected Ort error, got {err:?}");
    }

    #[test]
    fn encoding_builds_expected_masks_for_multi_token_words() {
        let encoded = fake_encoded(&["1a John Doe"], &["movie character", "vehicle"]);

        assert_eq!(encoded.input_ids.shape(), &[1, 12]);
        assert_eq!(
            encoded.input_ids.row(0).to_vec(),
            vec![1, 128_002, 1421, 1470, 128_002, 1508, 128_003, 16, 64, 499, 380, 2]
        );
        assert_eq!(
            encoded.attention_masks.row(0).to_vec(),
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            encoded.word_masks.row(0).to_vec(),
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 3, 0]
        );
        assert_eq!(encoded.text_lengths.row(0).to_vec(), vec![3]);
    }

    #[test]
    fn span_tensors_include_valid_span_indices_and_masks() {
        let encoded = fake_encoded(&["My name is Bond"], &["person"]);
        let (span_idx, span_mask) = make_span_tensors(&encoded, 3);

        assert_eq!(span_idx.shape(), &[1, 12, 2]);
        assert_eq!(span_mask.shape(), &[1, 12]);
        assert_eq!(span_idx[[0, 0, 0]], 0);
        assert_eq!(span_idx[[0, 0, 1]], 0);
        assert!(span_mask[[0, 0]]);
        assert_eq!(span_idx[[0, 1, 0]], 0);
        assert_eq!(span_idx[[0, 1, 1]], 1);
        assert!(span_mask[[0, 1]]);
        // Last valid span for a 4-word text is the single-word span (3, 3), at
        // dimension `start * max_width + width` = 3 * 3 + 0 = 9.
        assert_eq!(span_idx[[0, 9, 0]], 3);
        assert_eq!(span_idx[[0, 9, 1]], 3);
        assert!(span_mask[[0, 9]]);
        // Dimension 11 (start 3, width 2 -> span end 5) runs past the text, so it
        // stays zeroed and masked out.
        assert_eq!(span_idx[[0, 11, 0]], 0);
        assert_eq!(span_idx[[0, 11, 1]], 0);
        assert!(!span_mask[[0, 11]]);
    }

    #[test]
    fn decode_logits_returns_spans_above_threshold() {
        let encoded = fake_encoded(&["Ada works at Xberg"], &["person", "organization"]);
        let context = EntityContext {
            texts: encoded.texts,
            tokens: encoded.tokens,
            entities: encoded.entities,
            num_words: encoded.num_words,
        };
        let mut logits = Array4::<f32>::from_elem((1, 4, 2, 2), -10.0);
        logits[[0, 0, 0, 0]] = 10.0;
        logits[[0, 3, 0, 1]] = 9.0;

        let output = decode_logits(logits.view().into_dyn(), context, 0.5, 2, true, false, false)
            .expect("decoded");

        assert_eq!(output.spans[0].len(), 2);
        assert_eq!(output.spans[0][0].text(), "Ada");
        assert_eq!(output.spans[0][0].class(), "person");
        assert_eq!(output.spans[0][1].text(), "Xberg");
        assert_eq!(output.spans[0][1].class(), "organization");
    }

    #[test]
    fn decode_logits_rejects_unexpected_shape() {
        let encoded = fake_encoded(&["Ada"], &["person"]);
        let context = EntityContext {
            texts: encoded.texts,
            tokens: encoded.tokens,
            entities: encoded.entities,
            num_words: encoded.num_words,
        };
        let logits = Array4::<f32>::zeros((1, 2, 2, 1));
        let error = decode_logits(logits.view().into_dyn(), context, 0.5, 2, true, false, false)
            .expect_err("shape mismatch");

        assert!(matches!(error, GlinerError::UnexpectedLogitsShape { .. }));
    }

    #[test]
    fn greedy_search_keeps_adjacent_spans_and_filters_overlaps() {
        let spans = vec![
            Span::new(0, 0, 3, "Ada".to_string(), "person".to_string(), 0.9).expect("span"),
            Span::new(0, 3, 8, "Xberg".to_string(), "organization".to_string(), 0.8).expect("span"),
            Span::new(0, 0, 8, "Ada Xberg".to_string(), "organization".to_string(), 0.7).expect("span"),
        ];

        let selected = greedy_search(&spans, true, false, false);
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].text(), "Ada");
        assert_eq!(selected[1].text(), "Xberg");
    }

    #[test]
    fn validates_model_schema_names() {
        let inputs = INPUT_NAMES.map(str::to_string);
        let outputs = OUTPUT_NAMES.map(str::to_string);

        validate_schema_names("input", &INPUT_NAMES, &inputs).expect("valid inputs");
        validate_schema_names("output", &OUTPUT_NAMES, &outputs).expect("valid outputs");

        let error = validate_schema_names("input", &INPUT_NAMES, &["input_ids".to_string()])
            .expect_err("missing inputs");
        assert!(matches!(error, GlinerError::UnexpectedModelSchema { kind: "input", .. }));
    }
}
