#include <stdint.h>
#include <stdbool.h> 
typedef struct RustStr { uint8_t* const start; uintptr_t len; } RustStr;
typedef struct __private__FfiSlice { void* const start; uintptr_t len; } __private__FfiSlice;
void* __swift_bridge__null_pointer(void);


typedef struct __private__OptionU8 { uint8_t val; bool is_some; } __private__OptionU8;
typedef struct __private__OptionI8 { int8_t val; bool is_some; } __private__OptionI8;
typedef struct __private__OptionU16 { uint16_t val; bool is_some; } __private__OptionU16;
typedef struct __private__OptionI16 { int16_t val; bool is_some; } __private__OptionI16;
typedef struct __private__OptionU32 { uint32_t val; bool is_some; } __private__OptionU32;
typedef struct __private__OptionI32 { int32_t val; bool is_some; } __private__OptionI32;
typedef struct __private__OptionU64 { uint64_t val; bool is_some; } __private__OptionU64;
typedef struct __private__OptionI64 { int64_t val; bool is_some; } __private__OptionI64;
typedef struct __private__OptionUsize { uintptr_t val; bool is_some; } __private__OptionUsize;
typedef struct __private__OptionIsize { intptr_t val; bool is_some; } __private__OptionIsize;
typedef struct __private__OptionF32 { float val; bool is_some; } __private__OptionF32;
typedef struct __private__OptionF64 { double val; bool is_some; } __private__OptionF64;
typedef struct __private__OptionBool { bool val; bool is_some; } __private__OptionBool;

void* __swift_bridge__$Vec_u8$new();
void __swift_bridge__$Vec_u8$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_u8$len(void* const vec);
void __swift_bridge__$Vec_u8$push(void* const vec, uint8_t val);
__private__OptionU8 __swift_bridge__$Vec_u8$pop(void* const vec);
__private__OptionU8 __swift_bridge__$Vec_u8$get(void* const vec, uintptr_t index);
__private__OptionU8 __swift_bridge__$Vec_u8$get_mut(void* const vec, uintptr_t index);
uint8_t const * __swift_bridge__$Vec_u8$as_ptr(void* const vec);

void* __swift_bridge__$Vec_u16$new();
void __swift_bridge__$Vec_u16$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_u16$len(void* const vec);
void __swift_bridge__$Vec_u16$push(void* const vec, uint16_t val);
__private__OptionU16 __swift_bridge__$Vec_u16$pop(void* const vec);
__private__OptionU16 __swift_bridge__$Vec_u16$get(void* const vec, uintptr_t index);
__private__OptionU16 __swift_bridge__$Vec_u16$get_mut(void* const vec, uintptr_t index);
uint16_t const * __swift_bridge__$Vec_u16$as_ptr(void* const vec);

void* __swift_bridge__$Vec_u32$new();
void __swift_bridge__$Vec_u32$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_u32$len(void* const vec);
void __swift_bridge__$Vec_u32$push(void* const vec, uint32_t val);
__private__OptionU32 __swift_bridge__$Vec_u32$pop(void* const vec);
__private__OptionU32 __swift_bridge__$Vec_u32$get(void* const vec, uintptr_t index);
__private__OptionU32 __swift_bridge__$Vec_u32$get_mut(void* const vec, uintptr_t index);
uint32_t const * __swift_bridge__$Vec_u32$as_ptr(void* const vec);

void* __swift_bridge__$Vec_u64$new();
void __swift_bridge__$Vec_u64$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_u64$len(void* const vec);
void __swift_bridge__$Vec_u64$push(void* const vec, uint64_t val);
__private__OptionU64 __swift_bridge__$Vec_u64$pop(void* const vec);
__private__OptionU64 __swift_bridge__$Vec_u64$get(void* const vec, uintptr_t index);
__private__OptionU64 __swift_bridge__$Vec_u64$get_mut(void* const vec, uintptr_t index);
uint64_t const * __swift_bridge__$Vec_u64$as_ptr(void* const vec);

void* __swift_bridge__$Vec_usize$new();
void __swift_bridge__$Vec_usize$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_usize$len(void* const vec);
void __swift_bridge__$Vec_usize$push(void* const vec, uintptr_t val);
__private__OptionUsize __swift_bridge__$Vec_usize$pop(void* const vec);
__private__OptionUsize __swift_bridge__$Vec_usize$get(void* const vec, uintptr_t index);
__private__OptionUsize __swift_bridge__$Vec_usize$get_mut(void* const vec, uintptr_t index);
uintptr_t const * __swift_bridge__$Vec_usize$as_ptr(void* const vec);

void* __swift_bridge__$Vec_i8$new();
void __swift_bridge__$Vec_i8$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_i8$len(void* const vec);
void __swift_bridge__$Vec_i8$push(void* const vec, int8_t val);
__private__OptionI8 __swift_bridge__$Vec_i8$pop(void* const vec);
__private__OptionI8 __swift_bridge__$Vec_i8$get(void* const vec, uintptr_t index);
__private__OptionI8 __swift_bridge__$Vec_i8$get_mut(void* const vec, uintptr_t index);
int8_t const * __swift_bridge__$Vec_i8$as_ptr(void* const vec);

void* __swift_bridge__$Vec_i16$new();
void __swift_bridge__$Vec_i16$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_i16$len(void* const vec);
void __swift_bridge__$Vec_i16$push(void* const vec, int16_t val);
__private__OptionI16 __swift_bridge__$Vec_i16$pop(void* const vec);
__private__OptionI16 __swift_bridge__$Vec_i16$get(void* const vec, uintptr_t index);
__private__OptionI16 __swift_bridge__$Vec_i16$get_mut(void* const vec, uintptr_t index);
int16_t const * __swift_bridge__$Vec_i16$as_ptr(void* const vec);

void* __swift_bridge__$Vec_i32$new();
void __swift_bridge__$Vec_i32$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_i32$len(void* const vec);
void __swift_bridge__$Vec_i32$push(void* const vec, int32_t val);
__private__OptionI32 __swift_bridge__$Vec_i32$pop(void* const vec);
__private__OptionI32 __swift_bridge__$Vec_i32$get(void* const vec, uintptr_t index);
__private__OptionI32 __swift_bridge__$Vec_i32$get_mut(void* const vec, uintptr_t index);
int32_t const * __swift_bridge__$Vec_i32$as_ptr(void* const vec);

void* __swift_bridge__$Vec_i64$new();
void __swift_bridge__$Vec_i64$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_i64$len(void* const vec);
void __swift_bridge__$Vec_i64$push(void* const vec, int64_t val);
__private__OptionI64 __swift_bridge__$Vec_i64$pop(void* const vec);
__private__OptionI64 __swift_bridge__$Vec_i64$get(void* const vec, uintptr_t index);
__private__OptionI64 __swift_bridge__$Vec_i64$get_mut(void* const vec, uintptr_t index);
int64_t const * __swift_bridge__$Vec_i64$as_ptr(void* const vec);

void* __swift_bridge__$Vec_isize$new();
void __swift_bridge__$Vec_isize$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_isize$len(void* const vec);
void __swift_bridge__$Vec_isize$push(void* const vec, intptr_t val);
__private__OptionIsize __swift_bridge__$Vec_isize$pop(void* const vec);
__private__OptionIsize __swift_bridge__$Vec_isize$get(void* const vec, uintptr_t index);
__private__OptionIsize __swift_bridge__$Vec_isize$get_mut(void* const vec, uintptr_t index);
intptr_t const * __swift_bridge__$Vec_isize$as_ptr(void* const vec);

void* __swift_bridge__$Vec_bool$new();
void __swift_bridge__$Vec_bool$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_bool$len(void* const vec);
void __swift_bridge__$Vec_bool$push(void* const vec, bool val);
__private__OptionBool __swift_bridge__$Vec_bool$pop(void* const vec);
__private__OptionBool __swift_bridge__$Vec_bool$get(void* const vec, uintptr_t index);
__private__OptionBool __swift_bridge__$Vec_bool$get_mut(void* const vec, uintptr_t index);
bool const * __swift_bridge__$Vec_bool$as_ptr(void* const vec);

void* __swift_bridge__$Vec_f32$new();
void __swift_bridge__$Vec_f32$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_f32$len(void* const vec);
void __swift_bridge__$Vec_f32$push(void* const vec, float val);
__private__OptionF32 __swift_bridge__$Vec_f32$pop(void* const vec);
__private__OptionF32 __swift_bridge__$Vec_f32$get(void* const vec, uintptr_t index);
__private__OptionF32 __swift_bridge__$Vec_f32$get_mut(void* const vec, uintptr_t index);
float const * __swift_bridge__$Vec_f32$as_ptr(void* const vec);

void* __swift_bridge__$Vec_f64$new();
void __swift_bridge__$Vec_f64$_free(void* const vec);
uintptr_t __swift_bridge__$Vec_f64$len(void* const vec);
void __swift_bridge__$Vec_f64$push(void* const vec, double val);
__private__OptionF64 __swift_bridge__$Vec_f64$pop(void* const vec);
__private__OptionF64 __swift_bridge__$Vec_f64$get(void* const vec, uintptr_t index);
__private__OptionF64 __swift_bridge__$Vec_f64$get_mut(void* const vec, uintptr_t index);
double const * __swift_bridge__$Vec_f64$as_ptr(void* const vec);

#include <stdint.h>
typedef struct RustString RustString;
void __swift_bridge__$RustString$_free(void* self);

void* __swift_bridge__$Vec_RustString$new(void);
void __swift_bridge__$Vec_RustString$drop(void* vec_ptr);
void __swift_bridge__$Vec_RustString$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_RustString$pop(void* vec_ptr);
void* __swift_bridge__$Vec_RustString$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_RustString$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_RustString$len(void* vec_ptr);
void* __swift_bridge__$Vec_RustString$as_ptr(void* vec_ptr);

void* __swift_bridge__$RustString$new(void);
void* __swift_bridge__$RustString$new_with_str(struct RustStr str);
uintptr_t __swift_bridge__$RustString$len(void* self);
struct RustStr __swift_bridge__$RustString$as_str(void* self);
struct RustStr __swift_bridge__$RustString$trim(void* self);
bool __swift_bridge__$RustStr$partial_eq(struct RustStr lhs, struct RustStr rhs);


void __swift_bridge__$call_boxed_fn_once_no_args_no_return(void* boxed_fnonce);
void __swift_bridge__$free_boxed_fn_once_no_args_no_return(void* boxed_fnonce);


struct __private__ResultPtrAndPtr { bool is_ok; void* ok_or_err; };
// File automatically generated by swift-bridge.
#include <stdint.h>
#include <stdbool.h>
typedef struct AccelerationConfig AccelerationConfig;
void __swift_bridge__$AccelerationConfig$_free(void* self);

void* __swift_bridge__$Vec_AccelerationConfig$new(void);
void __swift_bridge__$Vec_AccelerationConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_AccelerationConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_AccelerationConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_AccelerationConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_AccelerationConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_AccelerationConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_AccelerationConfig$as_ptr(void* vec_ptr);

typedef struct ContentFilterConfig ContentFilterConfig;
void __swift_bridge__$ContentFilterConfig$_free(void* self);

void* __swift_bridge__$Vec_ContentFilterConfig$new(void);
void __swift_bridge__$Vec_ContentFilterConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_ContentFilterConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ContentFilterConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ContentFilterConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ContentFilterConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ContentFilterConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_ContentFilterConfig$as_ptr(void* vec_ptr);

typedef struct EmailConfig EmailConfig;
void __swift_bridge__$EmailConfig$_free(void* self);

void* __swift_bridge__$Vec_EmailConfig$new(void);
void __swift_bridge__$Vec_EmailConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmailConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmailConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmailConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmailConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmailConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmailConfig$as_ptr(void* vec_ptr);

typedef struct ExtractionConfig ExtractionConfig;
void __swift_bridge__$ExtractionConfig$_free(void* self);

void* __swift_bridge__$Vec_ExtractionConfig$new(void);
void __swift_bridge__$Vec_ExtractionConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExtractionConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExtractionConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractionConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExtractionConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExtractionConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractionConfig$as_ptr(void* vec_ptr);

typedef struct FileExtractionConfig FileExtractionConfig;
void __swift_bridge__$FileExtractionConfig$_free(void* self);

void* __swift_bridge__$Vec_FileExtractionConfig$new(void);
void __swift_bridge__$Vec_FileExtractionConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_FileExtractionConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_FileExtractionConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_FileExtractionConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_FileExtractionConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_FileExtractionConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_FileExtractionConfig$as_ptr(void* vec_ptr);

typedef struct ImageExtractionConfig ImageExtractionConfig;
void __swift_bridge__$ImageExtractionConfig$_free(void* self);

void* __swift_bridge__$Vec_ImageExtractionConfig$new(void);
void __swift_bridge__$Vec_ImageExtractionConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_ImageExtractionConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ImageExtractionConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ImageExtractionConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ImageExtractionConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ImageExtractionConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_ImageExtractionConfig$as_ptr(void* vec_ptr);

typedef struct TokenReductionOptions TokenReductionOptions;
void __swift_bridge__$TokenReductionOptions$_free(void* self);

void* __swift_bridge__$Vec_TokenReductionOptions$new(void);
void __swift_bridge__$Vec_TokenReductionOptions$drop(void* vec_ptr);
void __swift_bridge__$Vec_TokenReductionOptions$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TokenReductionOptions$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TokenReductionOptions$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TokenReductionOptions$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TokenReductionOptions$len(void* vec_ptr);
void* __swift_bridge__$Vec_TokenReductionOptions$as_ptr(void* vec_ptr);

typedef struct LanguageDetectionConfig LanguageDetectionConfig;
void __swift_bridge__$LanguageDetectionConfig$_free(void* self);

void* __swift_bridge__$Vec_LanguageDetectionConfig$new(void);
void __swift_bridge__$Vec_LanguageDetectionConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_LanguageDetectionConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LanguageDetectionConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LanguageDetectionConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LanguageDetectionConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LanguageDetectionConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_LanguageDetectionConfig$as_ptr(void* vec_ptr);

typedef struct HtmlOutputConfig HtmlOutputConfig;
void __swift_bridge__$HtmlOutputConfig$_free(void* self);

void* __swift_bridge__$Vec_HtmlOutputConfig$new(void);
void __swift_bridge__$Vec_HtmlOutputConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_HtmlOutputConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HtmlOutputConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HtmlOutputConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HtmlOutputConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HtmlOutputConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_HtmlOutputConfig$as_ptr(void* vec_ptr);

typedef struct LayoutDetectionConfig LayoutDetectionConfig;
void __swift_bridge__$LayoutDetectionConfig$_free(void* self);

void* __swift_bridge__$Vec_LayoutDetectionConfig$new(void);
void __swift_bridge__$Vec_LayoutDetectionConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_LayoutDetectionConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LayoutDetectionConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LayoutDetectionConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LayoutDetectionConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LayoutDetectionConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_LayoutDetectionConfig$as_ptr(void* vec_ptr);

typedef struct LlmConfig LlmConfig;
void __swift_bridge__$LlmConfig$_free(void* self);

void* __swift_bridge__$Vec_LlmConfig$new(void);
void __swift_bridge__$Vec_LlmConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_LlmConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LlmConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LlmConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LlmConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LlmConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_LlmConfig$as_ptr(void* vec_ptr);

typedef struct StructuredExtractionConfig StructuredExtractionConfig;
void __swift_bridge__$StructuredExtractionConfig$_free(void* self);

void* __swift_bridge__$Vec_StructuredExtractionConfig$new(void);
void __swift_bridge__$Vec_StructuredExtractionConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_StructuredExtractionConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_StructuredExtractionConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredExtractionConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_StructuredExtractionConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_StructuredExtractionConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredExtractionConfig$as_ptr(void* vec_ptr);

typedef struct OcrQualityThresholds OcrQualityThresholds;
void __swift_bridge__$OcrQualityThresholds$_free(void* self);

void* __swift_bridge__$Vec_OcrQualityThresholds$new(void);
void __swift_bridge__$Vec_OcrQualityThresholds$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrQualityThresholds$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrQualityThresholds$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrQualityThresholds$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrQualityThresholds$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrQualityThresholds$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrQualityThresholds$as_ptr(void* vec_ptr);

typedef struct OcrPipelineStage OcrPipelineStage;
void __swift_bridge__$OcrPipelineStage$_free(void* self);

void* __swift_bridge__$Vec_OcrPipelineStage$new(void);
void __swift_bridge__$Vec_OcrPipelineStage$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrPipelineStage$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrPipelineStage$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrPipelineStage$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrPipelineStage$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrPipelineStage$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrPipelineStage$as_ptr(void* vec_ptr);

typedef struct OcrPipelineConfig OcrPipelineConfig;
void __swift_bridge__$OcrPipelineConfig$_free(void* self);

void* __swift_bridge__$Vec_OcrPipelineConfig$new(void);
void __swift_bridge__$Vec_OcrPipelineConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrPipelineConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrPipelineConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrPipelineConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrPipelineConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrPipelineConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrPipelineConfig$as_ptr(void* vec_ptr);

typedef struct OcrConfig OcrConfig;
void __swift_bridge__$OcrConfig$_free(void* self);

void* __swift_bridge__$Vec_OcrConfig$new(void);
void __swift_bridge__$Vec_OcrConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrConfig$as_ptr(void* vec_ptr);

typedef struct PageConfig PageConfig;
void __swift_bridge__$PageConfig$_free(void* self);

void* __swift_bridge__$Vec_PageConfig$new(void);
void __swift_bridge__$Vec_PageConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageConfig$as_ptr(void* vec_ptr);

typedef struct PdfConfig PdfConfig;
void __swift_bridge__$PdfConfig$_free(void* self);

void* __swift_bridge__$Vec_PdfConfig$new(void);
void __swift_bridge__$Vec_PdfConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_PdfConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PdfConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PdfConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PdfConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PdfConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_PdfConfig$as_ptr(void* vec_ptr);

typedef struct HierarchyConfig HierarchyConfig;
void __swift_bridge__$HierarchyConfig$_free(void* self);

void* __swift_bridge__$Vec_HierarchyConfig$new(void);
void __swift_bridge__$Vec_HierarchyConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_HierarchyConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HierarchyConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HierarchyConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HierarchyConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HierarchyConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_HierarchyConfig$as_ptr(void* vec_ptr);

typedef struct PostProcessorConfig PostProcessorConfig;
void __swift_bridge__$PostProcessorConfig$_free(void* self);

void* __swift_bridge__$Vec_PostProcessorConfig$new(void);
void __swift_bridge__$Vec_PostProcessorConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_PostProcessorConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PostProcessorConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PostProcessorConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PostProcessorConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PostProcessorConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_PostProcessorConfig$as_ptr(void* vec_ptr);

typedef struct ChunkingConfig ChunkingConfig;
void __swift_bridge__$ChunkingConfig$_free(void* self);

void* __swift_bridge__$Vec_ChunkingConfig$new(void);
void __swift_bridge__$Vec_ChunkingConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkingConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkingConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkingConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkingConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkingConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkingConfig$as_ptr(void* vec_ptr);

typedef struct EmbeddingConfig EmbeddingConfig;
void __swift_bridge__$EmbeddingConfig$_free(void* self);

void* __swift_bridge__$Vec_EmbeddingConfig$new(void);
void __swift_bridge__$Vec_EmbeddingConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmbeddingConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmbeddingConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmbeddingConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmbeddingConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmbeddingConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmbeddingConfig$as_ptr(void* vec_ptr);

typedef struct TreeSitterConfig TreeSitterConfig;
void __swift_bridge__$TreeSitterConfig$_free(void* self);

void* __swift_bridge__$Vec_TreeSitterConfig$new(void);
void __swift_bridge__$Vec_TreeSitterConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_TreeSitterConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TreeSitterConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TreeSitterConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TreeSitterConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TreeSitterConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_TreeSitterConfig$as_ptr(void* vec_ptr);

typedef struct TreeSitterProcessConfig TreeSitterProcessConfig;
void __swift_bridge__$TreeSitterProcessConfig$_free(void* self);

void* __swift_bridge__$Vec_TreeSitterProcessConfig$new(void);
void __swift_bridge__$Vec_TreeSitterProcessConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_TreeSitterProcessConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TreeSitterProcessConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TreeSitterProcessConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TreeSitterProcessConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TreeSitterProcessConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_TreeSitterProcessConfig$as_ptr(void* vec_ptr);

typedef struct SupportedFormat SupportedFormat;
void __swift_bridge__$SupportedFormat$_free(void* self);

void* __swift_bridge__$Vec_SupportedFormat$new(void);
void __swift_bridge__$Vec_SupportedFormat$drop(void* vec_ptr);
void __swift_bridge__$Vec_SupportedFormat$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_SupportedFormat$pop(void* vec_ptr);
void* __swift_bridge__$Vec_SupportedFormat$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_SupportedFormat$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_SupportedFormat$len(void* vec_ptr);
void* __swift_bridge__$Vec_SupportedFormat$as_ptr(void* vec_ptr);

typedef struct ServerConfig ServerConfig;
void __swift_bridge__$ServerConfig$_free(void* self);

void* __swift_bridge__$Vec_ServerConfig$new(void);
void __swift_bridge__$Vec_ServerConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_ServerConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ServerConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ServerConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ServerConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ServerConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_ServerConfig$as_ptr(void* vec_ptr);

typedef struct StructuredDataResult StructuredDataResult;
void __swift_bridge__$StructuredDataResult$_free(void* self);

void* __swift_bridge__$Vec_StructuredDataResult$new(void);
void __swift_bridge__$Vec_StructuredDataResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_StructuredDataResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_StructuredDataResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredDataResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_StructuredDataResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_StructuredDataResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredDataResult$as_ptr(void* vec_ptr);

typedef struct StreamReader StreamReader;
void __swift_bridge__$StreamReader$_free(void* self);

void* __swift_bridge__$Vec_StreamReader$new(void);
void __swift_bridge__$Vec_StreamReader$drop(void* vec_ptr);
void __swift_bridge__$Vec_StreamReader$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_StreamReader$pop(void* vec_ptr);
void* __swift_bridge__$Vec_StreamReader$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_StreamReader$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_StreamReader$len(void* vec_ptr);
void* __swift_bridge__$Vec_StreamReader$as_ptr(void* vec_ptr);

typedef struct ImageOcrResult ImageOcrResult;
void __swift_bridge__$ImageOcrResult$_free(void* self);

void* __swift_bridge__$Vec_ImageOcrResult$new(void);
void __swift_bridge__$Vec_ImageOcrResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_ImageOcrResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ImageOcrResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ImageOcrResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ImageOcrResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ImageOcrResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_ImageOcrResult$as_ptr(void* vec_ptr);

typedef struct HtmlExtractionResult HtmlExtractionResult;
void __swift_bridge__$HtmlExtractionResult$_free(void* self);

void* __swift_bridge__$Vec_HtmlExtractionResult$new(void);
void __swift_bridge__$Vec_HtmlExtractionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_HtmlExtractionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HtmlExtractionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HtmlExtractionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HtmlExtractionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HtmlExtractionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_HtmlExtractionResult$as_ptr(void* vec_ptr);

typedef struct ExtractedInlineImage ExtractedInlineImage;
void __swift_bridge__$ExtractedInlineImage$_free(void* self);

void* __swift_bridge__$Vec_ExtractedInlineImage$new(void);
void __swift_bridge__$Vec_ExtractedInlineImage$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExtractedInlineImage$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExtractedInlineImage$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractedInlineImage$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExtractedInlineImage$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExtractedInlineImage$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractedInlineImage$as_ptr(void* vec_ptr);

typedef struct Drawing Drawing;
void __swift_bridge__$Drawing$_free(void* self);

void* __swift_bridge__$Vec_Drawing$new(void);
void __swift_bridge__$Vec_Drawing$drop(void* vec_ptr);
void __swift_bridge__$Vec_Drawing$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Drawing$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Drawing$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Drawing$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Drawing$len(void* vec_ptr);
void* __swift_bridge__$Vec_Drawing$as_ptr(void* vec_ptr);

typedef struct AnchorProperties AnchorProperties;
void __swift_bridge__$AnchorProperties$_free(void* self);

void* __swift_bridge__$Vec_AnchorProperties$new(void);
void __swift_bridge__$Vec_AnchorProperties$drop(void* vec_ptr);
void __swift_bridge__$Vec_AnchorProperties$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_AnchorProperties$pop(void* vec_ptr);
void* __swift_bridge__$Vec_AnchorProperties$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_AnchorProperties$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_AnchorProperties$len(void* vec_ptr);
void* __swift_bridge__$Vec_AnchorProperties$as_ptr(void* vec_ptr);

typedef struct HeaderFooter HeaderFooter;
void __swift_bridge__$HeaderFooter$_free(void* self);

void* __swift_bridge__$Vec_HeaderFooter$new(void);
void __swift_bridge__$Vec_HeaderFooter$drop(void* vec_ptr);
void __swift_bridge__$Vec_HeaderFooter$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HeaderFooter$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HeaderFooter$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HeaderFooter$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HeaderFooter$len(void* vec_ptr);
void* __swift_bridge__$Vec_HeaderFooter$as_ptr(void* vec_ptr);

typedef struct Note Note;
void __swift_bridge__$Note$_free(void* self);

void* __swift_bridge__$Vec_Note$new(void);
void __swift_bridge__$Vec_Note$drop(void* vec_ptr);
void __swift_bridge__$Vec_Note$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Note$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Note$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Note$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Note$len(void* vec_ptr);
void* __swift_bridge__$Vec_Note$as_ptr(void* vec_ptr);

typedef struct PageMarginsPoints PageMarginsPoints;
void __swift_bridge__$PageMarginsPoints$_free(void* self);

