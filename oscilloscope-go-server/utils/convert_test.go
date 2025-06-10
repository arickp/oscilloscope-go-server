package utils

import (
	"image/color"
	"testing"
)

func TestHexStringToColor_Valid(t *testing.T) {
	tests := []struct {
		input    string
		expected color.RGBA
	}{
		{"#ff0000", color.RGBA{255, 0, 0, 255}},
		{"#00ff00ff", color.RGBA{0, 255, 0, 255}},
		{"#0000ff80", color.RGBA{0, 0, 255, 128}},
	}

	for _, tt := range tests {
		col, err := HexStringToColor(tt.input)
		if err != nil {
			t.Errorf("HexStringToColor(%q) unexpected error: %v", tt.input, err)
			continue
		}
		got := col.(color.RGBA)
		if got != tt.expected {
			t.Errorf("HexStringToColor(%q) = %v; want %v", tt.input, got, tt.expected)
		}
	}
}

func TestHexStringToColor_Random(t *testing.T) {
	col, err := HexStringToColor("random")
	if err != nil {
		t.Fatalf("expected no error for 'random', got %v", err)
	}
	_, ok := col.(color.RGBA)
	if !ok {
		t.Fatalf("expected color.RGBA for 'random', got %T", col)
	}
}

func TestHexStringToColor_Invalid(t *testing.T) {
	badInputs := []string{"#123", "#zzzzzz", "#1234567", "#123456789", "nothex", ""}
	for _, input := range badInputs {
		_, err := HexStringToColor(input)
		if err == nil {
			t.Errorf("HexStringToColor(%q) should fail but didn't", input)
		}
	}
}

func TestParseUint8FromHexString(t *testing.T) {
	tests := map[string]uint8{
		"00": 0,
		"7f": 127,
		"FF": 255,
		"ff": 255,
	}
	for input, want := range tests {
		got, err := ParseUint8FromHexString(input)
		if err != nil {
			t.Errorf("ParseUint8FromHexString(%q) unexpected error: %v", input, err)
		}
		if got != want {
			t.Errorf("ParseUint8FromHexString(%q) = %d; want %d", input, got, want)
		}
	}
}

func TestParseUint8FromHexString_Invalid(t *testing.T) {
	invalid := []string{"", "g1", "123", "xyz", "1000"}
	for _, input := range invalid {
		_, err := ParseUint8FromHexString(input)
		if err == nil {
			t.Errorf("ParseUint8FromHexString(%q) should fail but didn't", input)
		}
	}
}
