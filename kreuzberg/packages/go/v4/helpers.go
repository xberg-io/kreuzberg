package kreuzberg

// BoolPtr returns a pointer to a bool value.
// Useful for setting optional boolean fields in configuration structs.
func BoolPtr(v bool) *bool {
	return &v
}

// StringPtr returns a pointer to a string value.
// Useful for setting optional string fields in configuration structs.
func StringPtr(v string) *string {
	return &v
}

// IntPtr returns a pointer to an int value.
// Useful for setting optional int fields in configuration structs.
func IntPtr(v int) *int {
	return &v
}

// Int32Ptr returns a pointer to an int32 value.
// Useful for setting optional int32 fields in configuration structs.
func Int32Ptr(v int32) *int32 {
	return &v
}

// Int64Ptr returns a pointer to an int64 value.
// Useful for setting optional int64 fields in configuration structs.
func Int64Ptr(v int64) *int64 {
	return &v
}

// FloatPtr returns a pointer to a float64 value (alias for Float64Ptr).
// Useful for setting optional float fields in configuration structs.
func FloatPtr(v float64) *float64 {
	return &v
}

// Float32Ptr returns a pointer to a float32 value.
// Useful for setting optional float32 fields in configuration structs.
func Float32Ptr(v float32) *float32 {
	return &v
}

// Float64Ptr returns a pointer to a float64 value.
// Useful for setting optional float64 fields in configuration structs.
func Float64Ptr(v float64) *float64 {
	return &v
}

// Uint32Ptr returns a pointer to a uint32 value.
// Useful for setting optional uint32 fields in configuration structs.
func Uint32Ptr(v uint32) *uint32 {
	return &v
}

// Uint64Ptr returns a pointer to a uint64 value.
// Useful for setting optional uint64 fields in configuration structs.
func Uint64Ptr(v uint64) *uint64 {
	return &v
}

// TextDirectionPtr returns a pointer to a TextDirection value.
// Useful for setting optional TextDirection fields in metadata structs.
func TextDirectionPtr(v TextDirection) *TextDirection {
	return &v
}

// LinkTypePtr returns a pointer to a LinkType value.
// Useful for setting optional LinkType fields in metadata structs.
func LinkTypePtr(v LinkType) *LinkType {
	return &v
}

// ImageTypePtr returns a pointer to an ImageType value.
// Useful for setting optional ImageType fields in metadata structs.
func ImageTypePtr(v ImageType) *ImageType {
	return &v
}

// StructuredDataTypePtr returns a pointer to a StructuredDataType value.
// Useful for setting optional StructuredDataType fields in metadata structs.
func StructuredDataTypePtr(v StructuredDataType) *StructuredDataType {
	return &v
}