void* __swift_bridge__$Vec_PageMarginsPoints$new(void);
void __swift_bridge__$Vec_PageMarginsPoints$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageMarginsPoints$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageMarginsPoints$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageMarginsPoints$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageMarginsPoints$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageMarginsPoints$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageMarginsPoints$as_ptr(void* vec_ptr);

typedef struct StyleDefinition StyleDefinition;
void __swift_bridge__$StyleDefinition$_free(void* self);

void* __swift_bridge__$Vec_StyleDefinition$new(void);
void __swift_bridge__$Vec_StyleDefinition$drop(void* vec_ptr);
void __swift_bridge__$Vec_StyleDefinition$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_StyleDefinition$pop(void* vec_ptr);
void* __swift_bridge__$Vec_StyleDefinition$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_StyleDefinition$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_StyleDefinition$len(void* vec_ptr);
void* __swift_bridge__$Vec_StyleDefinition$as_ptr(void* vec_ptr);

typedef struct ResolvedStyle ResolvedStyle;
void __swift_bridge__$ResolvedStyle$_free(void* self);

void* __swift_bridge__$Vec_ResolvedStyle$new(void);
void __swift_bridge__$Vec_ResolvedStyle$drop(void* vec_ptr);
void __swift_bridge__$Vec_ResolvedStyle$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ResolvedStyle$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ResolvedStyle$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ResolvedStyle$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ResolvedStyle$len(void* vec_ptr);
void* __swift_bridge__$Vec_ResolvedStyle$as_ptr(void* vec_ptr);

typedef struct TableProperties TableProperties;
void __swift_bridge__$TableProperties$_free(void* self);

void* __swift_bridge__$Vec_TableProperties$new(void);
void __swift_bridge__$Vec_TableProperties$drop(void* vec_ptr);
void __swift_bridge__$Vec_TableProperties$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TableProperties$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TableProperties$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TableProperties$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TableProperties$len(void* vec_ptr);
void* __swift_bridge__$Vec_TableProperties$as_ptr(void* vec_ptr);

typedef struct XlsxAppProperties XlsxAppProperties;
void __swift_bridge__$XlsxAppProperties$_free(void* self);

void* __swift_bridge__$Vec_XlsxAppProperties$new(void);
void __swift_bridge__$Vec_XlsxAppProperties$drop(void* vec_ptr);
void __swift_bridge__$Vec_XlsxAppProperties$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_XlsxAppProperties$pop(void* vec_ptr);
void* __swift_bridge__$Vec_XlsxAppProperties$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_XlsxAppProperties$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_XlsxAppProperties$len(void* vec_ptr);
void* __swift_bridge__$Vec_XlsxAppProperties$as_ptr(void* vec_ptr);

typedef struct PptxAppProperties PptxAppProperties;
void __swift_bridge__$PptxAppProperties$_free(void* self);

void* __swift_bridge__$Vec_PptxAppProperties$new(void);
void __swift_bridge__$Vec_PptxAppProperties$drop(void* vec_ptr);
void __swift_bridge__$Vec_PptxAppProperties$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PptxAppProperties$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PptxAppProperties$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PptxAppProperties$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PptxAppProperties$len(void* vec_ptr);
void* __swift_bridge__$Vec_PptxAppProperties$as_ptr(void* vec_ptr);

typedef struct CustomProperties CustomProperties;
void __swift_bridge__$CustomProperties$_free(void* self);

void* __swift_bridge__$Vec_CustomProperties$new(void);
void __swift_bridge__$Vec_CustomProperties$drop(void* vec_ptr);
void __swift_bridge__$Vec_CustomProperties$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_CustomProperties$pop(void* vec_ptr);
void* __swift_bridge__$Vec_CustomProperties$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_CustomProperties$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_CustomProperties$len(void* vec_ptr);
void* __swift_bridge__$Vec_CustomProperties$as_ptr(void* vec_ptr);

typedef struct OdtProperties OdtProperties;
void __swift_bridge__$OdtProperties$_free(void* self);

void* __swift_bridge__$Vec_OdtProperties$new(void);
void __swift_bridge__$Vec_OdtProperties$drop(void* vec_ptr);
void __swift_bridge__$Vec_OdtProperties$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OdtProperties$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OdtProperties$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OdtProperties$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OdtProperties$len(void* vec_ptr);
void* __swift_bridge__$Vec_OdtProperties$as_ptr(void* vec_ptr);

typedef struct ZipBombValidator ZipBombValidator;
void __swift_bridge__$ZipBombValidator$_free(void* self);

void* __swift_bridge__$Vec_ZipBombValidator$new(void);
void __swift_bridge__$Vec_ZipBombValidator$drop(void* vec_ptr);
void __swift_bridge__$Vec_ZipBombValidator$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ZipBombValidator$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ZipBombValidator$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ZipBombValidator$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ZipBombValidator$len(void* vec_ptr);
void* __swift_bridge__$Vec_ZipBombValidator$as_ptr(void* vec_ptr);

typedef struct TokenReductionConfig TokenReductionConfig;
void __swift_bridge__$TokenReductionConfig$_free(void* self);

void* __swift_bridge__$Vec_TokenReductionConfig$new(void);
void __swift_bridge__$Vec_TokenReductionConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_TokenReductionConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TokenReductionConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TokenReductionConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TokenReductionConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TokenReductionConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_TokenReductionConfig$as_ptr(void* vec_ptr);

typedef struct PdfAnnotation PdfAnnotation;
void __swift_bridge__$PdfAnnotation$_free(void* self);

void* __swift_bridge__$Vec_PdfAnnotation$new(void);
void __swift_bridge__$Vec_PdfAnnotation$drop(void* vec_ptr);
void __swift_bridge__$Vec_PdfAnnotation$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PdfAnnotation$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PdfAnnotation$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PdfAnnotation$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PdfAnnotation$len(void* vec_ptr);
void* __swift_bridge__$Vec_PdfAnnotation$as_ptr(void* vec_ptr);

typedef struct DjotContent DjotContent;
void __swift_bridge__$DjotContent$_free(void* self);

void* __swift_bridge__$Vec_DjotContent$new(void);
void __swift_bridge__$Vec_DjotContent$drop(void* vec_ptr);
void __swift_bridge__$Vec_DjotContent$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DjotContent$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DjotContent$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DjotContent$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DjotContent$len(void* vec_ptr);
void* __swift_bridge__$Vec_DjotContent$as_ptr(void* vec_ptr);

typedef struct FormattedBlock FormattedBlock;
void __swift_bridge__$FormattedBlock$_free(void* self);

void* __swift_bridge__$Vec_FormattedBlock$new(void);
void __swift_bridge__$Vec_FormattedBlock$drop(void* vec_ptr);
void __swift_bridge__$Vec_FormattedBlock$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_FormattedBlock$pop(void* vec_ptr);
void* __swift_bridge__$Vec_FormattedBlock$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_FormattedBlock$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_FormattedBlock$len(void* vec_ptr);
void* __swift_bridge__$Vec_FormattedBlock$as_ptr(void* vec_ptr);

typedef struct InlineElement InlineElement;
void __swift_bridge__$InlineElement$_free(void* self);

void* __swift_bridge__$Vec_InlineElement$new(void);
void __swift_bridge__$Vec_InlineElement$drop(void* vec_ptr);
void __swift_bridge__$Vec_InlineElement$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_InlineElement$pop(void* vec_ptr);
void* __swift_bridge__$Vec_InlineElement$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_InlineElement$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_InlineElement$len(void* vec_ptr);
void* __swift_bridge__$Vec_InlineElement$as_ptr(void* vec_ptr);

typedef struct DjotImage DjotImage;
void __swift_bridge__$DjotImage$_free(void* self);

void* __swift_bridge__$Vec_DjotImage$new(void);
void __swift_bridge__$Vec_DjotImage$drop(void* vec_ptr);
void __swift_bridge__$Vec_DjotImage$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DjotImage$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DjotImage$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DjotImage$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DjotImage$len(void* vec_ptr);
void* __swift_bridge__$Vec_DjotImage$as_ptr(void* vec_ptr);

typedef struct DjotLink DjotLink;
void __swift_bridge__$DjotLink$_free(void* self);

void* __swift_bridge__$Vec_DjotLink$new(void);
void __swift_bridge__$Vec_DjotLink$drop(void* vec_ptr);
void __swift_bridge__$Vec_DjotLink$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DjotLink$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DjotLink$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DjotLink$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DjotLink$len(void* vec_ptr);
void* __swift_bridge__$Vec_DjotLink$as_ptr(void* vec_ptr);

typedef struct Footnote Footnote;
void __swift_bridge__$Footnote$_free(void* self);

void* __swift_bridge__$Vec_Footnote$new(void);
void __swift_bridge__$Vec_Footnote$drop(void* vec_ptr);
void __swift_bridge__$Vec_Footnote$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Footnote$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Footnote$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Footnote$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Footnote$len(void* vec_ptr);
void* __swift_bridge__$Vec_Footnote$as_ptr(void* vec_ptr);

typedef struct DocumentStructure DocumentStructure;
void __swift_bridge__$DocumentStructure$_free(void* self);

void* __swift_bridge__$Vec_DocumentStructure$new(void);
void __swift_bridge__$Vec_DocumentStructure$drop(void* vec_ptr);
void __swift_bridge__$Vec_DocumentStructure$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DocumentStructure$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DocumentStructure$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DocumentStructure$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DocumentStructure$len(void* vec_ptr);
void* __swift_bridge__$Vec_DocumentStructure$as_ptr(void* vec_ptr);

typedef struct DocumentRelationship DocumentRelationship;
void __swift_bridge__$DocumentRelationship$_free(void* self);

void* __swift_bridge__$Vec_DocumentRelationship$new(void);
void __swift_bridge__$Vec_DocumentRelationship$drop(void* vec_ptr);
void __swift_bridge__$Vec_DocumentRelationship$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DocumentRelationship$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DocumentRelationship$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DocumentRelationship$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DocumentRelationship$len(void* vec_ptr);
void* __swift_bridge__$Vec_DocumentRelationship$as_ptr(void* vec_ptr);

typedef struct DocumentNode DocumentNode;
void __swift_bridge__$DocumentNode$_free(void* self);

void* __swift_bridge__$Vec_DocumentNode$new(void);
void __swift_bridge__$Vec_DocumentNode$drop(void* vec_ptr);
void __swift_bridge__$Vec_DocumentNode$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DocumentNode$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DocumentNode$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DocumentNode$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DocumentNode$len(void* vec_ptr);
void* __swift_bridge__$Vec_DocumentNode$as_ptr(void* vec_ptr);

typedef struct GridCell GridCell;
void __swift_bridge__$GridCell$_free(void* self);

void* __swift_bridge__$Vec_GridCell$new(void);
void __swift_bridge__$Vec_GridCell$drop(void* vec_ptr);
void __swift_bridge__$Vec_GridCell$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_GridCell$pop(void* vec_ptr);
void* __swift_bridge__$Vec_GridCell$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_GridCell$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_GridCell$len(void* vec_ptr);
void* __swift_bridge__$Vec_GridCell$as_ptr(void* vec_ptr);

typedef struct TextAnnotation TextAnnotation;
void __swift_bridge__$TextAnnotation$_free(void* self);

void* __swift_bridge__$Vec_TextAnnotation$new(void);
void __swift_bridge__$Vec_TextAnnotation$drop(void* vec_ptr);
void __swift_bridge__$Vec_TextAnnotation$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TextAnnotation$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TextAnnotation$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TextAnnotation$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TextAnnotation$len(void* vec_ptr);
void* __swift_bridge__$Vec_TextAnnotation$as_ptr(void* vec_ptr);

typedef struct ExtractionResult ExtractionResult;
void __swift_bridge__$ExtractionResult$_free(void* self);

void* __swift_bridge__$Vec_ExtractionResult$new(void);
void __swift_bridge__$Vec_ExtractionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExtractionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExtractionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExtractionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExtractionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractionResult$as_ptr(void* vec_ptr);

typedef struct ArchiveEntry ArchiveEntry;
void __swift_bridge__$ArchiveEntry$_free(void* self);

void* __swift_bridge__$Vec_ArchiveEntry$new(void);
void __swift_bridge__$Vec_ArchiveEntry$drop(void* vec_ptr);
void __swift_bridge__$Vec_ArchiveEntry$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ArchiveEntry$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ArchiveEntry$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ArchiveEntry$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ArchiveEntry$len(void* vec_ptr);
void* __swift_bridge__$Vec_ArchiveEntry$as_ptr(void* vec_ptr);

typedef struct ProcessingWarning ProcessingWarning;
void __swift_bridge__$ProcessingWarning$_free(void* self);

void* __swift_bridge__$Vec_ProcessingWarning$new(void);
void __swift_bridge__$Vec_ProcessingWarning$drop(void* vec_ptr);
void __swift_bridge__$Vec_ProcessingWarning$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ProcessingWarning$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ProcessingWarning$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ProcessingWarning$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ProcessingWarning$len(void* vec_ptr);
void* __swift_bridge__$Vec_ProcessingWarning$as_ptr(void* vec_ptr);

typedef struct LlmUsage LlmUsage;
void __swift_bridge__$LlmUsage$_free(void* self);

void* __swift_bridge__$Vec_LlmUsage$new(void);
void __swift_bridge__$Vec_LlmUsage$drop(void* vec_ptr);
void __swift_bridge__$Vec_LlmUsage$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LlmUsage$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LlmUsage$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LlmUsage$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LlmUsage$len(void* vec_ptr);
void* __swift_bridge__$Vec_LlmUsage$as_ptr(void* vec_ptr);

typedef struct Chunk Chunk;
void __swift_bridge__$Chunk$_free(void* self);

void* __swift_bridge__$Vec_Chunk$new(void);
void __swift_bridge__$Vec_Chunk$drop(void* vec_ptr);
void __swift_bridge__$Vec_Chunk$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Chunk$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Chunk$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Chunk$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Chunk$len(void* vec_ptr);
void* __swift_bridge__$Vec_Chunk$as_ptr(void* vec_ptr);

typedef struct HeadingContext HeadingContext;
void __swift_bridge__$HeadingContext$_free(void* self);

void* __swift_bridge__$Vec_HeadingContext$new(void);
void __swift_bridge__$Vec_HeadingContext$drop(void* vec_ptr);
void __swift_bridge__$Vec_HeadingContext$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HeadingContext$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HeadingContext$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HeadingContext$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HeadingContext$len(void* vec_ptr);
void* __swift_bridge__$Vec_HeadingContext$as_ptr(void* vec_ptr);

typedef struct HeadingLevel HeadingLevel;
void __swift_bridge__$HeadingLevel$_free(void* self);

void* __swift_bridge__$Vec_HeadingLevel$new(void);
void __swift_bridge__$Vec_HeadingLevel$drop(void* vec_ptr);
void __swift_bridge__$Vec_HeadingLevel$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HeadingLevel$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HeadingLevel$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HeadingLevel$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HeadingLevel$len(void* vec_ptr);
void* __swift_bridge__$Vec_HeadingLevel$as_ptr(void* vec_ptr);

typedef struct ChunkMetadata ChunkMetadata;
void __swift_bridge__$ChunkMetadata$_free(void* self);

void* __swift_bridge__$Vec_ChunkMetadata$new(void);
void __swift_bridge__$Vec_ChunkMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkMetadata$as_ptr(void* vec_ptr);

typedef struct ExtractedImage ExtractedImage;
void __swift_bridge__$ExtractedImage$_free(void* self);

void* __swift_bridge__$Vec_ExtractedImage$new(void);
void __swift_bridge__$Vec_ExtractedImage$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExtractedImage$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExtractedImage$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractedImage$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExtractedImage$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExtractedImage$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractedImage$as_ptr(void* vec_ptr);

typedef struct ElementMetadata ElementMetadata;
void __swift_bridge__$ElementMetadata$_free(void* self);

void* __swift_bridge__$Vec_ElementMetadata$new(void);
void __swift_bridge__$Vec_ElementMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_ElementMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ElementMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ElementMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ElementMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ElementMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_ElementMetadata$as_ptr(void* vec_ptr);

typedef struct Element Element;
void __swift_bridge__$Element$_free(void* self);

void* __swift_bridge__$Vec_Element$new(void);
void __swift_bridge__$Vec_Element$drop(void* vec_ptr);
void __swift_bridge__$Vec_Element$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Element$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Element$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Element$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Element$len(void* vec_ptr);
void* __swift_bridge__$Vec_Element$as_ptr(void* vec_ptr);

typedef struct ExcelWorkbook ExcelWorkbook;
void __swift_bridge__$ExcelWorkbook$_free(void* self);

void* __swift_bridge__$Vec_ExcelWorkbook$new(void);
void __swift_bridge__$Vec_ExcelWorkbook$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExcelWorkbook$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExcelWorkbook$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExcelWorkbook$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExcelWorkbook$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExcelWorkbook$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExcelWorkbook$as_ptr(void* vec_ptr);

typedef struct ExcelSheet ExcelSheet;
void __swift_bridge__$ExcelSheet$_free(void* self);

void* __swift_bridge__$Vec_ExcelSheet$new(void);
void __swift_bridge__$Vec_ExcelSheet$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExcelSheet$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExcelSheet$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExcelSheet$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExcelSheet$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExcelSheet$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExcelSheet$as_ptr(void* vec_ptr);

typedef struct XmlExtractionResult XmlExtractionResult;
void __swift_bridge__$XmlExtractionResult$_free(void* self);

void* __swift_bridge__$Vec_XmlExtractionResult$new(void);
void __swift_bridge__$Vec_XmlExtractionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_XmlExtractionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_XmlExtractionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_XmlExtractionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_XmlExtractionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_XmlExtractionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_XmlExtractionResult$as_ptr(void* vec_ptr);

typedef struct TextExtractionResult TextExtractionResult;
void __swift_bridge__$TextExtractionResult$_free(void* self);

void* __swift_bridge__$Vec_TextExtractionResult$new(void);
void __swift_bridge__$Vec_TextExtractionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_TextExtractionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TextExtractionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TextExtractionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TextExtractionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TextExtractionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_TextExtractionResult$as_ptr(void* vec_ptr);

typedef struct PptxExtractionResult PptxExtractionResult;
void __swift_bridge__$PptxExtractionResult$_free(void* self);

void* __swift_bridge__$Vec_PptxExtractionResult$new(void);
void __swift_bridge__$Vec_PptxExtractionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_PptxExtractionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PptxExtractionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PptxExtractionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PptxExtractionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PptxExtractionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_PptxExtractionResult$as_ptr(void* vec_ptr);

typedef struct EmailExtractionResult EmailExtractionResult;
void __swift_bridge__$EmailExtractionResult$_free(void* self);

void* __swift_bridge__$Vec_EmailExtractionResult$new(void);
void __swift_bridge__$Vec_EmailExtractionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmailExtractionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmailExtractionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmailExtractionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmailExtractionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmailExtractionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmailExtractionResult$as_ptr(void* vec_ptr);

typedef struct EmailAttachment EmailAttachment;
void __swift_bridge__$EmailAttachment$_free(void* self);

void* __swift_bridge__$Vec_EmailAttachment$new(void);
void __swift_bridge__$Vec_EmailAttachment$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmailAttachment$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmailAttachment$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmailAttachment$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmailAttachment$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmailAttachment$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmailAttachment$as_ptr(void* vec_ptr);

typedef struct OcrExtractionResult OcrExtractionResult;
void __swift_bridge__$OcrExtractionResult$_free(void* self);

void* __swift_bridge__$Vec_OcrExtractionResult$new(void);
void __swift_bridge__$Vec_OcrExtractionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrExtractionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrExtractionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrExtractionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrExtractionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrExtractionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrExtractionResult$as_ptr(void* vec_ptr);

typedef struct OcrTable OcrTable;
void __swift_bridge__$OcrTable$_free(void* self);

void* __swift_bridge__$Vec_OcrTable$new(void);
void __swift_bridge__$Vec_OcrTable$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrTable$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrTable$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrTable$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrTable$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrTable$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrTable$as_ptr(void* vec_ptr);

typedef struct OcrTableBoundingBox OcrTableBoundingBox;
void __swift_bridge__$OcrTableBoundingBox$_free(void* self);

void* __swift_bridge__$Vec_OcrTableBoundingBox$new(void);
void __swift_bridge__$Vec_OcrTableBoundingBox$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrTableBoundingBox$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrTableBoundingBox$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrTableBoundingBox$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrTableBoundingBox$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrTableBoundingBox$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrTableBoundingBox$as_ptr(void* vec_ptr);

typedef struct ImagePreprocessingConfig ImagePreprocessingConfig;
void __swift_bridge__$ImagePreprocessingConfig$_free(void* self);

void* __swift_bridge__$Vec_ImagePreprocessingConfig$new(void);
void __swift_bridge__$Vec_ImagePreprocessingConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_ImagePreprocessingConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ImagePreprocessingConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ImagePreprocessingConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ImagePreprocessingConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ImagePreprocessingConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_ImagePreprocessingConfig$as_ptr(void* vec_ptr);

typedef struct TesseractConfig TesseractConfig;
void __swift_bridge__$TesseractConfig$_free(void* self);

void* __swift_bridge__$Vec_TesseractConfig$new(void);
void __swift_bridge__$Vec_TesseractConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_TesseractConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TesseractConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TesseractConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TesseractConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TesseractConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_TesseractConfig$as_ptr(void* vec_ptr);

typedef struct ImagePreprocessingMetadata ImagePreprocessingMetadata;
void __swift_bridge__$ImagePreprocessingMetadata$_free(void* self);

void* __swift_bridge__$Vec_ImagePreprocessingMetadata$new(void);
void __swift_bridge__$Vec_ImagePreprocessingMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_ImagePreprocessingMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ImagePreprocessingMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ImagePreprocessingMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ImagePreprocessingMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ImagePreprocessingMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_ImagePreprocessingMetadata$as_ptr(void* vec_ptr);

typedef struct Metadata Metadata;
void __swift_bridge__$Metadata$_free(void* self);

void* __swift_bridge__$Vec_Metadata$new(void);
void __swift_bridge__$Vec_Metadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_Metadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Metadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Metadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Metadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Metadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_Metadata$as_ptr(void* vec_ptr);

typedef struct ExcelMetadata ExcelMetadata;
void __swift_bridge__$ExcelMetadata$_free(void* self);

void* __swift_bridge__$Vec_ExcelMetadata$new(void);
void __swift_bridge__$Vec_ExcelMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExcelMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExcelMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExcelMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExcelMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExcelMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExcelMetadata$as_ptr(void* vec_ptr);

typedef struct EmailMetadata EmailMetadata;
void __swift_bridge__$EmailMetadata$_free(void* self);

void* __swift_bridge__$Vec_EmailMetadata$new(void);
void __swift_bridge__$Vec_EmailMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmailMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmailMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmailMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmailMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmailMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmailMetadata$as_ptr(void* vec_ptr);

typedef struct ArchiveMetadata ArchiveMetadata;
void __swift_bridge__$ArchiveMetadata$_free(void* self);

void* __swift_bridge__$Vec_ArchiveMetadata$new(void);
void __swift_bridge__$Vec_ArchiveMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_ArchiveMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ArchiveMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ArchiveMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ArchiveMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ArchiveMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_ArchiveMetadata$as_ptr(void* vec_ptr);

typedef struct XmlMetadata XmlMetadata;
void __swift_bridge__$XmlMetadata$_free(void* self);

void* __swift_bridge__$Vec_XmlMetadata$new(void);
void __swift_bridge__$Vec_XmlMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_XmlMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_XmlMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_XmlMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_XmlMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_XmlMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_XmlMetadata$as_ptr(void* vec_ptr);

typedef struct TextMetadata TextMetadata;
void __swift_bridge__$TextMetadata$_free(void* self);

void* __swift_bridge__$Vec_TextMetadata$new(void);
void __swift_bridge__$Vec_TextMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_TextMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TextMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TextMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TextMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TextMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_TextMetadata$as_ptr(void* vec_ptr);

typedef struct HeaderMetadata HeaderMetadata;
void __swift_bridge__$HeaderMetadata$_free(void* self);

void* __swift_bridge__$Vec_HeaderMetadata$new(void);
void __swift_bridge__$Vec_HeaderMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_HeaderMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HeaderMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HeaderMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HeaderMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HeaderMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_HeaderMetadata$as_ptr(void* vec_ptr);

typedef struct LinkMetadata LinkMetadata;
void __swift_bridge__$LinkMetadata$_free(void* self);

void* __swift_bridge__$Vec_LinkMetadata$new(void);
void __swift_bridge__$Vec_LinkMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_LinkMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LinkMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LinkMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LinkMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LinkMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_LinkMetadata$as_ptr(void* vec_ptr);

typedef struct ImageMetadataType ImageMetadataType;
void __swift_bridge__$ImageMetadataType$_free(void* self);

void* __swift_bridge__$Vec_ImageMetadataType$new(void);
void __swift_bridge__$Vec_ImageMetadataType$drop(void* vec_ptr);
void __swift_bridge__$Vec_ImageMetadataType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ImageMetadataType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ImageMetadataType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ImageMetadataType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ImageMetadataType$len(void* vec_ptr);
void* __swift_bridge__$Vec_ImageMetadataType$as_ptr(void* vec_ptr);

typedef struct StructuredData StructuredData;
void __swift_bridge__$StructuredData$_free(void* self);

void* __swift_bridge__$Vec_StructuredData$new(void);
void __swift_bridge__$Vec_StructuredData$drop(void* vec_ptr);
void __swift_bridge__$Vec_StructuredData$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_StructuredData$pop(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredData$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_StructuredData$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_StructuredData$len(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredData$as_ptr(void* vec_ptr);

typedef struct HtmlMetadata HtmlMetadata;
void __swift_bridge__$HtmlMetadata$_free(void* self);

void* __swift_bridge__$Vec_HtmlMetadata$new(void);
void __swift_bridge__$Vec_HtmlMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_HtmlMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HtmlMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HtmlMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HtmlMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HtmlMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_HtmlMetadata$as_ptr(void* vec_ptr);

typedef struct OcrMetadata OcrMetadata;
void __swift_bridge__$OcrMetadata$_free(void* self);

void* __swift_bridge__$Vec_OcrMetadata$new(void);
void __swift_bridge__$Vec_OcrMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrMetadata$as_ptr(void* vec_ptr);

typedef struct ErrorMetadata ErrorMetadata;
void __swift_bridge__$ErrorMetadata$_free(void* self);

void* __swift_bridge__$Vec_ErrorMetadata$new(void);
void __swift_bridge__$Vec_ErrorMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_ErrorMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ErrorMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ErrorMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ErrorMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ErrorMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_ErrorMetadata$as_ptr(void* vec_ptr);

typedef struct PptxMetadata PptxMetadata;
void __swift_bridge__$PptxMetadata$_free(void* self);

void* __swift_bridge__$Vec_PptxMetadata$new(void);
void __swift_bridge__$Vec_PptxMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_PptxMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PptxMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PptxMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PptxMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PptxMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_PptxMetadata$as_ptr(void* vec_ptr);

typedef struct DocxMetadata DocxMetadata;
void __swift_bridge__$DocxMetadata$_free(void* self);

void* __swift_bridge__$Vec_DocxMetadata$new(void);
void __swift_bridge__$Vec_DocxMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_DocxMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DocxMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DocxMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DocxMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DocxMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_DocxMetadata$as_ptr(void* vec_ptr);

typedef struct CsvMetadata CsvMetadata;
void __swift_bridge__$CsvMetadata$_free(void* self);

void* __swift_bridge__$Vec_CsvMetadata$new(void);
void __swift_bridge__$Vec_CsvMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_CsvMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_CsvMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_CsvMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_CsvMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_CsvMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_CsvMetadata$as_ptr(void* vec_ptr);

typedef struct BibtexMetadata BibtexMetadata;
void __swift_bridge__$BibtexMetadata$_free(void* self);

void* __swift_bridge__$Vec_BibtexMetadata$new(void);
void __swift_bridge__$Vec_BibtexMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_BibtexMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_BibtexMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_BibtexMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_BibtexMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_BibtexMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_BibtexMetadata$as_ptr(void* vec_ptr);

typedef struct CitationMetadata CitationMetadata;
void __swift_bridge__$CitationMetadata$_free(void* self);

void* __swift_bridge__$Vec_CitationMetadata$new(void);
void __swift_bridge__$Vec_CitationMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_CitationMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_CitationMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_CitationMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_CitationMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_CitationMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_CitationMetadata$as_ptr(void* vec_ptr);

typedef struct YearRange YearRange;
void __swift_bridge__$YearRange$_free(void* self);

void* __swift_bridge__$Vec_YearRange$new(void);
void __swift_bridge__$Vec_YearRange$drop(void* vec_ptr);
void __swift_bridge__$Vec_YearRange$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_YearRange$pop(void* vec_ptr);
void* __swift_bridge__$Vec_YearRange$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_YearRange$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_YearRange$len(void* vec_ptr);
void* __swift_bridge__$Vec_YearRange$as_ptr(void* vec_ptr);

typedef struct FictionBookMetadata FictionBookMetadata;
void __swift_bridge__$FictionBookMetadata$_free(void* self);

void* __swift_bridge__$Vec_FictionBookMetadata$new(void);
void __swift_bridge__$Vec_FictionBookMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_FictionBookMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_FictionBookMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_FictionBookMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_FictionBookMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_FictionBookMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_FictionBookMetadata$as_ptr(void* vec_ptr);

typedef struct DbfMetadata DbfMetadata;
void __swift_bridge__$DbfMetadata$_free(void* self);

void* __swift_bridge__$Vec_DbfMetadata$new(void);
void __swift_bridge__$Vec_DbfMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_DbfMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DbfMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DbfMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DbfMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DbfMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_DbfMetadata$as_ptr(void* vec_ptr);

typedef struct DbfFieldInfo DbfFieldInfo;
void __swift_bridge__$DbfFieldInfo$_free(void* self);

void* __swift_bridge__$Vec_DbfFieldInfo$new(void);
void __swift_bridge__$Vec_DbfFieldInfo$drop(void* vec_ptr);
void __swift_bridge__$Vec_DbfFieldInfo$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DbfFieldInfo$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DbfFieldInfo$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DbfFieldInfo$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DbfFieldInfo$len(void* vec_ptr);
void* __swift_bridge__$Vec_DbfFieldInfo$as_ptr(void* vec_ptr);

typedef struct JatsMetadata JatsMetadata;
void __swift_bridge__$JatsMetadata$_free(void* self);

void* __swift_bridge__$Vec_JatsMetadata$new(void);
void __swift_bridge__$Vec_JatsMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_JatsMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_JatsMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_JatsMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_JatsMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_JatsMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_JatsMetadata$as_ptr(void* vec_ptr);

typedef struct ContributorRole ContributorRole;
void __swift_bridge__$ContributorRole$_free(void* self);

void* __swift_bridge__$Vec_ContributorRole$new(void);
void __swift_bridge__$Vec_ContributorRole$drop(void* vec_ptr);
void __swift_bridge__$Vec_ContributorRole$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ContributorRole$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ContributorRole$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ContributorRole$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ContributorRole$len(void* vec_ptr);
void* __swift_bridge__$Vec_ContributorRole$as_ptr(void* vec_ptr);

typedef struct EpubMetadata EpubMetadata;
void __swift_bridge__$EpubMetadata$_free(void* self);

void* __swift_bridge__$Vec_EpubMetadata$new(void);
void __swift_bridge__$Vec_EpubMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_EpubMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EpubMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EpubMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EpubMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EpubMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_EpubMetadata$as_ptr(void* vec_ptr);

typedef struct PstMetadata PstMetadata;
void __swift_bridge__$PstMetadata$_free(void* self);

void* __swift_bridge__$Vec_PstMetadata$new(void);
void __swift_bridge__$Vec_PstMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_PstMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PstMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PstMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PstMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PstMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_PstMetadata$as_ptr(void* vec_ptr);

typedef struct OcrConfidence OcrConfidence;
void __swift_bridge__$OcrConfidence$_free(void* self);

void* __swift_bridge__$Vec_OcrConfidence$new(void);
void __swift_bridge__$Vec_OcrConfidence$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrConfidence$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrConfidence$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrConfidence$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrConfidence$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrConfidence$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrConfidence$as_ptr(void* vec_ptr);

typedef struct OcrRotation OcrRotation;
void __swift_bridge__$OcrRotation$_free(void* self);

void* __swift_bridge__$Vec_OcrRotation$new(void);
void __swift_bridge__$Vec_OcrRotation$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrRotation$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrRotation$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrRotation$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrRotation$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrRotation$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrRotation$as_ptr(void* vec_ptr);

typedef struct OcrElement OcrElement;
void __swift_bridge__$OcrElement$_free(void* self);

void* __swift_bridge__$Vec_OcrElement$new(void);
void __swift_bridge__$Vec_OcrElement$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrElement$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrElement$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrElement$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrElement$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrElement$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrElement$as_ptr(void* vec_ptr);

typedef struct OcrElementConfig OcrElementConfig;
void __swift_bridge__$OcrElementConfig$_free(void* self);

void* __swift_bridge__$Vec_OcrElementConfig$new(void);
void __swift_bridge__$Vec_OcrElementConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrElementConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrElementConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrElementConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrElementConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrElementConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrElementConfig$as_ptr(void* vec_ptr);

typedef struct PageStructure PageStructure;
void __swift_bridge__$PageStructure$_free(void* self);

void* __swift_bridge__$Vec_PageStructure$new(void);
void __swift_bridge__$Vec_PageStructure$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageStructure$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageStructure$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageStructure$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageStructure$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageStructure$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageStructure$as_ptr(void* vec_ptr);

typedef struct PageBoundary PageBoundary;
void __swift_bridge__$PageBoundary$_free(void* self);

void* __swift_bridge__$Vec_PageBoundary$new(void);
void __swift_bridge__$Vec_PageBoundary$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageBoundary$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageBoundary$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageBoundary$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageBoundary$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageBoundary$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageBoundary$as_ptr(void* vec_ptr);

typedef struct PageInfo PageInfo;
void __swift_bridge__$PageInfo$_free(void* self);

void* __swift_bridge__$Vec_PageInfo$new(void);
void __swift_bridge__$Vec_PageInfo$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageInfo$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageInfo$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageInfo$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageInfo$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageInfo$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageInfo$as_ptr(void* vec_ptr);

typedef struct PageContent PageContent;
void __swift_bridge__$PageContent$_free(void* self);

void* __swift_bridge__$Vec_PageContent$new(void);
void __swift_bridge__$Vec_PageContent$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageContent$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageContent$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageContent$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageContent$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageContent$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageContent$as_ptr(void* vec_ptr);

typedef struct LayoutRegion LayoutRegion;
void __swift_bridge__$LayoutRegion$_free(void* self);

void* __swift_bridge__$Vec_LayoutRegion$new(void);
void __swift_bridge__$Vec_LayoutRegion$drop(void* vec_ptr);
void __swift_bridge__$Vec_LayoutRegion$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LayoutRegion$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LayoutRegion$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LayoutRegion$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LayoutRegion$len(void* vec_ptr);
void* __swift_bridge__$Vec_LayoutRegion$as_ptr(void* vec_ptr);

typedef struct PageHierarchy PageHierarchy;
void __swift_bridge__$PageHierarchy$_free(void* self);

void* __swift_bridge__$Vec_PageHierarchy$new(void);
void __swift_bridge__$Vec_PageHierarchy$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageHierarchy$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageHierarchy$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageHierarchy$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageHierarchy$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageHierarchy$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageHierarchy$as_ptr(void* vec_ptr);

typedef struct HierarchicalBlock HierarchicalBlock;
void __swift_bridge__$HierarchicalBlock$_free(void* self);

void* __swift_bridge__$Vec_HierarchicalBlock$new(void);
void __swift_bridge__$Vec_HierarchicalBlock$drop(void* vec_ptr);
void __swift_bridge__$Vec_HierarchicalBlock$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HierarchicalBlock$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HierarchicalBlock$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HierarchicalBlock$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HierarchicalBlock$len(void* vec_ptr);
void* __swift_bridge__$Vec_HierarchicalBlock$as_ptr(void* vec_ptr);

typedef struct Uri Uri;
void __swift_bridge__$Uri$_free(void* self);

void* __swift_bridge__$Vec_Uri$new(void);
void __swift_bridge__$Vec_Uri$drop(void* vec_ptr);
void __swift_bridge__$Vec_Uri$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Uri$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Uri$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Uri$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Uri$len(void* vec_ptr);
void* __swift_bridge__$Vec_Uri$as_ptr(void* vec_ptr);

typedef struct StringBufferPool StringBufferPool;
void __swift_bridge__$StringBufferPool$_free(void* self);

void* __swift_bridge__$Vec_StringBufferPool$new(void);
void __swift_bridge__$Vec_StringBufferPool$drop(void* vec_ptr);
void __swift_bridge__$Vec_StringBufferPool$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_StringBufferPool$pop(void* vec_ptr);
void* __swift_bridge__$Vec_StringBufferPool$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_StringBufferPool$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_StringBufferPool$len(void* vec_ptr);
void* __swift_bridge__$Vec_StringBufferPool$as_ptr(void* vec_ptr);

typedef struct ByteBufferPool ByteBufferPool;
void __swift_bridge__$ByteBufferPool$_free(void* self);

void* __swift_bridge__$Vec_ByteBufferPool$new(void);
void __swift_bridge__$Vec_ByteBufferPool$drop(void* vec_ptr);
void __swift_bridge__$Vec_ByteBufferPool$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ByteBufferPool$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ByteBufferPool$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ByteBufferPool$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ByteBufferPool$len(void* vec_ptr);
void* __swift_bridge__$Vec_ByteBufferPool$as_ptr(void* vec_ptr);

typedef struct TracingLayer TracingLayer;
void __swift_bridge__$TracingLayer$_free(void* self);

void* __swift_bridge__$Vec_TracingLayer$new(void);
void __swift_bridge__$Vec_TracingLayer$drop(void* vec_ptr);
void __swift_bridge__$Vec_TracingLayer$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TracingLayer$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TracingLayer$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TracingLayer$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TracingLayer$len(void* vec_ptr);
void* __swift_bridge__$Vec_TracingLayer$as_ptr(void* vec_ptr);

typedef struct ApiDoc ApiDoc;
void __swift_bridge__$ApiDoc$_free(void* self);

void* __swift_bridge__$Vec_ApiDoc$new(void);
void __swift_bridge__$Vec_ApiDoc$drop(void* vec_ptr);
void __swift_bridge__$Vec_ApiDoc$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ApiDoc$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ApiDoc$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ApiDoc$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ApiDoc$len(void* vec_ptr);
void* __swift_bridge__$Vec_ApiDoc$as_ptr(void* vec_ptr);

typedef struct HealthResponse HealthResponse;
void __swift_bridge__$HealthResponse$_free(void* self);

void* __swift_bridge__$Vec_HealthResponse$new(void);
void __swift_bridge__$Vec_HealthResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_HealthResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HealthResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HealthResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HealthResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HealthResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_HealthResponse$as_ptr(void* vec_ptr);

typedef struct InfoResponse InfoResponse;
void __swift_bridge__$InfoResponse$_free(void* self);

void* __swift_bridge__$Vec_InfoResponse$new(void);
void __swift_bridge__$Vec_InfoResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_InfoResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_InfoResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_InfoResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_InfoResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_InfoResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_InfoResponse$as_ptr(void* vec_ptr);

typedef struct ExtractResponse ExtractResponse;
void __swift_bridge__$ExtractResponse$_free(void* self);

void* __swift_bridge__$Vec_ExtractResponse$new(void);
void __swift_bridge__$Vec_ExtractResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExtractResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExtractResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExtractResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExtractResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractResponse$as_ptr(void* vec_ptr);

typedef struct ApiState ApiState;
void __swift_bridge__$ApiState$_free(void* self);

void* __swift_bridge__$Vec_ApiState$new(void);
void __swift_bridge__$Vec_ApiState$drop(void* vec_ptr);
void __swift_bridge__$Vec_ApiState$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ApiState$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ApiState$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ApiState$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ApiState$len(void* vec_ptr);
void* __swift_bridge__$Vec_ApiState$as_ptr(void* vec_ptr);

typedef struct CacheStatsResponse CacheStatsResponse;
void __swift_bridge__$CacheStatsResponse$_free(void* self);

void* __swift_bridge__$Vec_CacheStatsResponse$new(void);
void __swift_bridge__$Vec_CacheStatsResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_CacheStatsResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_CacheStatsResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_CacheStatsResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_CacheStatsResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_CacheStatsResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_CacheStatsResponse$as_ptr(void* vec_ptr);

typedef struct CacheClearResponse CacheClearResponse;
void __swift_bridge__$CacheClearResponse$_free(void* self);

void* __swift_bridge__$Vec_CacheClearResponse$new(void);
void __swift_bridge__$Vec_CacheClearResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_CacheClearResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_CacheClearResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_CacheClearResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_CacheClearResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_CacheClearResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_CacheClearResponse$as_ptr(void* vec_ptr);

typedef struct EmbedRequest EmbedRequest;
void __swift_bridge__$EmbedRequest$_free(void* self);

void* __swift_bridge__$Vec_EmbedRequest$new(void);
void __swift_bridge__$Vec_EmbedRequest$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmbedRequest$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmbedRequest$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmbedRequest$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmbedRequest$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmbedRequest$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmbedRequest$as_ptr(void* vec_ptr);

typedef struct EmbedResponse EmbedResponse;
void __swift_bridge__$EmbedResponse$_free(void* self);

void* __swift_bridge__$Vec_EmbedResponse$new(void);
void __swift_bridge__$Vec_EmbedResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmbedResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmbedResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmbedResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmbedResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmbedResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmbedResponse$as_ptr(void* vec_ptr);

typedef struct ChunkRequest ChunkRequest;
void __swift_bridge__$ChunkRequest$_free(void* self);

void* __swift_bridge__$Vec_ChunkRequest$new(void);
void __swift_bridge__$Vec_ChunkRequest$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkRequest$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkRequest$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkRequest$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkRequest$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkRequest$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkRequest$as_ptr(void* vec_ptr);

typedef struct ChunkResponse ChunkResponse;
void __swift_bridge__$ChunkResponse$_free(void* self);

void* __swift_bridge__$Vec_ChunkResponse$new(void);
void __swift_bridge__$Vec_ChunkResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkResponse$as_ptr(void* vec_ptr);

typedef struct VersionResponse VersionResponse;
void __swift_bridge__$VersionResponse$_free(void* self);

void* __swift_bridge__$Vec_VersionResponse$new(void);
void __swift_bridge__$Vec_VersionResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_VersionResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_VersionResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_VersionResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_VersionResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_VersionResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_VersionResponse$as_ptr(void* vec_ptr);

typedef struct DetectResponse DetectResponse;
void __swift_bridge__$DetectResponse$_free(void* self);

void* __swift_bridge__$Vec_DetectResponse$new(void);
void __swift_bridge__$Vec_DetectResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_DetectResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DetectResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DetectResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DetectResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DetectResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_DetectResponse$as_ptr(void* vec_ptr);

typedef struct ManifestEntryResponse ManifestEntryResponse;
void __swift_bridge__$ManifestEntryResponse$_free(void* self);

void* __swift_bridge__$Vec_ManifestEntryResponse$new(void);
void __swift_bridge__$Vec_ManifestEntryResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_ManifestEntryResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ManifestEntryResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ManifestEntryResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ManifestEntryResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ManifestEntryResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_ManifestEntryResponse$as_ptr(void* vec_ptr);

typedef struct ManifestResponse ManifestResponse;
void __swift_bridge__$ManifestResponse$_free(void* self);

void* __swift_bridge__$Vec_ManifestResponse$new(void);
void __swift_bridge__$Vec_ManifestResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_ManifestResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ManifestResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ManifestResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ManifestResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ManifestResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_ManifestResponse$as_ptr(void* vec_ptr);

typedef struct WarmRequest WarmRequest;
void __swift_bridge__$WarmRequest$_free(void* self);

void* __swift_bridge__$Vec_WarmRequest$new(void);
void __swift_bridge__$Vec_WarmRequest$drop(void* vec_ptr);
void __swift_bridge__$Vec_WarmRequest$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_WarmRequest$pop(void* vec_ptr);
void* __swift_bridge__$Vec_WarmRequest$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_WarmRequest$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_WarmRequest$len(void* vec_ptr);
void* __swift_bridge__$Vec_WarmRequest$as_ptr(void* vec_ptr);

typedef struct WarmResponse WarmResponse;
void __swift_bridge__$WarmResponse$_free(void* self);

void* __swift_bridge__$Vec_WarmResponse$new(void);
void __swift_bridge__$Vec_WarmResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_WarmResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_WarmResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_WarmResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_WarmResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_WarmResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_WarmResponse$as_ptr(void* vec_ptr);

typedef struct StructuredExtractionResponse StructuredExtractionResponse;
void __swift_bridge__$StructuredExtractionResponse$_free(void* self);

void* __swift_bridge__$Vec_StructuredExtractionResponse$new(void);
void __swift_bridge__$Vec_StructuredExtractionResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_StructuredExtractionResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_StructuredExtractionResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredExtractionResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_StructuredExtractionResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_StructuredExtractionResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredExtractionResponse$as_ptr(void* vec_ptr);

typedef struct OpenWebDocumentResponse OpenWebDocumentResponse;
void __swift_bridge__$OpenWebDocumentResponse$_free(void* self);

void* __swift_bridge__$Vec_OpenWebDocumentResponse$new(void);
void __swift_bridge__$Vec_OpenWebDocumentResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_OpenWebDocumentResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OpenWebDocumentResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OpenWebDocumentResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OpenWebDocumentResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OpenWebDocumentResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_OpenWebDocumentResponse$as_ptr(void* vec_ptr);

typedef struct DoclingCompatResponse DoclingCompatResponse;
void __swift_bridge__$DoclingCompatResponse$_free(void* self);

void* __swift_bridge__$Vec_DoclingCompatResponse$new(void);
void __swift_bridge__$Vec_DoclingCompatResponse$drop(void* vec_ptr);
void __swift_bridge__$Vec_DoclingCompatResponse$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DoclingCompatResponse$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DoclingCompatResponse$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DoclingCompatResponse$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DoclingCompatResponse$len(void* vec_ptr);
void* __swift_bridge__$Vec_DoclingCompatResponse$as_ptr(void* vec_ptr);

typedef struct ExtractFileParams ExtractFileParams;
void __swift_bridge__$ExtractFileParams$_free(void* self);

void* __swift_bridge__$Vec_ExtractFileParams$new(void);
void __swift_bridge__$Vec_ExtractFileParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExtractFileParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExtractFileParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractFileParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExtractFileParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExtractFileParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractFileParams$as_ptr(void* vec_ptr);

typedef struct ExtractBytesParams ExtractBytesParams;
void __swift_bridge__$ExtractBytesParams$_free(void* self);

void* __swift_bridge__$Vec_ExtractBytesParams$new(void);
void __swift_bridge__$Vec_ExtractBytesParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExtractBytesParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExtractBytesParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractBytesParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExtractBytesParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExtractBytesParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractBytesParams$as_ptr(void* vec_ptr);

typedef struct BatchExtractFilesParams BatchExtractFilesParams;
void __swift_bridge__$BatchExtractFilesParams$_free(void* self);

void* __swift_bridge__$Vec_BatchExtractFilesParams$new(void);
void __swift_bridge__$Vec_BatchExtractFilesParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_BatchExtractFilesParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_BatchExtractFilesParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_BatchExtractFilesParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_BatchExtractFilesParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_BatchExtractFilesParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_BatchExtractFilesParams$as_ptr(void* vec_ptr);

typedef struct DetectMimeTypeParams DetectMimeTypeParams;
void __swift_bridge__$DetectMimeTypeParams$_free(void* self);

void* __swift_bridge__$Vec_DetectMimeTypeParams$new(void);
void __swift_bridge__$Vec_DetectMimeTypeParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_DetectMimeTypeParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DetectMimeTypeParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DetectMimeTypeParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DetectMimeTypeParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DetectMimeTypeParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_DetectMimeTypeParams$as_ptr(void* vec_ptr);

typedef struct CacheWarmParams CacheWarmParams;
void __swift_bridge__$CacheWarmParams$_free(void* self);

void* __swift_bridge__$Vec_CacheWarmParams$new(void);
void __swift_bridge__$Vec_CacheWarmParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_CacheWarmParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_CacheWarmParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_CacheWarmParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_CacheWarmParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_CacheWarmParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_CacheWarmParams$as_ptr(void* vec_ptr);

typedef struct EmbedTextParams EmbedTextParams;
void __swift_bridge__$EmbedTextParams$_free(void* self);

void* __swift_bridge__$Vec_EmbedTextParams$new(void);
void __swift_bridge__$Vec_EmbedTextParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmbedTextParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmbedTextParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmbedTextParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmbedTextParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmbedTextParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmbedTextParams$as_ptr(void* vec_ptr);

typedef struct ExtractStructuredParams ExtractStructuredParams;
void __swift_bridge__$ExtractStructuredParams$_free(void* self);

void* __swift_bridge__$Vec_ExtractStructuredParams$new(void);
void __swift_bridge__$Vec_ExtractStructuredParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExtractStructuredParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExtractStructuredParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractStructuredParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExtractStructuredParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExtractStructuredParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExtractStructuredParams$as_ptr(void* vec_ptr);

typedef struct ChunkTextParams ChunkTextParams;
void __swift_bridge__$ChunkTextParams$_free(void* self);

void* __swift_bridge__$Vec_ChunkTextParams$new(void);
void __swift_bridge__$Vec_ChunkTextParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkTextParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkTextParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkTextParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkTextParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkTextParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkTextParams$as_ptr(void* vec_ptr);

typedef struct DetectedBoundary DetectedBoundary;
void __swift_bridge__$DetectedBoundary$_free(void* self);

void* __swift_bridge__$Vec_DetectedBoundary$new(void);
void __swift_bridge__$Vec_DetectedBoundary$drop(void* vec_ptr);
void __swift_bridge__$Vec_DetectedBoundary$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DetectedBoundary$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DetectedBoundary$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DetectedBoundary$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DetectedBoundary$len(void* vec_ptr);
void* __swift_bridge__$Vec_DetectedBoundary$as_ptr(void* vec_ptr);

typedef struct ChunkingResult ChunkingResult;
void __swift_bridge__$ChunkingResult$_free(void* self);

void* __swift_bridge__$Vec_ChunkingResult$new(void);
void __swift_bridge__$Vec_ChunkingResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkingResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkingResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkingResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkingResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkingResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkingResult$as_ptr(void* vec_ptr);

typedef struct MergedChunk MergedChunk;
void __swift_bridge__$MergedChunk$_free(void* self);

void* __swift_bridge__$Vec_MergedChunk$new(void);
void __swift_bridge__$Vec_MergedChunk$drop(void* vec_ptr);
void __swift_bridge__$Vec_MergedChunk$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_MergedChunk$pop(void* vec_ptr);
void* __swift_bridge__$Vec_MergedChunk$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_MergedChunk$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_MergedChunk$len(void* vec_ptr);
void* __swift_bridge__$Vec_MergedChunk$as_ptr(void* vec_ptr);

typedef struct YakeParams YakeParams;
void __swift_bridge__$YakeParams$_free(void* self);

void* __swift_bridge__$Vec_YakeParams$new(void);
void __swift_bridge__$Vec_YakeParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_YakeParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_YakeParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_YakeParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_YakeParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_YakeParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_YakeParams$as_ptr(void* vec_ptr);

typedef struct RakeParams RakeParams;
void __swift_bridge__$RakeParams$_free(void* self);

void* __swift_bridge__$Vec_RakeParams$new(void);
void __swift_bridge__$Vec_RakeParams$drop(void* vec_ptr);
void __swift_bridge__$Vec_RakeParams$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_RakeParams$pop(void* vec_ptr);
void* __swift_bridge__$Vec_RakeParams$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_RakeParams$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_RakeParams$len(void* vec_ptr);
void* __swift_bridge__$Vec_RakeParams$as_ptr(void* vec_ptr);

typedef struct KeywordConfig KeywordConfig;
void __swift_bridge__$KeywordConfig$_free(void* self);

void* __swift_bridge__$Vec_KeywordConfig$new(void);
void __swift_bridge__$Vec_KeywordConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_KeywordConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_KeywordConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_KeywordConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_KeywordConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_KeywordConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_KeywordConfig$as_ptr(void* vec_ptr);

typedef struct Keyword Keyword;
void __swift_bridge__$Keyword$_free(void* self);

void* __swift_bridge__$Vec_Keyword$new(void);
void __swift_bridge__$Vec_Keyword$drop(void* vec_ptr);
void __swift_bridge__$Vec_Keyword$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Keyword$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Keyword$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Keyword$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Keyword$len(void* vec_ptr);
void* __swift_bridge__$Vec_Keyword$as_ptr(void* vec_ptr);

typedef struct OcrCacheStats OcrCacheStats;
void __swift_bridge__$OcrCacheStats$_free(void* self);

void* __swift_bridge__$Vec_OcrCacheStats$new(void);
void __swift_bridge__$Vec_OcrCacheStats$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrCacheStats$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrCacheStats$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrCacheStats$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrCacheStats$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrCacheStats$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrCacheStats$as_ptr(void* vec_ptr);

typedef struct RecognizedTable RecognizedTable;
void __swift_bridge__$RecognizedTable$_free(void* self);

void* __swift_bridge__$Vec_RecognizedTable$new(void);
void __swift_bridge__$Vec_RecognizedTable$drop(void* vec_ptr);
void __swift_bridge__$Vec_RecognizedTable$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_RecognizedTable$pop(void* vec_ptr);
void* __swift_bridge__$Vec_RecognizedTable$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_RecognizedTable$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_RecognizedTable$len(void* vec_ptr);
void* __swift_bridge__$Vec_RecognizedTable$as_ptr(void* vec_ptr);

typedef struct TessdataManager TessdataManager;
void __swift_bridge__$TessdataManager$_free(void* self);

void* __swift_bridge__$Vec_TessdataManager$new(void);
void __swift_bridge__$Vec_TessdataManager$drop(void* vec_ptr);
void __swift_bridge__$Vec_TessdataManager$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TessdataManager$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TessdataManager$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TessdataManager$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TessdataManager$len(void* vec_ptr);
void* __swift_bridge__$Vec_TessdataManager$as_ptr(void* vec_ptr);

typedef struct PaddleOcrConfig PaddleOcrConfig;
void __swift_bridge__$PaddleOcrConfig$_free(void* self);

void* __swift_bridge__$Vec_PaddleOcrConfig$new(void);
void __swift_bridge__$Vec_PaddleOcrConfig$drop(void* vec_ptr);
void __swift_bridge__$Vec_PaddleOcrConfig$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PaddleOcrConfig$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PaddleOcrConfig$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PaddleOcrConfig$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PaddleOcrConfig$len(void* vec_ptr);
void* __swift_bridge__$Vec_PaddleOcrConfig$as_ptr(void* vec_ptr);

typedef struct ModelPaths ModelPaths;
void __swift_bridge__$ModelPaths$_free(void* self);

void* __swift_bridge__$Vec_ModelPaths$new(void);
void __swift_bridge__$Vec_ModelPaths$drop(void* vec_ptr);
void __swift_bridge__$Vec_ModelPaths$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ModelPaths$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ModelPaths$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ModelPaths$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ModelPaths$len(void* vec_ptr);
void* __swift_bridge__$Vec_ModelPaths$as_ptr(void* vec_ptr);

typedef struct OrientationResult OrientationResult;
void __swift_bridge__$OrientationResult$_free(void* self);

void* __swift_bridge__$Vec_OrientationResult$new(void);
void __swift_bridge__$Vec_OrientationResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_OrientationResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OrientationResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OrientationResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OrientationResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OrientationResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_OrientationResult$as_ptr(void* vec_ptr);

typedef struct BBox BBox;
void __swift_bridge__$BBox$_free(void* self);

void* __swift_bridge__$Vec_BBox$new(void);
void __swift_bridge__$Vec_BBox$drop(void* vec_ptr);
void __swift_bridge__$Vec_BBox$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_BBox$pop(void* vec_ptr);
void* __swift_bridge__$Vec_BBox$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_BBox$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_BBox$len(void* vec_ptr);
void* __swift_bridge__$Vec_BBox$as_ptr(void* vec_ptr);

typedef struct LayoutDetection LayoutDetection;
void __swift_bridge__$LayoutDetection$_free(void* self);

void* __swift_bridge__$Vec_LayoutDetection$new(void);
void __swift_bridge__$Vec_LayoutDetection$drop(void* vec_ptr);
void __swift_bridge__$Vec_LayoutDetection$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LayoutDetection$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LayoutDetection$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LayoutDetection$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LayoutDetection$len(void* vec_ptr);
void* __swift_bridge__$Vec_LayoutDetection$as_ptr(void* vec_ptr);

typedef struct DetectionResult DetectionResult;
void __swift_bridge__$DetectionResult$_free(void* self);

void* __swift_bridge__$Vec_DetectionResult$new(void);
void __swift_bridge__$Vec_DetectionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_DetectionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_DetectionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_DetectionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_DetectionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_DetectionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_DetectionResult$as_ptr(void* vec_ptr);

typedef struct EmbeddedFile EmbeddedFile;
void __swift_bridge__$EmbeddedFile$_free(void* self);

void* __swift_bridge__$Vec_EmbeddedFile$new(void);
void __swift_bridge__$Vec_EmbeddedFile$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmbeddedFile$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmbeddedFile$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmbeddedFile$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmbeddedFile$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmbeddedFile$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmbeddedFile$as_ptr(void* vec_ptr);

typedef struct PdfImage PdfImage;
void __swift_bridge__$PdfImage$_free(void* self);

void* __swift_bridge__$Vec_PdfImage$new(void);
void __swift_bridge__$Vec_PdfImage$drop(void* vec_ptr);
void __swift_bridge__$Vec_PdfImage$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PdfImage$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PdfImage$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PdfImage$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PdfImage$len(void* vec_ptr);
void* __swift_bridge__$Vec_PdfImage$as_ptr(void* vec_ptr);

typedef struct PageLayoutResult PageLayoutResult;
void __swift_bridge__$PageLayoutResult$_free(void* self);

void* __swift_bridge__$Vec_PageLayoutResult$new(void);
void __swift_bridge__$Vec_PageLayoutResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageLayoutResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageLayoutResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageLayoutResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageLayoutResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageLayoutResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageLayoutResult$as_ptr(void* vec_ptr);

typedef struct PageTiming PageTiming;
void __swift_bridge__$PageTiming$_free(void* self);

void* __swift_bridge__$Vec_PageTiming$new(void);
void __swift_bridge__$Vec_PageTiming$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageTiming$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageTiming$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageTiming$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageTiming$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageTiming$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageTiming$as_ptr(void* vec_ptr);

typedef struct CommonPdfMetadata CommonPdfMetadata;
void __swift_bridge__$CommonPdfMetadata$_free(void* self);

void* __swift_bridge__$Vec_CommonPdfMetadata$new(void);
void __swift_bridge__$Vec_CommonPdfMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_CommonPdfMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_CommonPdfMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_CommonPdfMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_CommonPdfMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_CommonPdfMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_CommonPdfMetadata$as_ptr(void* vec_ptr);

typedef struct PdfUnifiedExtractionResult PdfUnifiedExtractionResult;
void __swift_bridge__$PdfUnifiedExtractionResult$_free(void* self);

void* __swift_bridge__$Vec_PdfUnifiedExtractionResult$new(void);
void __swift_bridge__$Vec_PdfUnifiedExtractionResult$drop(void* vec_ptr);
void __swift_bridge__$Vec_PdfUnifiedExtractionResult$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PdfUnifiedExtractionResult$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PdfUnifiedExtractionResult$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PdfUnifiedExtractionResult$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PdfUnifiedExtractionResult$len(void* vec_ptr);
void* __swift_bridge__$Vec_PdfUnifiedExtractionResult$as_ptr(void* vec_ptr);

typedef struct ExecutionProviderType ExecutionProviderType;
void __swift_bridge__$ExecutionProviderType$_free(void* self);

void* __swift_bridge__$Vec_ExecutionProviderType$new(void);
void __swift_bridge__$Vec_ExecutionProviderType$drop(void* vec_ptr);
void __swift_bridge__$Vec_ExecutionProviderType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ExecutionProviderType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ExecutionProviderType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ExecutionProviderType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ExecutionProviderType$len(void* vec_ptr);
void* __swift_bridge__$Vec_ExecutionProviderType$as_ptr(void* vec_ptr);

typedef struct HtmlTheme HtmlTheme;
void __swift_bridge__$HtmlTheme$_free(void* self);

void* __swift_bridge__$Vec_HtmlTheme$new(void);
void __swift_bridge__$Vec_HtmlTheme$drop(void* vec_ptr);
void __swift_bridge__$Vec_HtmlTheme$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_HtmlTheme$pop(void* vec_ptr);
void* __swift_bridge__$Vec_HtmlTheme$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_HtmlTheme$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_HtmlTheme$len(void* vec_ptr);
void* __swift_bridge__$Vec_HtmlTheme$as_ptr(void* vec_ptr);

typedef struct TableModel TableModel;
void __swift_bridge__$TableModel$_free(void* self);

void* __swift_bridge__$Vec_TableModel$new(void);
void __swift_bridge__$Vec_TableModel$drop(void* vec_ptr);
void __swift_bridge__$Vec_TableModel$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TableModel$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TableModel$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TableModel$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TableModel$len(void* vec_ptr);
void* __swift_bridge__$Vec_TableModel$as_ptr(void* vec_ptr);

typedef struct PdfBackend PdfBackend;
void __swift_bridge__$PdfBackend$_free(void* self);

void* __swift_bridge__$Vec_PdfBackend$new(void);
void __swift_bridge__$Vec_PdfBackend$drop(void* vec_ptr);
void __swift_bridge__$Vec_PdfBackend$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PdfBackend$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PdfBackend$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PdfBackend$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PdfBackend$len(void* vec_ptr);
void* __swift_bridge__$Vec_PdfBackend$as_ptr(void* vec_ptr);

typedef struct ChunkerType ChunkerType;
void __swift_bridge__$ChunkerType$_free(void* self);

void* __swift_bridge__$Vec_ChunkerType$new(void);
void __swift_bridge__$Vec_ChunkerType$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkerType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkerType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkerType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkerType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkerType$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkerType$as_ptr(void* vec_ptr);

typedef struct ChunkSizing ChunkSizing;
void __swift_bridge__$ChunkSizing$_free(void* self);

void* __swift_bridge__$Vec_ChunkSizing$new(void);
void __swift_bridge__$Vec_ChunkSizing$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkSizing$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkSizing$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkSizing$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkSizing$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkSizing$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkSizing$as_ptr(void* vec_ptr);

typedef struct EmbeddingModelType EmbeddingModelType;
void __swift_bridge__$EmbeddingModelType$_free(void* self);

void* __swift_bridge__$Vec_EmbeddingModelType$new(void);
void __swift_bridge__$Vec_EmbeddingModelType$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmbeddingModelType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmbeddingModelType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmbeddingModelType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmbeddingModelType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmbeddingModelType$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmbeddingModelType$as_ptr(void* vec_ptr);

typedef struct CodeContentMode CodeContentMode;
void __swift_bridge__$CodeContentMode$_free(void* self);

void* __swift_bridge__$Vec_CodeContentMode$new(void);
void __swift_bridge__$Vec_CodeContentMode$drop(void* vec_ptr);
void __swift_bridge__$Vec_CodeContentMode$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_CodeContentMode$pop(void* vec_ptr);
void* __swift_bridge__$Vec_CodeContentMode$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_CodeContentMode$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_CodeContentMode$len(void* vec_ptr);
void* __swift_bridge__$Vec_CodeContentMode$as_ptr(void* vec_ptr);

typedef struct FracType FracType;
void __swift_bridge__$FracType$_free(void* self);

void* __swift_bridge__$Vec_FracType$new(void);
void __swift_bridge__$Vec_FracType$drop(void* vec_ptr);
void __swift_bridge__$Vec_FracType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_FracType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_FracType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_FracType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_FracType$len(void* vec_ptr);
void* __swift_bridge__$Vec_FracType$as_ptr(void* vec_ptr);

typedef struct OcrBackendType OcrBackendType;
void __swift_bridge__$OcrBackendType$_free(void* self);

void* __swift_bridge__$Vec_OcrBackendType$new(void);
void __swift_bridge__$Vec_OcrBackendType$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrBackendType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrBackendType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrBackendType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrBackendType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrBackendType$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrBackendType$as_ptr(void* vec_ptr);

typedef struct ProcessingStage ProcessingStage;
void __swift_bridge__$ProcessingStage$_free(void* self);

void* __swift_bridge__$Vec_ProcessingStage$new(void);
void __swift_bridge__$Vec_ProcessingStage$drop(void* vec_ptr);
void __swift_bridge__$Vec_ProcessingStage$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ProcessingStage$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ProcessingStage$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ProcessingStage$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ProcessingStage$len(void* vec_ptr);
void* __swift_bridge__$Vec_ProcessingStage$as_ptr(void* vec_ptr);

typedef struct ReductionLevel ReductionLevel;
void __swift_bridge__$ReductionLevel$_free(void* self);

void* __swift_bridge__$Vec_ReductionLevel$new(void);
void __swift_bridge__$Vec_ReductionLevel$drop(void* vec_ptr);
void __swift_bridge__$Vec_ReductionLevel$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ReductionLevel$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ReductionLevel$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ReductionLevel$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ReductionLevel$len(void* vec_ptr);
void* __swift_bridge__$Vec_ReductionLevel$as_ptr(void* vec_ptr);

typedef struct PdfAnnotationType PdfAnnotationType;
void __swift_bridge__$PdfAnnotationType$_free(void* self);

void* __swift_bridge__$Vec_PdfAnnotationType$new(void);
void __swift_bridge__$Vec_PdfAnnotationType$drop(void* vec_ptr);
void __swift_bridge__$Vec_PdfAnnotationType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PdfAnnotationType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PdfAnnotationType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PdfAnnotationType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PdfAnnotationType$len(void* vec_ptr);
void* __swift_bridge__$Vec_PdfAnnotationType$as_ptr(void* vec_ptr);

typedef struct BlockType BlockType;
void __swift_bridge__$BlockType$_free(void* self);

void* __swift_bridge__$Vec_BlockType$new(void);
void __swift_bridge__$Vec_BlockType$drop(void* vec_ptr);
void __swift_bridge__$Vec_BlockType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_BlockType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_BlockType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_BlockType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_BlockType$len(void* vec_ptr);
void* __swift_bridge__$Vec_BlockType$as_ptr(void* vec_ptr);

typedef struct InlineType InlineType;
void __swift_bridge__$InlineType$_free(void* self);

void* __swift_bridge__$Vec_InlineType$new(void);
void __swift_bridge__$Vec_InlineType$drop(void* vec_ptr);
void __swift_bridge__$Vec_InlineType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_InlineType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_InlineType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_InlineType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_InlineType$len(void* vec_ptr);
void* __swift_bridge__$Vec_InlineType$as_ptr(void* vec_ptr);

typedef struct RelationshipKind RelationshipKind;
void __swift_bridge__$RelationshipKind$_free(void* self);

void* __swift_bridge__$Vec_RelationshipKind$new(void);
void __swift_bridge__$Vec_RelationshipKind$drop(void* vec_ptr);
void __swift_bridge__$Vec_RelationshipKind$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_RelationshipKind$pop(void* vec_ptr);
void* __swift_bridge__$Vec_RelationshipKind$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_RelationshipKind$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_RelationshipKind$len(void* vec_ptr);
void* __swift_bridge__$Vec_RelationshipKind$as_ptr(void* vec_ptr);

typedef struct ContentLayer ContentLayer;
void __swift_bridge__$ContentLayer$_free(void* self);

void* __swift_bridge__$Vec_ContentLayer$new(void);
void __swift_bridge__$Vec_ContentLayer$drop(void* vec_ptr);
void __swift_bridge__$Vec_ContentLayer$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ContentLayer$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ContentLayer$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ContentLayer$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ContentLayer$len(void* vec_ptr);
void* __swift_bridge__$Vec_ContentLayer$as_ptr(void* vec_ptr);

typedef struct NodeContent NodeContent;
void __swift_bridge__$NodeContent$_free(void* self);

void* __swift_bridge__$Vec_NodeContent$new(void);
void __swift_bridge__$Vec_NodeContent$drop(void* vec_ptr);
void __swift_bridge__$Vec_NodeContent$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_NodeContent$pop(void* vec_ptr);
void* __swift_bridge__$Vec_NodeContent$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_NodeContent$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_NodeContent$len(void* vec_ptr);
void* __swift_bridge__$Vec_NodeContent$as_ptr(void* vec_ptr);

typedef struct AnnotationKind AnnotationKind;
void __swift_bridge__$AnnotationKind$_free(void* self);

void* __swift_bridge__$Vec_AnnotationKind$new(void);
void __swift_bridge__$Vec_AnnotationKind$drop(void* vec_ptr);
void __swift_bridge__$Vec_AnnotationKind$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_AnnotationKind$pop(void* vec_ptr);
void* __swift_bridge__$Vec_AnnotationKind$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_AnnotationKind$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_AnnotationKind$len(void* vec_ptr);
void* __swift_bridge__$Vec_AnnotationKind$as_ptr(void* vec_ptr);

typedef struct ChunkType ChunkType;
void __swift_bridge__$ChunkType$_free(void* self);

void* __swift_bridge__$Vec_ChunkType$new(void);
void __swift_bridge__$Vec_ChunkType$drop(void* vec_ptr);
void __swift_bridge__$Vec_ChunkType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ChunkType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ChunkType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ChunkType$len(void* vec_ptr);
void* __swift_bridge__$Vec_ChunkType$as_ptr(void* vec_ptr);

typedef struct ElementType ElementType;
void __swift_bridge__$ElementType$_free(void* self);

void* __swift_bridge__$Vec_ElementType$new(void);
void __swift_bridge__$Vec_ElementType$drop(void* vec_ptr);
void __swift_bridge__$Vec_ElementType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ElementType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ElementType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ElementType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ElementType$len(void* vec_ptr);
void* __swift_bridge__$Vec_ElementType$as_ptr(void* vec_ptr);

typedef struct FormatMetadata FormatMetadata;
void __swift_bridge__$FormatMetadata$_free(void* self);

void* __swift_bridge__$Vec_FormatMetadata$new(void);
void __swift_bridge__$Vec_FormatMetadata$drop(void* vec_ptr);
void __swift_bridge__$Vec_FormatMetadata$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_FormatMetadata$pop(void* vec_ptr);
void* __swift_bridge__$Vec_FormatMetadata$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_FormatMetadata$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_FormatMetadata$len(void* vec_ptr);
void* __swift_bridge__$Vec_FormatMetadata$as_ptr(void* vec_ptr);

typedef struct TextDirection TextDirection;
void __swift_bridge__$TextDirection$_free(void* self);

void* __swift_bridge__$Vec_TextDirection$new(void);
void __swift_bridge__$Vec_TextDirection$drop(void* vec_ptr);
void __swift_bridge__$Vec_TextDirection$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TextDirection$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TextDirection$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TextDirection$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TextDirection$len(void* vec_ptr);
void* __swift_bridge__$Vec_TextDirection$as_ptr(void* vec_ptr);

typedef struct LinkType LinkType;
void __swift_bridge__$LinkType$_free(void* self);

void* __swift_bridge__$Vec_LinkType$new(void);
void __swift_bridge__$Vec_LinkType$drop(void* vec_ptr);
void __swift_bridge__$Vec_LinkType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LinkType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LinkType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LinkType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LinkType$len(void* vec_ptr);
void* __swift_bridge__$Vec_LinkType$as_ptr(void* vec_ptr);

typedef struct ImageType ImageType;
void __swift_bridge__$ImageType$_free(void* self);

void* __swift_bridge__$Vec_ImageType$new(void);
void __swift_bridge__$Vec_ImageType$drop(void* vec_ptr);
void __swift_bridge__$Vec_ImageType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ImageType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ImageType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ImageType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ImageType$len(void* vec_ptr);
void* __swift_bridge__$Vec_ImageType$as_ptr(void* vec_ptr);

typedef struct StructuredDataType StructuredDataType;
void __swift_bridge__$StructuredDataType$_free(void* self);

void* __swift_bridge__$Vec_StructuredDataType$new(void);
void __swift_bridge__$Vec_StructuredDataType$drop(void* vec_ptr);
void __swift_bridge__$Vec_StructuredDataType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_StructuredDataType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredDataType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_StructuredDataType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_StructuredDataType$len(void* vec_ptr);
void* __swift_bridge__$Vec_StructuredDataType$as_ptr(void* vec_ptr);

typedef struct OcrBoundingGeometry OcrBoundingGeometry;
void __swift_bridge__$OcrBoundingGeometry$_free(void* self);

void* __swift_bridge__$Vec_OcrBoundingGeometry$new(void);
void __swift_bridge__$Vec_OcrBoundingGeometry$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrBoundingGeometry$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrBoundingGeometry$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrBoundingGeometry$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrBoundingGeometry$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrBoundingGeometry$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrBoundingGeometry$as_ptr(void* vec_ptr);

typedef struct OcrElementLevel OcrElementLevel;
void __swift_bridge__$OcrElementLevel$_free(void* self);

void* __swift_bridge__$Vec_OcrElementLevel$new(void);
void __swift_bridge__$Vec_OcrElementLevel$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrElementLevel$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrElementLevel$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrElementLevel$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrElementLevel$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrElementLevel$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrElementLevel$as_ptr(void* vec_ptr);

typedef struct PageUnitType PageUnitType;
void __swift_bridge__$PageUnitType$_free(void* self);

void* __swift_bridge__$Vec_PageUnitType$new(void);
void __swift_bridge__$Vec_PageUnitType$drop(void* vec_ptr);
void __swift_bridge__$Vec_PageUnitType$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PageUnitType$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PageUnitType$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PageUnitType$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PageUnitType$len(void* vec_ptr);
void* __swift_bridge__$Vec_PageUnitType$as_ptr(void* vec_ptr);

typedef struct UriKind UriKind;
void __swift_bridge__$UriKind$_free(void* self);

void* __swift_bridge__$Vec_UriKind$new(void);
void __swift_bridge__$Vec_UriKind$drop(void* vec_ptr);
void __swift_bridge__$Vec_UriKind$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_UriKind$pop(void* vec_ptr);
void* __swift_bridge__$Vec_UriKind$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_UriKind$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_UriKind$len(void* vec_ptr);
void* __swift_bridge__$Vec_UriKind$as_ptr(void* vec_ptr);

typedef struct PoolError PoolError;
void __swift_bridge__$PoolError$_free(void* self);

void* __swift_bridge__$Vec_PoolError$new(void);
void __swift_bridge__$Vec_PoolError$drop(void* vec_ptr);
void __swift_bridge__$Vec_PoolError$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PoolError$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PoolError$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PoolError$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PoolError$len(void* vec_ptr);
void* __swift_bridge__$Vec_PoolError$as_ptr(void* vec_ptr);

typedef struct KeywordAlgorithm KeywordAlgorithm;
void __swift_bridge__$KeywordAlgorithm$_free(void* self);

void* __swift_bridge__$Vec_KeywordAlgorithm$new(void);
void __swift_bridge__$Vec_KeywordAlgorithm$drop(void* vec_ptr);
void __swift_bridge__$Vec_KeywordAlgorithm$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_KeywordAlgorithm$pop(void* vec_ptr);
void* __swift_bridge__$Vec_KeywordAlgorithm$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_KeywordAlgorithm$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_KeywordAlgorithm$len(void* vec_ptr);
void* __swift_bridge__$Vec_KeywordAlgorithm$as_ptr(void* vec_ptr);

typedef struct PSMMode PSMMode;
void __swift_bridge__$PSMMode$_free(void* self);

void* __swift_bridge__$Vec_PSMMode$new(void);
void __swift_bridge__$Vec_PSMMode$drop(void* vec_ptr);
void __swift_bridge__$Vec_PSMMode$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PSMMode$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PSMMode$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PSMMode$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PSMMode$len(void* vec_ptr);
void* __swift_bridge__$Vec_PSMMode$as_ptr(void* vec_ptr);

typedef struct PaddleLanguage PaddleLanguage;
void __swift_bridge__$PaddleLanguage$_free(void* self);

void* __swift_bridge__$Vec_PaddleLanguage$new(void);
void __swift_bridge__$Vec_PaddleLanguage$drop(void* vec_ptr);
void __swift_bridge__$Vec_PaddleLanguage$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PaddleLanguage$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PaddleLanguage$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PaddleLanguage$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PaddleLanguage$len(void* vec_ptr);
void* __swift_bridge__$Vec_PaddleLanguage$as_ptr(void* vec_ptr);

typedef struct LayoutClass LayoutClass;
void __swift_bridge__$LayoutClass$_free(void* self);

void* __swift_bridge__$Vec_LayoutClass$new(void);
void __swift_bridge__$Vec_LayoutClass$drop(void* vec_ptr);
void __swift_bridge__$Vec_LayoutClass$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_LayoutClass$pop(void* vec_ptr);
void* __swift_bridge__$Vec_LayoutClass$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_LayoutClass$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_LayoutClass$len(void* vec_ptr);
void* __swift_bridge__$Vec_LayoutClass$as_ptr(void* vec_ptr);

typedef struct OcrBackendBox OcrBackendBox;
void __swift_bridge__$OcrBackendBox$_free(void* self);

void* __swift_bridge__$Vec_OcrBackendBox$new(void);
void __swift_bridge__$Vec_OcrBackendBox$drop(void* vec_ptr);
void __swift_bridge__$Vec_OcrBackendBox$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_OcrBackendBox$pop(void* vec_ptr);
void* __swift_bridge__$Vec_OcrBackendBox$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_OcrBackendBox$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_OcrBackendBox$len(void* vec_ptr);
void* __swift_bridge__$Vec_OcrBackendBox$as_ptr(void* vec_ptr);

typedef struct PostProcessorBox PostProcessorBox;
void __swift_bridge__$PostProcessorBox$_free(void* self);

void* __swift_bridge__$Vec_PostProcessorBox$new(void);
void __swift_bridge__$Vec_PostProcessorBox$drop(void* vec_ptr);
void __swift_bridge__$Vec_PostProcessorBox$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_PostProcessorBox$pop(void* vec_ptr);
void* __swift_bridge__$Vec_PostProcessorBox$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_PostProcessorBox$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_PostProcessorBox$len(void* vec_ptr);
void* __swift_bridge__$Vec_PostProcessorBox$as_ptr(void* vec_ptr);

typedef struct ValidatorBox ValidatorBox;
void __swift_bridge__$ValidatorBox$_free(void* self);

void* __swift_bridge__$Vec_ValidatorBox$new(void);
void __swift_bridge__$Vec_ValidatorBox$drop(void* vec_ptr);
void __swift_bridge__$Vec_ValidatorBox$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_ValidatorBox$pop(void* vec_ptr);
void* __swift_bridge__$Vec_ValidatorBox$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_ValidatorBox$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_ValidatorBox$len(void* vec_ptr);
void* __swift_bridge__$Vec_ValidatorBox$as_ptr(void* vec_ptr);

typedef struct EmbeddingBackendBox EmbeddingBackendBox;
void __swift_bridge__$EmbeddingBackendBox$_free(void* self);

void* __swift_bridge__$Vec_EmbeddingBackendBox$new(void);
void __swift_bridge__$Vec_EmbeddingBackendBox$drop(void* vec_ptr);
void __swift_bridge__$Vec_EmbeddingBackendBox$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_EmbeddingBackendBox$pop(void* vec_ptr);
void* __swift_bridge__$Vec_EmbeddingBackendBox$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_EmbeddingBackendBox$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_EmbeddingBackendBox$len(void* vec_ptr);
void* __swift_bridge__$Vec_EmbeddingBackendBox$as_ptr(void* vec_ptr);

void* __swift_bridge__$AccelerationConfig$new(void* provider, uint32_t device_id);
void* __swift_bridge__$AccelerationConfig$provider(void* self);
uint32_t __swift_bridge__$AccelerationConfig$device_id(void* self);
void* __swift_bridge__$ContentFilterConfig$new(bool include_headers, bool include_footers, bool strip_repeating_text, bool include_watermarks);
bool __swift_bridge__$ContentFilterConfig$include_headers(void* self);
bool __swift_bridge__$ContentFilterConfig$include_footers(void* self);
bool __swift_bridge__$ContentFilterConfig$strip_repeating_text(void* self);
bool __swift_bridge__$ContentFilterConfig$include_watermarks(void* self);
void* __swift_bridge__$EmailConfig$new(struct __private__OptionU32 msg_fallback_codepage);
struct __private__OptionU32 __swift_bridge__$EmailConfig$msg_fallback_codepage(void* self);
void* __swift_bridge__$ExtractionConfig$new(bool use_cache, bool enable_quality_processing, void* ocr, bool force_ocr, void* force_ocr_pages, bool disable_ocr, void* chunking, void* content_filter, void* images, void* pdf_options, void* token_reduction, void* language_detection, void* pages, void* postprocessor, void* html_options, void* html_output, struct __private__OptionU64 extraction_timeout_secs, struct __private__OptionUsize max_concurrent_extractions, void* result_format, void* security_limits, void* output_format, void* layout, bool include_document_structure, void* acceleration, void* cache_namespace, struct __private__OptionU64 cache_ttl_secs, void* email, void* concurrency, uintptr_t max_archive_depth, void* tree_sitter, void* structured_extraction, void* cancel_token);
bool __swift_bridge__$ExtractionConfig$use_cache(void* self);
bool __swift_bridge__$ExtractionConfig$enable_quality_processing(void* self);
void* __swift_bridge__$ExtractionConfig$ocr(void* self);
bool __swift_bridge__$ExtractionConfig$force_ocr(void* self);
void* __swift_bridge__$ExtractionConfig$force_ocr_pages(void* self);
bool __swift_bridge__$ExtractionConfig$disable_ocr(void* self);
void* __swift_bridge__$ExtractionConfig$chunking(void* self);
void* __swift_bridge__$ExtractionConfig$content_filter(void* self);
void* __swift_bridge__$ExtractionConfig$images(void* self);
void* __swift_bridge__$ExtractionConfig$pdf_options(void* self);
void* __swift_bridge__$ExtractionConfig$token_reduction(void* self);
void* __swift_bridge__$ExtractionConfig$language_detection(void* self);
void* __swift_bridge__$ExtractionConfig$pages(void* self);
void* __swift_bridge__$ExtractionConfig$postprocessor(void* self);
void* __swift_bridge__$ExtractionConfig$html_options(void* self);
void* __swift_bridge__$ExtractionConfig$html_output(void* self);
struct __private__OptionU64 __swift_bridge__$ExtractionConfig$extraction_timeout_secs(void* self);
struct __private__OptionUsize __swift_bridge__$ExtractionConfig$max_concurrent_extractions(void* self);
void* __swift_bridge__$ExtractionConfig$result_format(void* self);
void* __swift_bridge__$ExtractionConfig$security_limits(void* self);
void* __swift_bridge__$ExtractionConfig$output_format(void* self);
void* __swift_bridge__$ExtractionConfig$layout(void* self);
bool __swift_bridge__$ExtractionConfig$include_document_structure(void* self);
void* __swift_bridge__$ExtractionConfig$acceleration(void* self);
void* __swift_bridge__$ExtractionConfig$cache_namespace(void* self);
struct __private__OptionU64 __swift_bridge__$ExtractionConfig$cache_ttl_secs(void* self);
void* __swift_bridge__$ExtractionConfig$email(void* self);
void* __swift_bridge__$ExtractionConfig$concurrency(void* self);
uintptr_t __swift_bridge__$ExtractionConfig$max_archive_depth(void* self);
void* __swift_bridge__$ExtractionConfig$tree_sitter(void* self);
void* __swift_bridge__$ExtractionConfig$structured_extraction(void* self);
void* __swift_bridge__$ExtractionConfig$cancel_token(void* self);
void* __swift_bridge__$FileExtractionConfig$new(struct __private__OptionBool enable_quality_processing, void* ocr, struct __private__OptionBool force_ocr, void* force_ocr_pages, struct __private__OptionBool disable_ocr, void* chunking, void* content_filter, void* images, void* pdf_options, void* token_reduction, void* language_detection, void* pages, void* postprocessor, void* html_options, void* result_format, void* output_format, struct __private__OptionBool include_document_structure, void* layout, struct __private__OptionU64 timeout_secs, void* tree_sitter, void* structured_extraction);
struct __private__OptionBool __swift_bridge__$FileExtractionConfig$enable_quality_processing(void* self);
void* __swift_bridge__$FileExtractionConfig$ocr(void* self);
struct __private__OptionBool __swift_bridge__$FileExtractionConfig$force_ocr(void* self);
void* __swift_bridge__$FileExtractionConfig$force_ocr_pages(void* self);
struct __private__OptionBool __swift_bridge__$FileExtractionConfig$disable_ocr(void* self);
void* __swift_bridge__$FileExtractionConfig$chunking(void* self);
void* __swift_bridge__$FileExtractionConfig$content_filter(void* self);
void* __swift_bridge__$FileExtractionConfig$images(void* self);
void* __swift_bridge__$FileExtractionConfig$pdf_options(void* self);
void* __swift_bridge__$FileExtractionConfig$token_reduction(void* self);
void* __swift_bridge__$FileExtractionConfig$language_detection(void* self);
void* __swift_bridge__$FileExtractionConfig$pages(void* self);
void* __swift_bridge__$FileExtractionConfig$postprocessor(void* self);
void* __swift_bridge__$FileExtractionConfig$html_options(void* self);
void* __swift_bridge__$FileExtractionConfig$result_format(void* self);
void* __swift_bridge__$FileExtractionConfig$output_format(void* self);
struct __private__OptionBool __swift_bridge__$FileExtractionConfig$include_document_structure(void* self);
void* __swift_bridge__$FileExtractionConfig$layout(void* self);
struct __private__OptionU64 __swift_bridge__$FileExtractionConfig$timeout_secs(void* self);
void* __swift_bridge__$FileExtractionConfig$tree_sitter(void* self);
void* __swift_bridge__$FileExtractionConfig$structured_extraction(void* self);
void* __swift_bridge__$ImageExtractionConfig$new(bool extract_images, int32_t target_dpi, int32_t max_image_dimension, bool inject_placeholders, bool auto_adjust_dpi, int32_t min_dpi, int32_t max_dpi, struct __private__OptionU32 max_images_per_page);
bool __swift_bridge__$ImageExtractionConfig$extract_images(void* self);
int32_t __swift_bridge__$ImageExtractionConfig$target_dpi(void* self);
int32_t __swift_bridge__$ImageExtractionConfig$max_image_dimension(void* self);
bool __swift_bridge__$ImageExtractionConfig$inject_placeholders(void* self);
bool __swift_bridge__$ImageExtractionConfig$auto_adjust_dpi(void* self);
int32_t __swift_bridge__$ImageExtractionConfig$min_dpi(void* self);
int32_t __swift_bridge__$ImageExtractionConfig$max_dpi(void* self);
struct __private__OptionU32 __swift_bridge__$ImageExtractionConfig$max_images_per_page(void* self);
void* __swift_bridge__$TokenReductionOptions$new(void* mode, bool preserve_important_words);
void* __swift_bridge__$TokenReductionOptions$mode(void* self);
bool __swift_bridge__$TokenReductionOptions$preserve_important_words(void* self);
void* __swift_bridge__$LanguageDetectionConfig$new(bool enabled, double min_confidence, bool detect_multiple);
bool __swift_bridge__$LanguageDetectionConfig$enabled(void* self);
double __swift_bridge__$LanguageDetectionConfig$min_confidence(void* self);
bool __swift_bridge__$LanguageDetectionConfig$detect_multiple(void* self);
void* __swift_bridge__$HtmlOutputConfig$new(void* css, void* css_file, void* theme, void* class_prefix, bool embed_css);
void* __swift_bridge__$HtmlOutputConfig$css(void* self);
void* __swift_bridge__$HtmlOutputConfig$css_file(void* self);
void* __swift_bridge__$HtmlOutputConfig$theme(void* self);
void* __swift_bridge__$HtmlOutputConfig$class_prefix(void* self);
bool __swift_bridge__$HtmlOutputConfig$embed_css(void* self);
void* __swift_bridge__$LayoutDetectionConfig$new(struct __private__OptionF32 confidence_threshold, bool apply_heuristics, void* table_model, void* acceleration);
struct __private__OptionF32 __swift_bridge__$LayoutDetectionConfig$confidence_threshold(void* self);
bool __swift_bridge__$LayoutDetectionConfig$apply_heuristics(void* self);
void* __swift_bridge__$LayoutDetectionConfig$table_model(void* self);
void* __swift_bridge__$LayoutDetectionConfig$acceleration(void* self);
void* __swift_bridge__$LlmConfig$new(void* model, void* api_key, void* base_url, struct __private__OptionU64 timeout_secs, struct __private__OptionU32 max_retries, struct __private__OptionF64 temperature, struct __private__OptionU64 max_tokens);
void* __swift_bridge__$LlmConfig$model(void* self);
void* __swift_bridge__$LlmConfig$api_key(void* self);
void* __swift_bridge__$LlmConfig$base_url(void* self);
struct __private__OptionU64 __swift_bridge__$LlmConfig$timeout_secs(void* self);
struct __private__OptionU32 __swift_bridge__$LlmConfig$max_retries(void* self);
struct __private__OptionF64 __swift_bridge__$LlmConfig$temperature(void* self);
struct __private__OptionU64 __swift_bridge__$LlmConfig$max_tokens(void* self);
void* __swift_bridge__$StructuredExtractionConfig$new(void* schema, void* schema_name, void* schema_description, bool strict, void* prompt, void* llm);
void* __swift_bridge__$StructuredExtractionConfig$schema(void* self);
void* __swift_bridge__$StructuredExtractionConfig$schema_name(void* self);
void* __swift_bridge__$StructuredExtractionConfig$schema_description(void* self);
bool __swift_bridge__$StructuredExtractionConfig$strict(void* self);
void* __swift_bridge__$StructuredExtractionConfig$prompt(void* self);
void* __swift_bridge__$StructuredExtractionConfig$llm(void* self);
void* __swift_bridge__$OcrQualityThresholds$new(uintptr_t min_total_non_whitespace, double min_non_whitespace_per_page, uintptr_t min_meaningful_word_len, uintptr_t min_meaningful_words, double min_alnum_ratio, uintptr_t min_garbage_chars, double max_fragmented_word_ratio, double critical_fragmented_word_ratio, double min_avg_word_length, uintptr_t min_words_for_avg_length_check, double min_consecutive_repeat_ratio, uintptr_t min_words_for_repeat_check, uintptr_t substantive_min_chars, uintptr_t non_text_min_chars, double alnum_ws_ratio_threshold, double pipeline_min_quality);
uintptr_t __swift_bridge__$OcrQualityThresholds$min_total_non_whitespace(void* self);
double __swift_bridge__$OcrQualityThresholds$min_non_whitespace_per_page(void* self);
uintptr_t __swift_bridge__$OcrQualityThresholds$min_meaningful_word_len(void* self);
uintptr_t __swift_bridge__$OcrQualityThresholds$min_meaningful_words(void* self);
double __swift_bridge__$OcrQualityThresholds$min_alnum_ratio(void* self);
uintptr_t __swift_bridge__$OcrQualityThresholds$min_garbage_chars(void* self);
double __swift_bridge__$OcrQualityThresholds$max_fragmented_word_ratio(void* self);
double __swift_bridge__$OcrQualityThresholds$critical_fragmented_word_ratio(void* self);
double __swift_bridge__$OcrQualityThresholds$min_avg_word_length(void* self);
uintptr_t __swift_bridge__$OcrQualityThresholds$min_words_for_avg_length_check(void* self);
double __swift_bridge__$OcrQualityThresholds$min_consecutive_repeat_ratio(void* self);
uintptr_t __swift_bridge__$OcrQualityThresholds$min_words_for_repeat_check(void* self);
uintptr_t __swift_bridge__$OcrQualityThresholds$substantive_min_chars(void* self);
uintptr_t __swift_bridge__$OcrQualityThresholds$non_text_min_chars(void* self);
double __swift_bridge__$OcrQualityThresholds$alnum_ws_ratio_threshold(void* self);
double __swift_bridge__$OcrQualityThresholds$pipeline_min_quality(void* self);
void* __swift_bridge__$OcrPipelineStage$new(void* backend, uint32_t priority, void* language, void* tesseract_config, void* paddle_ocr_config, void* vlm_config);
void* __swift_bridge__$OcrPipelineStage$backend(void* self);
uint32_t __swift_bridge__$OcrPipelineStage$priority(void* self);
void* __swift_bridge__$OcrPipelineStage$language(void* self);
void* __swift_bridge__$OcrPipelineStage$tesseract_config(void* self);
void* __swift_bridge__$OcrPipelineStage$paddle_ocr_config(void* self);
void* __swift_bridge__$OcrPipelineStage$vlm_config(void* self);
void* __swift_bridge__$OcrPipelineConfig$new(void* stages, void* quality_thresholds);
void* __swift_bridge__$OcrPipelineConfig$stages(void* self);
void* __swift_bridge__$OcrPipelineConfig$quality_thresholds(void* self);
void* __swift_bridge__$OcrConfig$new(bool enabled, void* backend, void* language, void* tesseract_config, void* output_format, void* paddle_ocr_config, void* element_config, void* quality_thresholds, void* pipeline, bool auto_rotate, void* vlm_config, void* vlm_prompt, void* acceleration);
bool __swift_bridge__$OcrConfig$enabled(void* self);
void* __swift_bridge__$OcrConfig$backend(void* self);
void* __swift_bridge__$OcrConfig$language(void* self);
void* __swift_bridge__$OcrConfig$tesseract_config(void* self);
void* __swift_bridge__$OcrConfig$output_format(void* self);
void* __swift_bridge__$OcrConfig$paddle_ocr_config(void* self);
void* __swift_bridge__$OcrConfig$element_config(void* self);
void* __swift_bridge__$OcrConfig$quality_thresholds(void* self);
void* __swift_bridge__$OcrConfig$pipeline(void* self);
bool __swift_bridge__$OcrConfig$auto_rotate(void* self);
void* __swift_bridge__$OcrConfig$vlm_config(void* self);
void* __swift_bridge__$OcrConfig$vlm_prompt(void* self);
void* __swift_bridge__$OcrConfig$acceleration(void* self);
void* __swift_bridge__$PageConfig$new(bool extract_pages, bool insert_page_markers, void* marker_format);
bool __swift_bridge__$PageConfig$extract_pages(void* self);
bool __swift_bridge__$PageConfig$insert_page_markers(void* self);
void* __swift_bridge__$PageConfig$marker_format(void* self);
void* __swift_bridge__$PdfConfig$new(void* backend, bool extract_images, void* passwords, bool extract_metadata, void* hierarchy, bool extract_annotations, struct __private__OptionF32 top_margin_fraction, struct __private__OptionF32 bottom_margin_fraction, bool allow_single_column_tables);
void* __swift_bridge__$PdfConfig$backend(void* self);
bool __swift_bridge__$PdfConfig$extract_images(void* self);
void* __swift_bridge__$PdfConfig$passwords(void* self);
bool __swift_bridge__$PdfConfig$extract_metadata(void* self);
void* __swift_bridge__$PdfConfig$hierarchy(void* self);
bool __swift_bridge__$PdfConfig$extract_annotations(void* self);
struct __private__OptionF32 __swift_bridge__$PdfConfig$top_margin_fraction(void* self);
struct __private__OptionF32 __swift_bridge__$PdfConfig$bottom_margin_fraction(void* self);
bool __swift_bridge__$PdfConfig$allow_single_column_tables(void* self);
void* __swift_bridge__$HierarchyConfig$new(bool enabled, uintptr_t k_clusters, bool include_bbox, struct __private__OptionF32 ocr_coverage_threshold);
bool __swift_bridge__$HierarchyConfig$enabled(void* self);
uintptr_t __swift_bridge__$HierarchyConfig$k_clusters(void* self);
bool __swift_bridge__$HierarchyConfig$include_bbox(void* self);
struct __private__OptionF32 __swift_bridge__$HierarchyConfig$ocr_coverage_threshold(void* self);
void* __swift_bridge__$PostProcessorConfig$new(bool enabled, void* enabled_processors, void* disabled_processors, void* enabled_set, void* disabled_set);
bool __swift_bridge__$PostProcessorConfig$enabled(void* self);
void* __swift_bridge__$PostProcessorConfig$enabled_processors(void* self);
void* __swift_bridge__$PostProcessorConfig$disabled_processors(void* self);
void* __swift_bridge__$PostProcessorConfig$enabled_set(void* self);
void* __swift_bridge__$PostProcessorConfig$disabled_set(void* self);
void* __swift_bridge__$ChunkingConfig$new(uintptr_t max_characters, uintptr_t overlap, bool trim, void* chunker_type, void* embedding, void* preset, void* sizing, bool prepend_heading_context, struct __private__OptionF32 topic_threshold);
uintptr_t __swift_bridge__$ChunkingConfig$max_characters(void* self);
uintptr_t __swift_bridge__$ChunkingConfig$overlap(void* self);
bool __swift_bridge__$ChunkingConfig$trim(void* self);
void* __swift_bridge__$ChunkingConfig$chunker_type(void* self);
void* __swift_bridge__$ChunkingConfig$embedding(void* self);
void* __swift_bridge__$ChunkingConfig$preset(void* self);
void* __swift_bridge__$ChunkingConfig$sizing(void* self);
bool __swift_bridge__$ChunkingConfig$prepend_heading_context(void* self);
struct __private__OptionF32 __swift_bridge__$ChunkingConfig$topic_threshold(void* self);
void* __swift_bridge__$EmbeddingConfig$new(void* model, bool normalize, uintptr_t batch_size, bool show_download_progress, void* cache_dir, void* acceleration, struct __private__OptionU64 max_embed_duration_secs);
void* __swift_bridge__$EmbeddingConfig$model(void* self);
bool __swift_bridge__$EmbeddingConfig$normalize(void* self);
uintptr_t __swift_bridge__$EmbeddingConfig$batch_size(void* self);
bool __swift_bridge__$EmbeddingConfig$show_download_progress(void* self);
void* __swift_bridge__$EmbeddingConfig$cache_dir(void* self);
void* __swift_bridge__$EmbeddingConfig$acceleration(void* self);
struct __private__OptionU64 __swift_bridge__$EmbeddingConfig$max_embed_duration_secs(void* self);
void* __swift_bridge__$TreeSitterConfig$new(bool enabled, void* cache_dir, void* languages, void* groups, void* process);
bool __swift_bridge__$TreeSitterConfig$enabled(void* self);
void* __swift_bridge__$TreeSitterConfig$cache_dir(void* self);
void* __swift_bridge__$TreeSitterConfig$languages(void* self);
void* __swift_bridge__$TreeSitterConfig$groups(void* self);
void* __swift_bridge__$TreeSitterConfig$process(void* self);
void* __swift_bridge__$TreeSitterProcessConfig$new(bool structure, bool imports, bool exports, bool comments, bool docstrings, bool symbols, bool diagnostics, struct __private__OptionUsize chunk_max_size, void* content_mode);
bool __swift_bridge__$TreeSitterProcessConfig$structure(void* self);
bool __swift_bridge__$TreeSitterProcessConfig$imports(void* self);
bool __swift_bridge__$TreeSitterProcessConfig$exports(void* self);
bool __swift_bridge__$TreeSitterProcessConfig$comments(void* self);
bool __swift_bridge__$TreeSitterProcessConfig$docstrings(void* self);
bool __swift_bridge__$TreeSitterProcessConfig$symbols(void* self);
bool __swift_bridge__$TreeSitterProcessConfig$diagnostics(void* self);
struct __private__OptionUsize __swift_bridge__$TreeSitterProcessConfig$chunk_max_size(void* self);
void* __swift_bridge__$TreeSitterProcessConfig$content_mode(void* self);
void* __swift_bridge__$SupportedFormat$new(void* extension_, void* mime_type);
void* __swift_bridge__$SupportedFormat$extension_(void* self);
void* __swift_bridge__$SupportedFormat$mime_type(void* self);
void* __swift_bridge__$ServerConfig$new(void* host, uint16_t port, void* cors_origins, uintptr_t max_request_body_bytes, uintptr_t max_multipart_field_bytes);
void* __swift_bridge__$ServerConfig$host(void* self);
uint16_t __swift_bridge__$ServerConfig$port(void* self);
void* __swift_bridge__$ServerConfig$cors_origins(void* self);
uintptr_t __swift_bridge__$ServerConfig$max_request_body_bytes(void* self);
uintptr_t __swift_bridge__$ServerConfig$max_multipart_field_bytes(void* self);
void* __swift_bridge__$StructuredDataResult$new(void* content, void* format, void* metadata, void* text_fields);
void* __swift_bridge__$StructuredDataResult$content(void* self);
void* __swift_bridge__$StructuredDataResult$format(void* self);
void* __swift_bridge__$StructuredDataResult$metadata(void* self);
void* __swift_bridge__$StructuredDataResult$text_fields(void* self);
void* __swift_bridge__$ImageOcrResult$new(void* content, void* boundaries, void* page_contents);
void* __swift_bridge__$ImageOcrResult$content(void* self);
void* __swift_bridge__$ImageOcrResult$boundaries(void* self);
void* __swift_bridge__$ImageOcrResult$page_contents(void* self);
void* __swift_bridge__$HtmlExtractionResult$new(void* markdown, void* images, void* warnings);
void* __swift_bridge__$HtmlExtractionResult$markdown(void* self);
void* __swift_bridge__$HtmlExtractionResult$images(void* self);
void* __swift_bridge__$HtmlExtractionResult$warnings(void* self);
void* __swift_bridge__$ExtractedInlineImage$new(void* data, void* format, void* filename, void* description, void* dimensions, void* attributes);
void* __swift_bridge__$ExtractedInlineImage$data(void* self);
void* __swift_bridge__$ExtractedInlineImage$format(void* self);
void* __swift_bridge__$ExtractedInlineImage$filename(void* self);
void* __swift_bridge__$ExtractedInlineImage$description(void* self);
void* __swift_bridge__$ExtractedInlineImage$dimensions(void* self);
void* __swift_bridge__$ExtractedInlineImage$attributes(void* self);
void* __swift_bridge__$Drawing$new(void* drawing_type, void* extent, void* doc_properties, void* image_ref);
void* __swift_bridge__$Drawing$drawing_type(void* self);
void* __swift_bridge__$Drawing$extent(void* self);
void* __swift_bridge__$Drawing$doc_properties(void* self);
void* __swift_bridge__$Drawing$image_ref(void* self);
void* __swift_bridge__$AnchorProperties$new(bool behind_doc, bool layout_in_cell, struct __private__OptionI64 relative_height, void* position_h, void* position_v, void* wrap_type);
bool __swift_bridge__$AnchorProperties$behind_doc(void* self);
bool __swift_bridge__$AnchorProperties$layout_in_cell(void* self);
struct __private__OptionI64 __swift_bridge__$AnchorProperties$relative_height(void* self);
void* __swift_bridge__$AnchorProperties$position_h(void* self);
void* __swift_bridge__$AnchorProperties$position_v(void* self);
void* __swift_bridge__$AnchorProperties$wrap_type(void* self);
void* __swift_bridge__$HeaderFooter$new(void* paragraphs, void* tables, void* header_type);
void* __swift_bridge__$HeaderFooter$paragraphs(void* self);
void* __swift_bridge__$HeaderFooter$tables(void* self);
void* __swift_bridge__$HeaderFooter$header_type(void* self);
void* __swift_bridge__$Note$new(void* id, void* note_type, void* paragraphs);
void* __swift_bridge__$Note$id(void* self);
void* __swift_bridge__$Note$note_type(void* self);
void* __swift_bridge__$Note$paragraphs(void* self);
void* __swift_bridge__$PageMarginsPoints$new(struct __private__OptionF64 top, struct __private__OptionF64 right, struct __private__OptionF64 bottom, struct __private__OptionF64 left, struct __private__OptionF64 header, struct __private__OptionF64 footer, struct __private__OptionF64 gutter);
struct __private__OptionF64 __swift_bridge__$PageMarginsPoints$top(void* self);
struct __private__OptionF64 __swift_bridge__$PageMarginsPoints$right(void* self);
struct __private__OptionF64 __swift_bridge__$PageMarginsPoints$bottom(void* self);
struct __private__OptionF64 __swift_bridge__$PageMarginsPoints$left(void* self);
struct __private__OptionF64 __swift_bridge__$PageMarginsPoints$header(void* self);
struct __private__OptionF64 __swift_bridge__$PageMarginsPoints$footer(void* self);
struct __private__OptionF64 __swift_bridge__$PageMarginsPoints$gutter(void* self);
void* __swift_bridge__$StyleDefinition$new(void* id, void* name, void* style_type, void* based_on, void* next_style, bool is_default, void* paragraph_properties, void* run_properties);
void* __swift_bridge__$StyleDefinition$id(void* self);
void* __swift_bridge__$StyleDefinition$name(void* self);
void* __swift_bridge__$StyleDefinition$style_type(void* self);
void* __swift_bridge__$StyleDefinition$based_on(void* self);
void* __swift_bridge__$StyleDefinition$next_style(void* self);
bool __swift_bridge__$StyleDefinition$is_default(void* self);
void* __swift_bridge__$StyleDefinition$paragraph_properties(void* self);
void* __swift_bridge__$StyleDefinition$run_properties(void* self);
void* __swift_bridge__$ResolvedStyle$new(void* paragraph_properties, void* run_properties);
void* __swift_bridge__$ResolvedStyle$paragraph_properties(void* self);
void* __swift_bridge__$ResolvedStyle$run_properties(void* self);
void* __swift_bridge__$TableProperties$new(void* style_id, void* width, void* alignment, void* layout, void* look, void* borders, void* cell_margins, void* indent, void* caption);
void* __swift_bridge__$TableProperties$style_id(void* self);
void* __swift_bridge__$TableProperties$width(void* self);
void* __swift_bridge__$TableProperties$alignment(void* self);
void* __swift_bridge__$TableProperties$layout(void* self);
void* __swift_bridge__$TableProperties$look(void* self);
void* __swift_bridge__$TableProperties$borders(void* self);
void* __swift_bridge__$TableProperties$cell_margins(void* self);
void* __swift_bridge__$TableProperties$indent(void* self);
void* __swift_bridge__$TableProperties$caption(void* self);
void* __swift_bridge__$XlsxAppProperties$new(void* application, void* app_version, struct __private__OptionI32 doc_security, struct __private__OptionBool scale_crop, struct __private__OptionBool links_up_to_date, struct __private__OptionBool shared_doc, struct __private__OptionBool hyperlinks_changed, void* company, void* worksheet_names);
void* __swift_bridge__$XlsxAppProperties$application(void* self);
void* __swift_bridge__$XlsxAppProperties$app_version(void* self);
struct __private__OptionI32 __swift_bridge__$XlsxAppProperties$doc_security(void* self);
struct __private__OptionBool __swift_bridge__$XlsxAppProperties$scale_crop(void* self);
struct __private__OptionBool __swift_bridge__$XlsxAppProperties$links_up_to_date(void* self);
struct __private__OptionBool __swift_bridge__$XlsxAppProperties$shared_doc(void* self);
struct __private__OptionBool __swift_bridge__$XlsxAppProperties$hyperlinks_changed(void* self);
void* __swift_bridge__$XlsxAppProperties$company(void* self);
void* __swift_bridge__$XlsxAppProperties$worksheet_names(void* self);
void* __swift_bridge__$PptxAppProperties$new(void* application, void* app_version, struct __private__OptionI32 total_time, void* company, struct __private__OptionI32 doc_security, struct __private__OptionBool scale_crop, struct __private__OptionBool links_up_to_date, struct __private__OptionBool shared_doc, struct __private__OptionBool hyperlinks_changed, struct __private__OptionI32 slides, struct __private__OptionI32 notes, struct __private__OptionI32 hidden_slides, struct __private__OptionI32 multimedia_clips, void* presentation_format, void* slide_titles);
void* __swift_bridge__$PptxAppProperties$application(void* self);
void* __swift_bridge__$PptxAppProperties$app_version(void* self);
struct __private__OptionI32 __swift_bridge__$PptxAppProperties$total_time(void* self);
void* __swift_bridge__$PptxAppProperties$company(void* self);
struct __private__OptionI32 __swift_bridge__$PptxAppProperties$doc_security(void* self);
struct __private__OptionBool __swift_bridge__$PptxAppProperties$scale_crop(void* self);
struct __private__OptionBool __swift_bridge__$PptxAppProperties$links_up_to_date(void* self);
struct __private__OptionBool __swift_bridge__$PptxAppProperties$shared_doc(void* self);
struct __private__OptionBool __swift_bridge__$PptxAppProperties$hyperlinks_changed(void* self);
struct __private__OptionI32 __swift_bridge__$PptxAppProperties$slides(void* self);
struct __private__OptionI32 __swift_bridge__$PptxAppProperties$notes(void* self);
struct __private__OptionI32 __swift_bridge__$PptxAppProperties$hidden_slides(void* self);
struct __private__OptionI32 __swift_bridge__$PptxAppProperties$multimedia_clips(void* self);
void* __swift_bridge__$PptxAppProperties$presentation_format(void* self);
void* __swift_bridge__$PptxAppProperties$slide_titles(void* self);
void* __swift_bridge__$OdtProperties$new(void* title, void* subject, void* creator, void* initial_creator, void* keywords, void* description, void* date, void* creation_date, void* language, void* generator, void* editing_duration, void* editing_cycles, struct __private__OptionI32 page_count, struct __private__OptionI32 word_count, struct __private__OptionI32 character_count, struct __private__OptionI32 paragraph_count, struct __private__OptionI32 table_count, struct __private__OptionI32 image_count);
void* __swift_bridge__$OdtProperties$title(void* self);
void* __swift_bridge__$OdtProperties$subject(void* self);
void* __swift_bridge__$OdtProperties$creator(void* self);
void* __swift_bridge__$OdtProperties$initial_creator(void* self);
void* __swift_bridge__$OdtProperties$keywords(void* self);
void* __swift_bridge__$OdtProperties$description(void* self);
void* __swift_bridge__$OdtProperties$date(void* self);
void* __swift_bridge__$OdtProperties$creation_date(void* self);
void* __swift_bridge__$OdtProperties$language(void* self);
void* __swift_bridge__$OdtProperties$generator(void* self);
void* __swift_bridge__$OdtProperties$editing_duration(void* self);
void* __swift_bridge__$OdtProperties$editing_cycles(void* self);
struct __private__OptionI32 __swift_bridge__$OdtProperties$page_count(void* self);
struct __private__OptionI32 __swift_bridge__$OdtProperties$word_count(void* self);
struct __private__OptionI32 __swift_bridge__$OdtProperties$character_count(void* self);
struct __private__OptionI32 __swift_bridge__$OdtProperties$paragraph_count(void* self);
struct __private__OptionI32 __swift_bridge__$OdtProperties$table_count(void* self);
struct __private__OptionI32 __swift_bridge__$OdtProperties$image_count(void* self);
void* __swift_bridge__$TokenReductionConfig$new(void* level, void* language_hint, bool preserve_markdown, bool preserve_code, float semantic_threshold, bool enable_parallel, bool use_simd, void* custom_stopwords, void* preserve_patterns, struct __private__OptionF32 target_reduction, bool enable_semantic_clustering);
void* __swift_bridge__$TokenReductionConfig$level(void* self);
void* __swift_bridge__$TokenReductionConfig$language_hint(void* self);
bool __swift_bridge__$TokenReductionConfig$preserve_markdown(void* self);
bool __swift_bridge__$TokenReductionConfig$preserve_code(void* self);
float __swift_bridge__$TokenReductionConfig$semantic_threshold(void* self);
bool __swift_bridge__$TokenReductionConfig$enable_parallel(void* self);
bool __swift_bridge__$TokenReductionConfig$use_simd(void* self);
void* __swift_bridge__$TokenReductionConfig$custom_stopwords(void* self);
void* __swift_bridge__$TokenReductionConfig$preserve_patterns(void* self);
struct __private__OptionF32 __swift_bridge__$TokenReductionConfig$target_reduction(void* self);
bool __swift_bridge__$TokenReductionConfig$enable_semantic_clustering(void* self);
void* __swift_bridge__$PdfAnnotation$new(void* annotation_type, void* content, uintptr_t page_number, void* bounding_box);
void* __swift_bridge__$PdfAnnotation$annotation_type(void* self);
void* __swift_bridge__$PdfAnnotation$content(void* self);
uintptr_t __swift_bridge__$PdfAnnotation$page_number(void* self);
void* __swift_bridge__$PdfAnnotation$bounding_box(void* self);
void* __swift_bridge__$DjotContent$new(void* plain_text, void* blocks, void* metadata, void* tables, void* images, void* links, void* footnotes, void* attributes);
void* __swift_bridge__$DjotContent$plain_text(void* self);
void* __swift_bridge__$DjotContent$blocks(void* self);
void* __swift_bridge__$DjotContent$metadata(void* self);
void* __swift_bridge__$DjotContent$tables(void* self);
void* __swift_bridge__$DjotContent$images(void* self);
void* __swift_bridge__$DjotContent$links(void* self);
void* __swift_bridge__$DjotContent$footnotes(void* self);
void* __swift_bridge__$DjotContent$attributes(void* self);
void* __swift_bridge__$FormattedBlock$new(void* block_type, struct __private__OptionUsize level, void* inline_content, void* attributes, void* language, void* code, void* children);
void* __swift_bridge__$FormattedBlock$block_type(void* self);
struct __private__OptionUsize __swift_bridge__$FormattedBlock$level(void* self);
void* __swift_bridge__$FormattedBlock$inline_content(void* self);
void* __swift_bridge__$FormattedBlock$attributes(void* self);
void* __swift_bridge__$FormattedBlock$language(void* self);
void* __swift_bridge__$FormattedBlock$code(void* self);
void* __swift_bridge__$FormattedBlock$children(void* self);
void* __swift_bridge__$InlineElement$new(void* element_type, void* content, void* attributes, void* metadata);
void* __swift_bridge__$InlineElement$element_type(void* self);
void* __swift_bridge__$InlineElement$content(void* self);
void* __swift_bridge__$InlineElement$attributes(void* self);
void* __swift_bridge__$InlineElement$metadata(void* self);
void* __swift_bridge__$DjotImage$new(void* src, void* alt, void* title, void* attributes);
void* __swift_bridge__$DjotImage$src(void* self);
void* __swift_bridge__$DjotImage$alt(void* self);
void* __swift_bridge__$DjotImage$title(void* self);
void* __swift_bridge__$DjotImage$attributes(void* self);
void* __swift_bridge__$DjotLink$new(void* url, void* text, void* title, void* attributes);
void* __swift_bridge__$DjotLink$url(void* self);
void* __swift_bridge__$DjotLink$text(void* self);
void* __swift_bridge__$DjotLink$title(void* self);
void* __swift_bridge__$DjotLink$attributes(void* self);
void* __swift_bridge__$Footnote$new(void* label, void* content);
void* __swift_bridge__$Footnote$label(void* self);
void* __swift_bridge__$Footnote$content(void* self);
void* __swift_bridge__$DocumentStructure$new(void* nodes, void* source_format, void* relationships);
void* __swift_bridge__$DocumentStructure$nodes(void* self);
void* __swift_bridge__$DocumentStructure$source_format(void* self);
void* __swift_bridge__$DocumentStructure$relationships(void* self);
void* __swift_bridge__$DocumentRelationship$new(uint32_t source, uint32_t target, void* kind);
uint32_t __swift_bridge__$DocumentRelationship$source(void* self);
uint32_t __swift_bridge__$DocumentRelationship$target(void* self);
void* __swift_bridge__$DocumentRelationship$kind(void* self);
void* __swift_bridge__$DocumentNode$new(void* id, void* content, struct __private__OptionU32 parent, void* children, void* content_layer, struct __private__OptionU32 page, struct __private__OptionU32 page_end, void* bbox, void* annotations, void* attributes);
void* __swift_bridge__$DocumentNode$id(void* self);
void* __swift_bridge__$DocumentNode$content(void* self);
struct __private__OptionU32 __swift_bridge__$DocumentNode$parent(void* self);
void* __swift_bridge__$DocumentNode$children(void* self);
void* __swift_bridge__$DocumentNode$content_layer(void* self);
struct __private__OptionU32 __swift_bridge__$DocumentNode$page(void* self);
struct __private__OptionU32 __swift_bridge__$DocumentNode$page_end(void* self);
void* __swift_bridge__$DocumentNode$bbox(void* self);
void* __swift_bridge__$DocumentNode$annotations(void* self);
void* __swift_bridge__$DocumentNode$attributes(void* self);
void* __swift_bridge__$GridCell$new(void* content, uint32_t row, uint32_t col, uint32_t row_span, uint32_t col_span, bool is_header, void* bbox);
void* __swift_bridge__$GridCell$content(void* self);
uint32_t __swift_bridge__$GridCell$row(void* self);
uint32_t __swift_bridge__$GridCell$col(void* self);
uint32_t __swift_bridge__$GridCell$row_span(void* self);
uint32_t __swift_bridge__$GridCell$col_span(void* self);
bool __swift_bridge__$GridCell$is_header(void* self);
void* __swift_bridge__$GridCell$bbox(void* self);
void* __swift_bridge__$TextAnnotation$new(uint32_t start, uint32_t end, void* kind);
uint32_t __swift_bridge__$TextAnnotation$start(void* self);
uint32_t __swift_bridge__$TextAnnotation$end(void* self);
void* __swift_bridge__$TextAnnotation$kind(void* self);
void* __swift_bridge__$ExtractionResult$new(void* content, void* mime_type, void* metadata, void* tables, void* detected_languages, void* chunks, void* images, void* pages, void* elements, void* djot_content, void* ocr_elements, void* document, struct __private__OptionF64 quality_score, void* processing_warnings, void* annotations, void* children, void* uris, void* structured_output, void* code_intelligence, void* llm_usage, void* formatted_content);
void* __swift_bridge__$ExtractionResult$content(void* self);
void* __swift_bridge__$ExtractionResult$mime_type(void* self);
void* __swift_bridge__$ExtractionResult$metadata(void* self);
void* __swift_bridge__$ExtractionResult$tables(void* self);
void* __swift_bridge__$ExtractionResult$detected_languages(void* self);
void* __swift_bridge__$ExtractionResult$chunks(void* self);
void* __swift_bridge__$ExtractionResult$images(void* self);
void* __swift_bridge__$ExtractionResult$pages(void* self);
void* __swift_bridge__$ExtractionResult$elements(void* self);
void* __swift_bridge__$ExtractionResult$djot_content(void* self);
void* __swift_bridge__$ExtractionResult$ocr_elements(void* self);
void* __swift_bridge__$ExtractionResult$document(void* self);
struct __private__OptionF64 __swift_bridge__$ExtractionResult$quality_score(void* self);
void* __swift_bridge__$ExtractionResult$processing_warnings(void* self);
void* __swift_bridge__$ExtractionResult$annotations(void* self);
void* __swift_bridge__$ExtractionResult$children(void* self);
void* __swift_bridge__$ExtractionResult$uris(void* self);
void* __swift_bridge__$ExtractionResult$structured_output(void* self);
void* __swift_bridge__$ExtractionResult$code_intelligence(void* self);
void* __swift_bridge__$ExtractionResult$llm_usage(void* self);
void* __swift_bridge__$ExtractionResult$formatted_content(void* self);
void* __swift_bridge__$ExtractionResult$ocr_internal_document(void* self);
void* __swift_bridge__$ArchiveEntry$new(void* path, void* mime_type, void* result);
void* __swift_bridge__$ArchiveEntry$path(void* self);
void* __swift_bridge__$ArchiveEntry$mime_type(void* self);
void* __swift_bridge__$ArchiveEntry$result(void* self);
void* __swift_bridge__$ProcessingWarning$new(void* source, void* message);
void* __swift_bridge__$ProcessingWarning$source(void* self);
void* __swift_bridge__$ProcessingWarning$message(void* self);
void* __swift_bridge__$LlmUsage$new(void* model, void* source, struct __private__OptionU64 input_tokens, struct __private__OptionU64 output_tokens, struct __private__OptionU64 total_tokens, struct __private__OptionF64 estimated_cost, void* finish_reason);
void* __swift_bridge__$LlmUsage$model(void* self);
void* __swift_bridge__$LlmUsage$source(void* self);
struct __private__OptionU64 __swift_bridge__$LlmUsage$input_tokens(void* self);
struct __private__OptionU64 __swift_bridge__$LlmUsage$output_tokens(void* self);
struct __private__OptionU64 __swift_bridge__$LlmUsage$total_tokens(void* self);
struct __private__OptionF64 __swift_bridge__$LlmUsage$estimated_cost(void* self);
void* __swift_bridge__$LlmUsage$finish_reason(void* self);
void* __swift_bridge__$Chunk$new(void* content, void* chunk_type, void* embedding, void* metadata);
void* __swift_bridge__$Chunk$content(void* self);
void* __swift_bridge__$Chunk$chunk_type(void* self);
void* __swift_bridge__$Chunk$embedding(void* self);
void* __swift_bridge__$Chunk$metadata(void* self);
void* __swift_bridge__$HeadingContext$new(void* headings);
void* __swift_bridge__$HeadingContext$headings(void* self);
void* __swift_bridge__$HeadingLevel$new(uint8_t level, void* text);
uint8_t __swift_bridge__$HeadingLevel$level(void* self);
void* __swift_bridge__$HeadingLevel$text(void* self);
void* __swift_bridge__$ChunkMetadata$new(uintptr_t byte_start, uintptr_t byte_end, struct __private__OptionUsize token_count, uintptr_t chunk_index, uintptr_t total_chunks, struct __private__OptionUsize first_page, struct __private__OptionUsize last_page, void* heading_context);
uintptr_t __swift_bridge__$ChunkMetadata$byte_start(void* self);
uintptr_t __swift_bridge__$ChunkMetadata$byte_end(void* self);
struct __private__OptionUsize __swift_bridge__$ChunkMetadata$token_count(void* self);
uintptr_t __swift_bridge__$ChunkMetadata$chunk_index(void* self);
uintptr_t __swift_bridge__$ChunkMetadata$total_chunks(void* self);
struct __private__OptionUsize __swift_bridge__$ChunkMetadata$first_page(void* self);
struct __private__OptionUsize __swift_bridge__$ChunkMetadata$last_page(void* self);
void* __swift_bridge__$ChunkMetadata$heading_context(void* self);
void* __swift_bridge__$ExtractedImage$new(void* data, void* format, uintptr_t image_index, struct __private__OptionUsize page_number, struct __private__OptionU32 width, struct __private__OptionU32 height, void* colorspace, struct __private__OptionU32 bits_per_component, bool is_mask, void* description, void* ocr_result, void* bounding_box, void* source_path);
void* __swift_bridge__$ExtractedImage$data(void* self);
void* __swift_bridge__$ExtractedImage$format(void* self);
uintptr_t __swift_bridge__$ExtractedImage$image_index(void* self);
struct __private__OptionUsize __swift_bridge__$ExtractedImage$page_number(void* self);
struct __private__OptionU32 __swift_bridge__$ExtractedImage$width(void* self);
struct __private__OptionU32 __swift_bridge__$ExtractedImage$height(void* self);
void* __swift_bridge__$ExtractedImage$colorspace(void* self);
struct __private__OptionU32 __swift_bridge__$ExtractedImage$bits_per_component(void* self);
bool __swift_bridge__$ExtractedImage$is_mask(void* self);
void* __swift_bridge__$ExtractedImage$description(void* self);
void* __swift_bridge__$ExtractedImage$ocr_result(void* self);
void* __swift_bridge__$ExtractedImage$bounding_box(void* self);
void* __swift_bridge__$ExtractedImage$source_path(void* self);
void* __swift_bridge__$ElementMetadata$new(struct __private__OptionUsize page_number, void* filename, void* coordinates, struct __private__OptionUsize element_index, void* additional);
struct __private__OptionUsize __swift_bridge__$ElementMetadata$page_number(void* self);
void* __swift_bridge__$ElementMetadata$filename(void* self);
void* __swift_bridge__$ElementMetadata$coordinates(void* self);
struct __private__OptionUsize __swift_bridge__$ElementMetadata$element_index(void* self);
void* __swift_bridge__$ElementMetadata$additional(void* self);
void* __swift_bridge__$Element$new(void* element_id, void* element_type, void* text, void* metadata);
void* __swift_bridge__$Element$element_id(void* self);
void* __swift_bridge__$Element$element_type(void* self);
void* __swift_bridge__$Element$text(void* self);
void* __swift_bridge__$Element$metadata(void* self);
void* __swift_bridge__$ExcelWorkbook$new(void* sheets, void* metadata);
void* __swift_bridge__$ExcelWorkbook$sheets(void* self);
void* __swift_bridge__$ExcelWorkbook$metadata(void* self);
void* __swift_bridge__$ExcelSheet$new(void* name, void* markdown, uintptr_t row_count, uintptr_t col_count, uintptr_t cell_count, void* table_cells);
void* __swift_bridge__$ExcelSheet$name(void* self);
void* __swift_bridge__$ExcelSheet$markdown(void* self);
uintptr_t __swift_bridge__$ExcelSheet$row_count(void* self);
uintptr_t __swift_bridge__$ExcelSheet$col_count(void* self);
uintptr_t __swift_bridge__$ExcelSheet$cell_count(void* self);
void* __swift_bridge__$ExcelSheet$table_cells(void* self);
void* __swift_bridge__$XmlExtractionResult$new(void* content, uintptr_t element_count, void* unique_elements);
void* __swift_bridge__$XmlExtractionResult$content(void* self);
uintptr_t __swift_bridge__$XmlExtractionResult$element_count(void* self);
void* __swift_bridge__$XmlExtractionResult$unique_elements(void* self);
void* __swift_bridge__$TextExtractionResult$new(void* content, uintptr_t line_count, uintptr_t word_count, uintptr_t character_count, void* headers, void* links, void* code_blocks);
void* __swift_bridge__$TextExtractionResult$content(void* self);
uintptr_t __swift_bridge__$TextExtractionResult$line_count(void* self);
uintptr_t __swift_bridge__$TextExtractionResult$word_count(void* self);
uintptr_t __swift_bridge__$TextExtractionResult$character_count(void* self);
void* __swift_bridge__$TextExtractionResult$headers(void* self);
void* __swift_bridge__$TextExtractionResult$links(void* self);
void* __swift_bridge__$TextExtractionResult$code_blocks(void* self);
void* __swift_bridge__$PptxExtractionResult$new(void* content, void* metadata, uintptr_t slide_count, uintptr_t image_count, uintptr_t table_count, void* images, void* page_structure, void* page_contents, void* document, void* hyperlinks, void* office_metadata);
void* __swift_bridge__$PptxExtractionResult$content(void* self);
void* __swift_bridge__$PptxExtractionResult$metadata(void* self);
uintptr_t __swift_bridge__$PptxExtractionResult$slide_count(void* self);
uintptr_t __swift_bridge__$PptxExtractionResult$image_count(void* self);
uintptr_t __swift_bridge__$PptxExtractionResult$table_count(void* self);
void* __swift_bridge__$PptxExtractionResult$images(void* self);
void* __swift_bridge__$PptxExtractionResult$page_structure(void* self);
void* __swift_bridge__$PptxExtractionResult$page_contents(void* self);
void* __swift_bridge__$PptxExtractionResult$document(void* self);
void* __swift_bridge__$PptxExtractionResult$hyperlinks(void* self);
void* __swift_bridge__$PptxExtractionResult$office_metadata(void* self);
void* __swift_bridge__$EmailExtractionResult$new(void* subject, void* from_email, void* to_emails, void* cc_emails, void* bcc_emails, void* date, void* message_id, void* plain_text, void* html_content, void* cleaned_text, void* attachments, void* metadata);
void* __swift_bridge__$EmailExtractionResult$subject(void* self);
void* __swift_bridge__$EmailExtractionResult$from_email(void* self);
void* __swift_bridge__$EmailExtractionResult$to_emails(void* self);
void* __swift_bridge__$EmailExtractionResult$cc_emails(void* self);
void* __swift_bridge__$EmailExtractionResult$bcc_emails(void* self);
void* __swift_bridge__$EmailExtractionResult$date(void* self);
void* __swift_bridge__$EmailExtractionResult$message_id(void* self);
void* __swift_bridge__$EmailExtractionResult$plain_text(void* self);
void* __swift_bridge__$EmailExtractionResult$html_content(void* self);
void* __swift_bridge__$EmailExtractionResult$cleaned_text(void* self);
void* __swift_bridge__$EmailExtractionResult$attachments(void* self);
void* __swift_bridge__$EmailExtractionResult$metadata(void* self);
void* __swift_bridge__$EmailAttachment$new(void* name, void* filename, void* mime_type, struct __private__OptionUsize size, bool is_image, void* data);
void* __swift_bridge__$EmailAttachment$name(void* self);
void* __swift_bridge__$EmailAttachment$filename(void* self);
void* __swift_bridge__$EmailAttachment$mime_type(void* self);
struct __private__OptionUsize __swift_bridge__$EmailAttachment$size(void* self);
bool __swift_bridge__$EmailAttachment$is_image(void* self);
void* __swift_bridge__$EmailAttachment$data(void* self);
void* __swift_bridge__$OcrExtractionResult$new(void* content, void* mime_type, void* metadata, void* tables, void* ocr_elements);
void* __swift_bridge__$OcrExtractionResult$content(void* self);
void* __swift_bridge__$OcrExtractionResult$mime_type(void* self);
void* __swift_bridge__$OcrExtractionResult$metadata(void* self);
void* __swift_bridge__$OcrExtractionResult$tables(void* self);
void* __swift_bridge__$OcrExtractionResult$ocr_elements(void* self);
void* __swift_bridge__$OcrExtractionResult$internal_document(void* self);
void* __swift_bridge__$OcrTable$new(void* cells, void* markdown, uintptr_t page_number, void* bounding_box);
void* __swift_bridge__$OcrTable$cells(void* self);
void* __swift_bridge__$OcrTable$markdown(void* self);
uintptr_t __swift_bridge__$OcrTable$page_number(void* self);
void* __swift_bridge__$OcrTable$bounding_box(void* self);
void* __swift_bridge__$OcrTableBoundingBox$new(uint32_t left, uint32_t top, uint32_t right, uint32_t bottom);
uint32_t __swift_bridge__$OcrTableBoundingBox$left(void* self);
uint32_t __swift_bridge__$OcrTableBoundingBox$top(void* self);
uint32_t __swift_bridge__$OcrTableBoundingBox$right(void* self);
uint32_t __swift_bridge__$OcrTableBoundingBox$bottom(void* self);
void* __swift_bridge__$ImagePreprocessingConfig$new(int32_t target_dpi, bool auto_rotate, bool deskew, bool denoise, bool contrast_enhance, void* binarization_method, bool invert_colors);
int32_t __swift_bridge__$ImagePreprocessingConfig$target_dpi(void* self);
bool __swift_bridge__$ImagePreprocessingConfig$auto_rotate(void* self);
bool __swift_bridge__$ImagePreprocessingConfig$deskew(void* self);
bool __swift_bridge__$ImagePreprocessingConfig$denoise(void* self);
bool __swift_bridge__$ImagePreprocessingConfig$contrast_enhance(void* self);
void* __swift_bridge__$ImagePreprocessingConfig$binarization_method(void* self);
bool __swift_bridge__$ImagePreprocessingConfig$invert_colors(void* self);
void* __swift_bridge__$TesseractConfig$new(void* language, int32_t psm, void* output_format, int32_t oem, double min_confidence, void* preprocessing, bool enable_table_detection, double table_min_confidence, int32_t table_column_threshold, double table_row_threshold_ratio, bool use_cache, bool classify_use_pre_adapted_templates, bool language_model_ngram_on, bool tessedit_dont_blkrej_good_wds, bool tessedit_dont_rowrej_good_wds, bool tessedit_enable_dict_correction, void* tessedit_char_whitelist, void* tessedit_char_blacklist, bool tessedit_use_primary_params_model, bool textord_space_size_is_variable, bool thresholding_method);
void* __swift_bridge__$TesseractConfig$language(void* self);
int32_t __swift_bridge__$TesseractConfig$psm(void* self);
void* __swift_bridge__$TesseractConfig$output_format(void* self);
int32_t __swift_bridge__$TesseractConfig$oem(void* self);
double __swift_bridge__$TesseractConfig$min_confidence(void* self);
void* __swift_bridge__$TesseractConfig$preprocessing(void* self);
bool __swift_bridge__$TesseractConfig$enable_table_detection(void* self);
double __swift_bridge__$TesseractConfig$table_min_confidence(void* self);
int32_t __swift_bridge__$TesseractConfig$table_column_threshold(void* self);
double __swift_bridge__$TesseractConfig$table_row_threshold_ratio(void* self);
bool __swift_bridge__$TesseractConfig$use_cache(void* self);
bool __swift_bridge__$TesseractConfig$classify_use_pre_adapted_templates(void* self);
bool __swift_bridge__$TesseractConfig$language_model_ngram_on(void* self);
bool __swift_bridge__$TesseractConfig$tessedit_dont_blkrej_good_wds(void* self);
bool __swift_bridge__$TesseractConfig$tessedit_dont_rowrej_good_wds(void* self);
bool __swift_bridge__$TesseractConfig$tessedit_enable_dict_correction(void* self);
void* __swift_bridge__$TesseractConfig$tessedit_char_whitelist(void* self);
void* __swift_bridge__$TesseractConfig$tessedit_char_blacklist(void* self);
bool __swift_bridge__$TesseractConfig$tessedit_use_primary_params_model(void* self);
bool __swift_bridge__$TesseractConfig$textord_space_size_is_variable(void* self);
bool __swift_bridge__$TesseractConfig$thresholding_method(void* self);
void* __swift_bridge__$ImagePreprocessingMetadata$new(void* original_dimensions, void* original_dpi, int32_t target_dpi, double scale_factor, bool auto_adjusted, int32_t final_dpi, void* new_dimensions, void* resample_method, bool dimension_clamped, struct __private__OptionI32 calculated_dpi, bool skipped_resize, void* resize_error);
void* __swift_bridge__$ImagePreprocessingMetadata$original_dimensions(void* self);
void* __swift_bridge__$ImagePreprocessingMetadata$original_dpi(void* self);
int32_t __swift_bridge__$ImagePreprocessingMetadata$target_dpi(void* self);
double __swift_bridge__$ImagePreprocessingMetadata$scale_factor(void* self);
bool __swift_bridge__$ImagePreprocessingMetadata$auto_adjusted(void* self);
int32_t __swift_bridge__$ImagePreprocessingMetadata$final_dpi(void* self);
void* __swift_bridge__$ImagePreprocessingMetadata$new_dimensions(void* self);
void* __swift_bridge__$ImagePreprocessingMetadata$resample_method(void* self);
bool __swift_bridge__$ImagePreprocessingMetadata$dimension_clamped(void* self);
struct __private__OptionI32 __swift_bridge__$ImagePreprocessingMetadata$calculated_dpi(void* self);
bool __swift_bridge__$ImagePreprocessingMetadata$skipped_resize(void* self);
void* __swift_bridge__$ImagePreprocessingMetadata$resize_error(void* self);
void* __swift_bridge__$Metadata$new(void* title, void* subject, void* authors, void* keywords, void* language, void* created_at, void* modified_at, void* created_by, void* modified_by, void* pages, void* format, void* image_preprocessing, void* json_schema, void* error, struct __private__OptionU64 extraction_duration_ms, void* category, void* tags, void* document_version, void* abstract_text, void* output_format, void* additional);
void* __swift_bridge__$Metadata$title(void* self);
void* __swift_bridge__$Metadata$subject(void* self);
void* __swift_bridge__$Metadata$authors(void* self);
void* __swift_bridge__$Metadata$keywords(void* self);
void* __swift_bridge__$Metadata$language(void* self);
void* __swift_bridge__$Metadata$created_at(void* self);
void* __swift_bridge__$Metadata$modified_at(void* self);
void* __swift_bridge__$Metadata$created_by(void* self);
void* __swift_bridge__$Metadata$modified_by(void* self);
void* __swift_bridge__$Metadata$pages(void* self);
void* __swift_bridge__$Metadata$format(void* self);
void* __swift_bridge__$Metadata$image_preprocessing(void* self);
void* __swift_bridge__$Metadata$json_schema(void* self);
void* __swift_bridge__$Metadata$error(void* self);
struct __private__OptionU64 __swift_bridge__$Metadata$extraction_duration_ms(void* self);
void* __swift_bridge__$Metadata$category(void* self);
void* __swift_bridge__$Metadata$tags(void* self);
void* __swift_bridge__$Metadata$document_version(void* self);
void* __swift_bridge__$Metadata$abstract_text(void* self);
void* __swift_bridge__$Metadata$output_format(void* self);
void* __swift_bridge__$Metadata$additional(void* self);
void* __swift_bridge__$ExcelMetadata$new(uintptr_t sheet_count, void* sheet_names);
uintptr_t __swift_bridge__$ExcelMetadata$sheet_count(void* self);
void* __swift_bridge__$ExcelMetadata$sheet_names(void* self);
void* __swift_bridge__$EmailMetadata$new(void* from_email, void* from_name, void* to_emails, void* cc_emails, void* bcc_emails, void* message_id, void* attachments);
void* __swift_bridge__$EmailMetadata$from_email(void* self);
void* __swift_bridge__$EmailMetadata$from_name(void* self);
void* __swift_bridge__$EmailMetadata$to_emails(void* self);
void* __swift_bridge__$EmailMetadata$cc_emails(void* self);
void* __swift_bridge__$EmailMetadata$bcc_emails(void* self);
void* __swift_bridge__$EmailMetadata$message_id(void* self);
void* __swift_bridge__$EmailMetadata$attachments(void* self);
void* __swift_bridge__$ArchiveMetadata$new(void* format, uintptr_t file_count, void* file_list, uintptr_t total_size, struct __private__OptionUsize compressed_size);
void* __swift_bridge__$ArchiveMetadata$format(void* self);
uintptr_t __swift_bridge__$ArchiveMetadata$file_count(void* self);
void* __swift_bridge__$ArchiveMetadata$file_list(void* self);
uintptr_t __swift_bridge__$ArchiveMetadata$total_size(void* self);
struct __private__OptionUsize __swift_bridge__$ArchiveMetadata$compressed_size(void* self);
void* __swift_bridge__$XmlMetadata$new(uintptr_t element_count, void* unique_elements);
uintptr_t __swift_bridge__$XmlMetadata$element_count(void* self);
void* __swift_bridge__$XmlMetadata$unique_elements(void* self);
void* __swift_bridge__$TextMetadata$new(uintptr_t line_count, uintptr_t word_count, uintptr_t character_count, void* headers, void* links, void* code_blocks);
uintptr_t __swift_bridge__$TextMetadata$line_count(void* self);
uintptr_t __swift_bridge__$TextMetadata$word_count(void* self);
uintptr_t __swift_bridge__$TextMetadata$character_count(void* self);
void* __swift_bridge__$TextMetadata$headers(void* self);
void* __swift_bridge__$TextMetadata$links(void* self);
void* __swift_bridge__$TextMetadata$code_blocks(void* self);
void* __swift_bridge__$HeaderMetadata$new(uint8_t level, void* text, void* id, uintptr_t depth, uintptr_t html_offset);
uint8_t __swift_bridge__$HeaderMetadata$level(void* self);
void* __swift_bridge__$HeaderMetadata$text(void* self);
void* __swift_bridge__$HeaderMetadata$id(void* self);
uintptr_t __swift_bridge__$HeaderMetadata$depth(void* self);
uintptr_t __swift_bridge__$HeaderMetadata$html_offset(void* self);
void* __swift_bridge__$LinkMetadata$new(void* href, void* text, void* title, void* link_type, void* rel, void* attributes);
void* __swift_bridge__$LinkMetadata$href(void* self);
void* __swift_bridge__$LinkMetadata$text(void* self);
void* __swift_bridge__$LinkMetadata$title(void* self);
void* __swift_bridge__$LinkMetadata$link_type(void* self);
void* __swift_bridge__$LinkMetadata$rel(void* self);
void* __swift_bridge__$LinkMetadata$attributes(void* self);
void* __swift_bridge__$ImageMetadataType$new(void* src, void* alt, void* title, void* dimensions, void* image_type, void* attributes);
void* __swift_bridge__$ImageMetadataType$src(void* self);
void* __swift_bridge__$ImageMetadataType$alt(void* self);
void* __swift_bridge__$ImageMetadataType$title(void* self);
void* __swift_bridge__$ImageMetadataType$dimensions(void* self);
void* __swift_bridge__$ImageMetadataType$image_type(void* self);
void* __swift_bridge__$ImageMetadataType$attributes(void* self);
void* __swift_bridge__$StructuredData$new(void* data_type, void* raw_json, void* schema_type);
void* __swift_bridge__$StructuredData$data_type(void* self);
void* __swift_bridge__$StructuredData$raw_json(void* self);
void* __swift_bridge__$StructuredData$schema_type(void* self);
void* __swift_bridge__$HtmlMetadata$new(void* title, void* description, void* keywords, void* author, void* canonical_url, void* base_href, void* language, void* text_direction, void* open_graph, void* twitter_card, void* meta_tags, void* headers, void* links, void* images, void* structured_data);
void* __swift_bridge__$HtmlMetadata$title(void* self);
void* __swift_bridge__$HtmlMetadata$description(void* self);
void* __swift_bridge__$HtmlMetadata$keywords(void* self);
void* __swift_bridge__$HtmlMetadata$author(void* self);
void* __swift_bridge__$HtmlMetadata$canonical_url(void* self);
void* __swift_bridge__$HtmlMetadata$base_href(void* self);
void* __swift_bridge__$HtmlMetadata$language(void* self);
void* __swift_bridge__$HtmlMetadata$text_direction(void* self);
void* __swift_bridge__$HtmlMetadata$open_graph(void* self);
void* __swift_bridge__$HtmlMetadata$twitter_card(void* self);
void* __swift_bridge__$HtmlMetadata$meta_tags(void* self);
void* __swift_bridge__$HtmlMetadata$headers(void* self);
void* __swift_bridge__$HtmlMetadata$links(void* self);
void* __swift_bridge__$HtmlMetadata$images(void* self);
void* __swift_bridge__$HtmlMetadata$structured_data(void* self);
void* __swift_bridge__$OcrMetadata$new(void* language, int32_t psm, void* output_format, uintptr_t table_count, struct __private__OptionUsize table_rows, struct __private__OptionUsize table_cols);
void* __swift_bridge__$OcrMetadata$language(void* self);
int32_t __swift_bridge__$OcrMetadata$psm(void* self);
void* __swift_bridge__$OcrMetadata$output_format(void* self);
uintptr_t __swift_bridge__$OcrMetadata$table_count(void* self);
struct __private__OptionUsize __swift_bridge__$OcrMetadata$table_rows(void* self);
struct __private__OptionUsize __swift_bridge__$OcrMetadata$table_cols(void* self);
void* __swift_bridge__$ErrorMetadata$new(void* error_type, void* message);
void* __swift_bridge__$ErrorMetadata$error_type(void* self);
void* __swift_bridge__$ErrorMetadata$message(void* self);
void* __swift_bridge__$PptxMetadata$new(uintptr_t slide_count, void* slide_names, struct __private__OptionUsize image_count, struct __private__OptionUsize table_count);
uintptr_t __swift_bridge__$PptxMetadata$slide_count(void* self);
void* __swift_bridge__$PptxMetadata$slide_names(void* self);
struct __private__OptionUsize __swift_bridge__$PptxMetadata$image_count(void* self);
struct __private__OptionUsize __swift_bridge__$PptxMetadata$table_count(void* self);
void* __swift_bridge__$DocxMetadata$new(void* core_properties, void* app_properties, void* custom_properties);
void* __swift_bridge__$DocxMetadata$core_properties(void* self);
void* __swift_bridge__$DocxMetadata$app_properties(void* self);
void* __swift_bridge__$DocxMetadata$custom_properties(void* self);
void* __swift_bridge__$CsvMetadata$new(uintptr_t row_count, uintptr_t column_count, void* delimiter, bool has_header, void* column_types);
uintptr_t __swift_bridge__$CsvMetadata$row_count(void* self);
uintptr_t __swift_bridge__$CsvMetadata$column_count(void* self);
void* __swift_bridge__$CsvMetadata$delimiter(void* self);
bool __swift_bridge__$CsvMetadata$has_header(void* self);
void* __swift_bridge__$CsvMetadata$column_types(void* self);
void* __swift_bridge__$BibtexMetadata$new(uintptr_t entry_count, void* citation_keys, void* authors, void* year_range, void* entry_types);
uintptr_t __swift_bridge__$BibtexMetadata$entry_count(void* self);
void* __swift_bridge__$BibtexMetadata$citation_keys(void* self);
void* __swift_bridge__$BibtexMetadata$authors(void* self);
void* __swift_bridge__$BibtexMetadata$year_range(void* self);
void* __swift_bridge__$BibtexMetadata$entry_types(void* self);
void* __swift_bridge__$CitationMetadata$new(uintptr_t citation_count, void* format, void* authors, void* year_range, void* dois, void* keywords);
uintptr_t __swift_bridge__$CitationMetadata$citation_count(void* self);
void* __swift_bridge__$CitationMetadata$format(void* self);
void* __swift_bridge__$CitationMetadata$authors(void* self);
void* __swift_bridge__$CitationMetadata$year_range(void* self);
void* __swift_bridge__$CitationMetadata$dois(void* self);
void* __swift_bridge__$CitationMetadata$keywords(void* self);
void* __swift_bridge__$YearRange$new(struct __private__OptionU32 min, struct __private__OptionU32 max, void* years);
struct __private__OptionU32 __swift_bridge__$YearRange$min(void* self);
struct __private__OptionU32 __swift_bridge__$YearRange$max(void* self);
void* __swift_bridge__$YearRange$years(void* self);
void* __swift_bridge__$FictionBookMetadata$new(void* genres, void* sequences, void* annotation);
void* __swift_bridge__$FictionBookMetadata$genres(void* self);
void* __swift_bridge__$FictionBookMetadata$sequences(void* self);
void* __swift_bridge__$FictionBookMetadata$annotation(void* self);
void* __swift_bridge__$DbfMetadata$new(uintptr_t record_count, uintptr_t field_count, void* fields);
uintptr_t __swift_bridge__$DbfMetadata$record_count(void* self);
uintptr_t __swift_bridge__$DbfMetadata$field_count(void* self);
void* __swift_bridge__$DbfMetadata$fields(void* self);
void* __swift_bridge__$DbfFieldInfo$new(void* name, void* field_type);
void* __swift_bridge__$DbfFieldInfo$name(void* self);
void* __swift_bridge__$DbfFieldInfo$field_type(void* self);
void* __swift_bridge__$JatsMetadata$new(void* copyright, void* license, void* history_dates, void* contributor_roles);
void* __swift_bridge__$JatsMetadata$copyright(void* self);
void* __swift_bridge__$JatsMetadata$license(void* self);
void* __swift_bridge__$JatsMetadata$history_dates(void* self);
void* __swift_bridge__$JatsMetadata$contributor_roles(void* self);
void* __swift_bridge__$ContributorRole$new(void* name, void* role);
void* __swift_bridge__$ContributorRole$name(void* self);
void* __swift_bridge__$ContributorRole$role(void* self);
void* __swift_bridge__$EpubMetadata$new(void* coverage, void* dc_format, void* relation, void* source, void* dc_type, void* cover_image);
void* __swift_bridge__$EpubMetadata$coverage(void* self);
void* __swift_bridge__$EpubMetadata$dc_format(void* self);
void* __swift_bridge__$EpubMetadata$relation(void* self);
void* __swift_bridge__$EpubMetadata$source(void* self);
void* __swift_bridge__$EpubMetadata$dc_type(void* self);
void* __swift_bridge__$EpubMetadata$cover_image(void* self);
void* __swift_bridge__$PstMetadata$new(uintptr_t message_count);
uintptr_t __swift_bridge__$PstMetadata$message_count(void* self);
void* __swift_bridge__$OcrConfidence$new(struct __private__OptionF64 detection, double recognition);
struct __private__OptionF64 __swift_bridge__$OcrConfidence$detection(void* self);
double __swift_bridge__$OcrConfidence$recognition(void* self);
void* __swift_bridge__$OcrRotation$new(double angle_degrees, struct __private__OptionF64 confidence);
double __swift_bridge__$OcrRotation$angle_degrees(void* self);
struct __private__OptionF64 __swift_bridge__$OcrRotation$confidence(void* self);
void* __swift_bridge__$OcrElement$new(void* text, void* geometry, void* confidence, void* level, void* rotation, uintptr_t page_number, void* parent_id, void* backend_metadata);
void* __swift_bridge__$OcrElement$text(void* self);
void* __swift_bridge__$OcrElement$geometry(void* self);
void* __swift_bridge__$OcrElement$confidence(void* self);
void* __swift_bridge__$OcrElement$level(void* self);
void* __swift_bridge__$OcrElement$rotation(void* self);
uintptr_t __swift_bridge__$OcrElement$page_number(void* self);
void* __swift_bridge__$OcrElement$parent_id(void* self);
void* __swift_bridge__$OcrElement$backend_metadata(void* self);
void* __swift_bridge__$OcrElementConfig$new(bool include_elements, void* min_level, double min_confidence, bool build_hierarchy);
bool __swift_bridge__$OcrElementConfig$include_elements(void* self);
void* __swift_bridge__$OcrElementConfig$min_level(void* self);
double __swift_bridge__$OcrElementConfig$min_confidence(void* self);
bool __swift_bridge__$OcrElementConfig$build_hierarchy(void* self);
void* __swift_bridge__$PageStructure$new(uintptr_t total_count, void* unit_type, void* boundaries, void* pages);
uintptr_t __swift_bridge__$PageStructure$total_count(void* self);
void* __swift_bridge__$PageStructure$unit_type(void* self);
void* __swift_bridge__$PageStructure$boundaries(void* self);
void* __swift_bridge__$PageStructure$pages(void* self);
void* __swift_bridge__$PageBoundary$new(uintptr_t byte_start, uintptr_t byte_end, uintptr_t page_number);
uintptr_t __swift_bridge__$PageBoundary$byte_start(void* self);
uintptr_t __swift_bridge__$PageBoundary$byte_end(void* self);
uintptr_t __swift_bridge__$PageBoundary$page_number(void* self);
void* __swift_bridge__$PageInfo$new(uintptr_t number, void* title, void* dimensions, struct __private__OptionUsize image_count, struct __private__OptionUsize table_count, struct __private__OptionBool hidden, struct __private__OptionBool is_blank);
uintptr_t __swift_bridge__$PageInfo$number(void* self);
void* __swift_bridge__$PageInfo$title(void* self);
void* __swift_bridge__$PageInfo$dimensions(void* self);
struct __private__OptionUsize __swift_bridge__$PageInfo$image_count(void* self);
struct __private__OptionUsize __swift_bridge__$PageInfo$table_count(void* self);
struct __private__OptionBool __swift_bridge__$PageInfo$hidden(void* self);
struct __private__OptionBool __swift_bridge__$PageInfo$is_blank(void* self);
void* __swift_bridge__$PageContent$new(uintptr_t page_number, void* content, void* tables, void* images, void* hierarchy, struct __private__OptionBool is_blank, void* layout_regions);
uintptr_t __swift_bridge__$PageContent$page_number(void* self);
void* __swift_bridge__$PageContent$content(void* self);
void* __swift_bridge__$PageContent$tables(void* self);
void* __swift_bridge__$PageContent$images(void* self);
void* __swift_bridge__$PageContent$hierarchy(void* self);
struct __private__OptionBool __swift_bridge__$PageContent$is_blank(void* self);
void* __swift_bridge__$PageContent$layout_regions(void* self);
void* __swift_bridge__$LayoutRegion$new(void* class_name, double confidence, void* bounding_box, double area_fraction);
void* __swift_bridge__$LayoutRegion$class_name(void* self);
double __swift_bridge__$LayoutRegion$confidence(void* self);
void* __swift_bridge__$LayoutRegion$bounding_box(void* self);
double __swift_bridge__$LayoutRegion$area_fraction(void* self);
void* __swift_bridge__$PageHierarchy$new(uintptr_t block_count, void* blocks);
uintptr_t __swift_bridge__$PageHierarchy$block_count(void* self);
void* __swift_bridge__$PageHierarchy$blocks(void* self);
void* __swift_bridge__$HierarchicalBlock$new(void* text, float font_size, void* level, void* bbox);
void* __swift_bridge__$HierarchicalBlock$text(void* self);
float __swift_bridge__$HierarchicalBlock$font_size(void* self);
void* __swift_bridge__$HierarchicalBlock$level(void* self);
void* __swift_bridge__$HierarchicalBlock$bbox(void* self);
void* __swift_bridge__$Uri$new(void* url, void* label, struct __private__OptionU32 page, void* kind);
void* __swift_bridge__$Uri$url(void* self);
void* __swift_bridge__$Uri$label(void* self);
struct __private__OptionU32 __swift_bridge__$Uri$page(void* self);
void* __swift_bridge__$Uri$kind(void* self);
void* __swift_bridge__$HealthResponse$new(void* status, void* version, void* plugins);
void* __swift_bridge__$HealthResponse$status(void* self);
void* __swift_bridge__$HealthResponse$version(void* self);
void* __swift_bridge__$HealthResponse$plugins(void* self);
void* __swift_bridge__$InfoResponse$new(void* version, bool rust_backend);
void* __swift_bridge__$InfoResponse$version(void* self);
bool __swift_bridge__$InfoResponse$rust_backend(void* self);
void* __swift_bridge__$ApiState$new(void* default_config, void* extraction_service);
void* __swift_bridge__$ApiState$default_config(void* self);
void* __swift_bridge__$ApiState$extraction_service(void* self);
void* __swift_bridge__$CacheStatsResponse$new(void* directory, uintptr_t total_files, double total_size_mb, double available_space_mb, double oldest_file_age_days, double newest_file_age_days);
void* __swift_bridge__$CacheStatsResponse$directory(void* self);
uintptr_t __swift_bridge__$CacheStatsResponse$total_files(void* self);
double __swift_bridge__$CacheStatsResponse$total_size_mb(void* self);
double __swift_bridge__$CacheStatsResponse$available_space_mb(void* self);
double __swift_bridge__$CacheStatsResponse$oldest_file_age_days(void* self);
double __swift_bridge__$CacheStatsResponse$newest_file_age_days(void* self);
void* __swift_bridge__$CacheClearResponse$new(void* directory, uintptr_t removed_files, double freed_mb);
void* __swift_bridge__$CacheClearResponse$directory(void* self);
uintptr_t __swift_bridge__$CacheClearResponse$removed_files(void* self);
double __swift_bridge__$CacheClearResponse$freed_mb(void* self);
void* __swift_bridge__$EmbedRequest$new(void* texts, void* config);
void* __swift_bridge__$EmbedRequest$texts(void* self);
void* __swift_bridge__$EmbedRequest$config(void* self);
void* __swift_bridge__$EmbedResponse$new(void* embeddings, void* model, uintptr_t dimensions, uintptr_t count);
void* __swift_bridge__$EmbedResponse$embeddings(void* self);
void* __swift_bridge__$EmbedResponse$model(void* self);
uintptr_t __swift_bridge__$EmbedResponse$dimensions(void* self);
uintptr_t __swift_bridge__$EmbedResponse$count(void* self);
void* __swift_bridge__$ChunkRequest$new(void* text, void* config, void* chunker_type);
void* __swift_bridge__$ChunkRequest$text(void* self);
void* __swift_bridge__$ChunkRequest$config(void* self);
void* __swift_bridge__$ChunkRequest$chunker_type(void* self);
void* __swift_bridge__$ChunkResponse$new(void* chunks, uintptr_t chunk_count, void* config, uintptr_t input_size_bytes, void* chunker_type);
void* __swift_bridge__$ChunkResponse$chunks(void* self);
uintptr_t __swift_bridge__$ChunkResponse$chunk_count(void* self);
void* __swift_bridge__$ChunkResponse$config(void* self);
uintptr_t __swift_bridge__$ChunkResponse$input_size_bytes(void* self);
void* __swift_bridge__$ChunkResponse$chunker_type(void* self);
void* __swift_bridge__$VersionResponse$new(void* version);
void* __swift_bridge__$VersionResponse$version(void* self);
void* __swift_bridge__$DetectResponse$new(void* mime_type, void* filename);
void* __swift_bridge__$DetectResponse$mime_type(void* self);
void* __swift_bridge__$DetectResponse$filename(void* self);
void* __swift_bridge__$ManifestEntryResponse$new(void* relative_path, void* sha256, uint64_t size_bytes, void* source_url);
void* __swift_bridge__$ManifestEntryResponse$relative_path(void* self);
void* __swift_bridge__$ManifestEntryResponse$sha256(void* self);
uint64_t __swift_bridge__$ManifestEntryResponse$size_bytes(void* self);
void* __swift_bridge__$ManifestEntryResponse$source_url(void* self);
void* __swift_bridge__$ManifestResponse$new(void* kreuzberg_version, uint64_t total_size_bytes, uintptr_t model_count, void* models);
void* __swift_bridge__$ManifestResponse$kreuzberg_version(void* self);
uint64_t __swift_bridge__$ManifestResponse$total_size_bytes(void* self);
uintptr_t __swift_bridge__$ManifestResponse$model_count(void* self);
void* __swift_bridge__$ManifestResponse$models(void* self);
void* __swift_bridge__$WarmRequest$new(bool all_embeddings, void* embedding_model);
bool __swift_bridge__$WarmRequest$all_embeddings(void* self);
void* __swift_bridge__$WarmRequest$embedding_model(void* self);
void* __swift_bridge__$WarmResponse$new(void* cache_dir, void* downloaded, void* already_cached);
void* __swift_bridge__$WarmResponse$cache_dir(void* self);
void* __swift_bridge__$WarmResponse$downloaded(void* self);
void* __swift_bridge__$WarmResponse$already_cached(void* self);
void* __swift_bridge__$StructuredExtractionResponse$new(void* structured_output, void* content, void* mime_type);
void* __swift_bridge__$StructuredExtractionResponse$structured_output(void* self);
void* __swift_bridge__$StructuredExtractionResponse$content(void* self);
void* __swift_bridge__$StructuredExtractionResponse$mime_type(void* self);
void* __swift_bridge__$OpenWebDocumentResponse$new(void* page_content, void* metadata);
void* __swift_bridge__$OpenWebDocumentResponse$page_content(void* self);
void* __swift_bridge__$OpenWebDocumentResponse$metadata(void* self);
void* __swift_bridge__$DoclingCompatResponse$new(void* document, void* status);
void* __swift_bridge__$DoclingCompatResponse$document(void* self);
void* __swift_bridge__$DoclingCompatResponse$status(void* self);
void* __swift_bridge__$ExtractFileParams$new(void* path, void* mime_type, void* config, void* pdf_password, void* response_format);
void* __swift_bridge__$ExtractFileParams$path(void* self);
void* __swift_bridge__$ExtractFileParams$mime_type(void* self);
void* __swift_bridge__$ExtractFileParams$config(void* self);
void* __swift_bridge__$ExtractFileParams$pdf_password(void* self);
void* __swift_bridge__$ExtractFileParams$response_format(void* self);
void* __swift_bridge__$ExtractBytesParams$new(void* data, void* mime_type, void* config, void* pdf_password, void* response_format);
void* __swift_bridge__$ExtractBytesParams$data(void* self);
void* __swift_bridge__$ExtractBytesParams$mime_type(void* self);
void* __swift_bridge__$ExtractBytesParams$config(void* self);
void* __swift_bridge__$ExtractBytesParams$pdf_password(void* self);
void* __swift_bridge__$ExtractBytesParams$response_format(void* self);
void* __swift_bridge__$BatchExtractFilesParams$new(void* paths, void* config, void* pdf_password, void* file_configs, void* response_format);
void* __swift_bridge__$BatchExtractFilesParams$paths(void* self);
void* __swift_bridge__$BatchExtractFilesParams$config(void* self);
void* __swift_bridge__$BatchExtractFilesParams$pdf_password(void* self);
void* __swift_bridge__$BatchExtractFilesParams$file_configs(void* self);
void* __swift_bridge__$BatchExtractFilesParams$response_format(void* self);
void* __swift_bridge__$DetectMimeTypeParams$new(void* path, bool use_content);
void* __swift_bridge__$DetectMimeTypeParams$path(void* self);
bool __swift_bridge__$DetectMimeTypeParams$use_content(void* self);
void* __swift_bridge__$CacheWarmParams$new(bool all_embeddings, void* embedding_model);
bool __swift_bridge__$CacheWarmParams$all_embeddings(void* self);
void* __swift_bridge__$CacheWarmParams$embedding_model(void* self);
void* __swift_bridge__$EmbedTextParams$new(void* texts, void* preset, void* model, void* api_key, void* embedding_plugin);
void* __swift_bridge__$EmbedTextParams$texts(void* self);
void* __swift_bridge__$EmbedTextParams$preset(void* self);
void* __swift_bridge__$EmbedTextParams$model(void* self);
void* __swift_bridge__$EmbedTextParams$api_key(void* self);
void* __swift_bridge__$EmbedTextParams$embedding_plugin(void* self);
void* __swift_bridge__$ExtractStructuredParams$new(void* path, void* schema, void* model, void* schema_name, void* schema_description, void* prompt, void* api_key, bool strict);
void* __swift_bridge__$ExtractStructuredParams$path(void* self);
void* __swift_bridge__$ExtractStructuredParams$schema(void* self);
void* __swift_bridge__$ExtractStructuredParams$model(void* self);
void* __swift_bridge__$ExtractStructuredParams$schema_name(void* self);
void* __swift_bridge__$ExtractStructuredParams$schema_description(void* self);
void* __swift_bridge__$ExtractStructuredParams$prompt(void* self);
void* __swift_bridge__$ExtractStructuredParams$api_key(void* self);
bool __swift_bridge__$ExtractStructuredParams$strict(void* self);
void* __swift_bridge__$ChunkTextParams$new(void* text, struct __private__OptionUsize max_characters, struct __private__OptionUsize overlap, void* chunker_type, struct __private__OptionF32 topic_threshold);
void* __swift_bridge__$ChunkTextParams$text(void* self);
struct __private__OptionUsize __swift_bridge__$ChunkTextParams$max_characters(void* self);
struct __private__OptionUsize __swift_bridge__$ChunkTextParams$overlap(void* self);
void* __swift_bridge__$ChunkTextParams$chunker_type(void* self);
struct __private__OptionF32 __swift_bridge__$ChunkTextParams$topic_threshold(void* self);
void* __swift_bridge__$DetectedBoundary$new(uintptr_t byte_offset, bool is_header);
uintptr_t __swift_bridge__$DetectedBoundary$byte_offset(void* self);
bool __swift_bridge__$DetectedBoundary$is_header(void* self);
void* __swift_bridge__$ChunkingResult$new(void* chunks, uintptr_t chunk_count);
void* __swift_bridge__$ChunkingResult$chunks(void* self);
uintptr_t __swift_bridge__$ChunkingResult$chunk_count(void* self);
void* __swift_bridge__$MergedChunk$new(void* text, uintptr_t byte_start, uintptr_t byte_end);
void* __swift_bridge__$MergedChunk$text(void* self);
uintptr_t __swift_bridge__$MergedChunk$byte_start(void* self);
uintptr_t __swift_bridge__$MergedChunk$byte_end(void* self);
void* __swift_bridge__$YakeParams$new(uintptr_t window_size);
uintptr_t __swift_bridge__$YakeParams$window_size(void* self);
void* __swift_bridge__$RakeParams$new(uintptr_t min_word_length, uintptr_t max_words_per_phrase);
uintptr_t __swift_bridge__$RakeParams$min_word_length(void* self);
uintptr_t __swift_bridge__$RakeParams$max_words_per_phrase(void* self);
void* __swift_bridge__$KeywordConfig$new(void* algorithm, uintptr_t max_keywords, float min_score, void* ngram_range, void* language, void* yake_params, void* rake_params);
void* __swift_bridge__$KeywordConfig$algorithm(void* self);
uintptr_t __swift_bridge__$KeywordConfig$max_keywords(void* self);
float __swift_bridge__$KeywordConfig$min_score(void* self);
void* __swift_bridge__$KeywordConfig$ngram_range(void* self);
void* __swift_bridge__$KeywordConfig$language(void* self);
void* __swift_bridge__$KeywordConfig$yake_params(void* self);
void* __swift_bridge__$KeywordConfig$rake_params(void* self);
void* __swift_bridge__$Keyword$new(void* text, float score, void* algorithm, void* positions);
void* __swift_bridge__$Keyword$text(void* self);
float __swift_bridge__$Keyword$score(void* self);
void* __swift_bridge__$Keyword$algorithm(void* self);
void* __swift_bridge__$Keyword$positions(void* self);
void* __swift_bridge__$OcrCacheStats$new(uintptr_t total_files, double total_size_mb);
uintptr_t __swift_bridge__$OcrCacheStats$total_files(void* self);
double __swift_bridge__$OcrCacheStats$total_size_mb(void* self);
void* __swift_bridge__$RecognizedTable$new(void* detection_bbox, void* cells, void* markdown);
void* __swift_bridge__$RecognizedTable$detection_bbox(void* self);
void* __swift_bridge__$RecognizedTable$cells(void* self);
void* __swift_bridge__$RecognizedTable$markdown(void* self);
void* __swift_bridge__$PaddleOcrConfig$new(void* language, void* cache_dir, bool use_angle_cls, bool enable_table_detection, float det_db_thresh, float det_db_box_thresh, float det_db_unclip_ratio, uint32_t det_limit_side_len, uint32_t rec_batch_num, uint32_t padding, float drop_score, void* model_tier);
void* __swift_bridge__$PaddleOcrConfig$language(void* self);
void* __swift_bridge__$PaddleOcrConfig$cache_dir(void* self);
bool __swift_bridge__$PaddleOcrConfig$use_angle_cls(void* self);
bool __swift_bridge__$PaddleOcrConfig$enable_table_detection(void* self);
float __swift_bridge__$PaddleOcrConfig$det_db_thresh(void* self);
float __swift_bridge__$PaddleOcrConfig$det_db_box_thresh(void* self);
float __swift_bridge__$PaddleOcrConfig$det_db_unclip_ratio(void* self);
uint32_t __swift_bridge__$PaddleOcrConfig$det_limit_side_len(void* self);
uint32_t __swift_bridge__$PaddleOcrConfig$rec_batch_num(void* self);
uint32_t __swift_bridge__$PaddleOcrConfig$padding(void* self);
float __swift_bridge__$PaddleOcrConfig$drop_score(void* self);
void* __swift_bridge__$PaddleOcrConfig$model_tier(void* self);
void* __swift_bridge__$ModelPaths$new(void* det_model, void* cls_model, void* rec_model, void* dict_file);
void* __swift_bridge__$ModelPaths$det_model(void* self);
void* __swift_bridge__$ModelPaths$cls_model(void* self);
void* __swift_bridge__$ModelPaths$rec_model(void* self);
void* __swift_bridge__$ModelPaths$dict_file(void* self);
void* __swift_bridge__$OrientationResult$new(uint32_t degrees, float confidence);
uint32_t __swift_bridge__$OrientationResult$degrees(void* self);
float __swift_bridge__$OrientationResult$confidence(void* self);
void* __swift_bridge__$BBox$new(float x1, float y1, float x2, float y2);
float __swift_bridge__$BBox$x1(void* self);
float __swift_bridge__$BBox$y1(void* self);
float __swift_bridge__$BBox$x2(void* self);
float __swift_bridge__$BBox$y2(void* self);
void* __swift_bridge__$LayoutDetection$new(void* class_name, float confidence, void* bbox);
void* __swift_bridge__$LayoutDetection$class_name(void* self);
float __swift_bridge__$LayoutDetection$confidence(void* self);
void* __swift_bridge__$LayoutDetection$bbox(void* self);
void* __swift_bridge__$DetectionResult$new(uint32_t page_width, uint32_t page_height, void* detections);
uint32_t __swift_bridge__$DetectionResult$page_width(void* self);
uint32_t __swift_bridge__$DetectionResult$page_height(void* self);
void* __swift_bridge__$DetectionResult$detections(void* self);
void* __swift_bridge__$EmbeddedFile$new(void* name, void* data, void* mime_type);
void* __swift_bridge__$EmbeddedFile$name(void* self);
void* __swift_bridge__$EmbeddedFile$data(void* self);
void* __swift_bridge__$EmbeddedFile$mime_type(void* self);
void* __swift_bridge__$PdfImage$new(uintptr_t page_number, uintptr_t image_index, int64_t width, int64_t height, void* color_space, struct __private__OptionI64 bits_per_component, void* filters, void* data, void* decoded_format);
uintptr_t __swift_bridge__$PdfImage$page_number(void* self);
uintptr_t __swift_bridge__$PdfImage$image_index(void* self);
int64_t __swift_bridge__$PdfImage$width(void* self);
int64_t __swift_bridge__$PdfImage$height(void* self);
void* __swift_bridge__$PdfImage$color_space(void* self);
struct __private__OptionI64 __swift_bridge__$PdfImage$bits_per_component(void* self);
void* __swift_bridge__$PdfImage$filters(void* self);
void* __swift_bridge__$PdfImage$data(void* self);
void* __swift_bridge__$PdfImage$decoded_format(void* self);
void* __swift_bridge__$PageLayoutResult$new(uintptr_t page_index, void* regions, float page_width_pts, float page_height_pts, uint32_t render_width_px, uint32_t render_height_px);
uintptr_t __swift_bridge__$PageLayoutResult$page_index(void* self);
void* __swift_bridge__$PageLayoutResult$regions(void* self);
float __swift_bridge__$PageLayoutResult$page_width_pts(void* self);
float __swift_bridge__$PageLayoutResult$page_height_pts(void* self);
uint32_t __swift_bridge__$PageLayoutResult$render_width_px(void* self);
uint32_t __swift_bridge__$PageLayoutResult$render_height_px(void* self);
void* __swift_bridge__$PageTiming$new(double render_ms, double preprocess_ms, double onnx_ms, double inference_ms, double postprocess_ms, double mapping_ms);
double __swift_bridge__$PageTiming$render_ms(void* self);
double __swift_bridge__$PageTiming$preprocess_ms(void* self);
double __swift_bridge__$PageTiming$onnx_ms(void* self);
double __swift_bridge__$PageTiming$inference_ms(void* self);
double __swift_bridge__$PageTiming$postprocess_ms(void* self);
double __swift_bridge__$PageTiming$mapping_ms(void* self);
void* __swift_bridge__$CommonPdfMetadata$new(void* title, void* subject, void* authors, void* keywords, void* created_at, void* modified_at, void* created_by);
void* __swift_bridge__$CommonPdfMetadata$title(void* self);
void* __swift_bridge__$CommonPdfMetadata$subject(void* self);
void* __swift_bridge__$CommonPdfMetadata$authors(void* self);
void* __swift_bridge__$CommonPdfMetadata$keywords(void* self);
void* __swift_bridge__$CommonPdfMetadata$created_at(void* self);
void* __swift_bridge__$CommonPdfMetadata$modified_at(void* self);
void* __swift_bridge__$CommonPdfMetadata$created_by(void* self);
void* __swift_bridge__$blake3_hash_bytes(void* data);
struct __private__ResultPtrAndPtr __swift_bridge__$blake3_hash_file(void* path);
uint64_t __swift_bridge__$fast_hash(void* data);
bool __swift_bridge__$validate_cache_key(void* key);
void* __swift_bridge__$validate_port(uint16_t port);
void* __swift_bridge__$validate_host(void* host);
void* __swift_bridge__$validate_cors_origin(void* origin);
void* __swift_bridge__$validate_upload_size(uintptr_t size);
void* __swift_bridge__$validate_binarization_method(void* method);
void* __swift_bridge__$validate_token_reduction_level(void* level);
void* __swift_bridge__$validate_ocr_backend(void* backend);
void* __swift_bridge__$validate_language_code(void* code);
void* __swift_bridge__$validate_tesseract_psm(int32_t psm);
void* __swift_bridge__$validate_tesseract_oem(int32_t oem);
void* __swift_bridge__$validate_output_format(void* format);
void* __swift_bridge__$validate_confidence(double confidence);
void* __swift_bridge__$validate_dpi(int32_t dpi);
void* __swift_bridge__$validate_chunking_params(uintptr_t max_chars, uintptr_t max_overlap);
void* __swift_bridge__$validate_llm_config_model(void* model);
struct __private__ResultPtrAndPtr __swift_bridge__$extract_bytes(void* content, void* mime_type, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$extract_file(void* path, void* mime_type, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$extract_file_sync(void* path, void* mime_type, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$extract_bytes_sync(void* content, void* mime_type, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$batch_extract_file_sync(void* items, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$batch_extract_bytes_sync(void* items, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$batch_extract_file(void* items, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$batch_extract_bytes(void* items, void* config);
bool __swift_bridge__$is_valid_format_field(void* field);
struct __private__ResultPtrAndPtr __swift_bridge__$validate_mime_type(void* mime_type);
struct __private__ResultPtrAndPtr __swift_bridge__$detect_or_validate(void* path, void* mime_type);
struct __private__ResultPtrAndPtr __swift_bridge__$detect_mime_type_from_bytes(void* content);
struct __private__ResultPtrAndPtr __swift_bridge__$get_extensions_for_mime(void* mime_type);
void* __swift_bridge__$list_supported_formats(void);
void* __swift_bridge__$clear_processor_cache(void);
void* __swift_bridge__$transform_extraction_result_to_elements(void* result);
struct __private__ResultPtrAndPtr __swift_bridge__$extract_email_content(void* data, void* mime_type, struct __private__OptionU32 fallback_codepage);
void* __swift_bridge__$cells_to_text(void* cells);
void* __swift_bridge__$cells_to_markdown(void* cells);
struct __private__ResultPtrAndPtr __swift_bridge__$djot_to_html(void* djot_source);
void* __swift_bridge__$dedup_text(void* texts);
void* __swift_bridge__$normalize_whitespace(void* s);
void* __swift_bridge__$register_default_extractors(void);
void* __swift_bridge__$unregister_extractor(void* name);
struct __private__ResultPtrAndPtr __swift_bridge__$list_extractors(void);
void* __swift_bridge__$clear_extractors(void);
void* __swift_bridge__$unregister_ocr_backend(void* name);
struct __private__ResultPtrAndPtr __swift_bridge__$list_ocr_backends(void);
void* __swift_bridge__$clear_ocr_backends(void);
struct __private__ResultPtrAndPtr __swift_bridge__$list_post_processors(void);
void* __swift_bridge__$unregister_renderer(void* name);
struct __private__ResultPtrAndPtr __swift_bridge__$list_renderers(void);
void* __swift_bridge__$clear_renderers(void);
struct __private__ResultPtrAndPtr __swift_bridge__$list_validators(void);
void* __swift_bridge__$clear_validators(void);
void* __swift_bridge__$sanitize_filename(void* path);
void* __swift_bridge__$sanitize_path(void* path);
bool __swift_bridge__$is_valid_utf8(void* bytes);
void* __swift_bridge__$clean_extracted_text(void* text);
struct __private__ResultPtrAndPtr __swift_bridge__$reduce_tokens(void* text, void* config, void* language_hint);
struct __private__ResultPtrAndPtr __swift_bridge__$batch_reduce_tokens(void* texts, void* config, void* language_hint);
void* __swift_bridge__$bold(uint32_t start, uint32_t end);
void* __swift_bridge__$italic(uint32_t start, uint32_t end);
void* __swift_bridge__$underline(uint32_t start, uint32_t end);
void* __swift_bridge__$link(uint32_t start, uint32_t end, void* url, void* title);
void* __swift_bridge__$code(uint32_t start, uint32_t end);
void* __swift_bridge__$strikethrough(uint32_t start, uint32_t end);
void* __swift_bridge__$subscript_(uint32_t start, uint32_t end);
void* __swift_bridge__$superscript(uint32_t start, uint32_t end);
void* __swift_bridge__$font_size(uint32_t start, uint32_t end, void* value);
void* __swift_bridge__$color(uint32_t start, uint32_t end, void* value);
void* __swift_bridge__$highlight(uint32_t start, uint32_t end);
void* __swift_bridge__$classify_uri(void* url);
void* __swift_bridge__$safe_decode(void* byte_data, void* encoding);
double __swift_bridge__$calculate_text_confidence(void* text);
void* __swift_bridge__$create_string_buffer_pool(uintptr_t pool_size, uintptr_t buffer_capacity);
void* __swift_bridge__$create_byte_buffer_pool(uintptr_t pool_size, uintptr_t buffer_capacity);
void* __swift_bridge__$openapi_json(void);
void* __swift_bridge__$serve_default(void);
struct __private__ResultPtrAndPtr __swift_bridge__$chunk_text(void* text, void* config, void* page_boundaries);
struct __private__ResultPtrAndPtr __swift_bridge__$chunk_text_with_heading_source(void* text, void* config, void* page_boundaries, void* heading_source);
struct __private__ResultPtrAndPtr __swift_bridge__$chunk_texts_batch(void* texts, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$chunk_semantic(void* text, void* config, void* page_boundaries);
void* __swift_bridge__$normalize(void* v);
void* __swift_bridge__$list_presets(void);
void* __swift_bridge__$warm_model(void* model_type, void* cache_dir);
void* __swift_bridge__$download_model(void* model_type, void* cache_dir);
int32_t __swift_bridge__$calculate_optimal_dpi(double page_width, double page_height, int32_t target_dpi, int32_t max_dimension, int32_t min_dpi, int32_t max_dpi);
struct __private__ResultPtrAndPtr __swift_bridge__$detect_languages(void* text, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$extract_keywords(void* text, void* config);
void* __swift_bridge__$compute_hash(void* data);
struct __private__ResultPtrAndPtr __swift_bridge__$render_pdf_page_to_png(void* pdf_bytes, uintptr_t page_index, struct __private__OptionI32 dpi, void* password);
struct __private__ResultPtrAndPtr __swift_bridge__$extract_text_from_pdf(void* pdf_bytes);
struct __private__ResultPtrAndPtr __swift_bridge__$serialize_to_toon(void* result);
struct __private__ResultPtrAndPtr __swift_bridge__$serialize_to_json(void* result);
void* __swift_bridge__$alef_phantom_vec_ocr_backend(void);
struct __private__ResultPtrAndPtr __swift_bridge__$ocr_backend_call_process_image(void* this, void* image_bytes, void* config);
struct __private__ResultPtrAndPtr __swift_bridge__$ocr_backend_call_process_image_file(void* this, void* path, void* config);
bool __swift_bridge__$ocr_backend_call_supports_language(void* this, void* lang);
void* __swift_bridge__$ocr_backend_call_backend_type(void* this);
void* __swift_bridge__$ocr_backend_call_supported_languages(void* this);
bool __swift_bridge__$ocr_backend_call_supports_table_detection(void* this);
bool __swift_bridge__$ocr_backend_call_supports_document_processing(void* this);
struct __private__ResultPtrAndPtr __swift_bridge__$ocr_backend_call_process_document(void* this, void* path, void* config);
void* __swift_bridge__$alef_phantom_vec_post_processor(void);
void* __swift_bridge__$post_processor_call_process(void* this, void* result, void* config);
void* __swift_bridge__$post_processor_call_processing_stage(void* this);
bool __swift_bridge__$post_processor_call_should_process(void* this, void* result, void* config);
uint64_t __swift_bridge__$post_processor_call_estimated_duration_ms(void* this, void* result);
void* __swift_bridge__$alef_phantom_vec_validator(void);
void* __swift_bridge__$validator_call_validate(void* this, void* result, void* config);
bool __swift_bridge__$validator_call_should_validate(void* this, void* result, void* config);
int32_t __swift_bridge__$validator_call_priority(void* this);
void* __swift_bridge__$alef_phantom_vec_embedding_backend(void);
uintptr_t __swift_bridge__$embedding_backend_call_dimensions(void* this);
struct __private__ResultPtrAndPtr __swift_bridge__$embedding_backend_call_embed(void* this, void* texts);


