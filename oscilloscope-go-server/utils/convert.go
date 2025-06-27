package utils

import (
	"fmt"
	"image/color"
	"math/rand"
	"strconv"
	"strings"
)

// Generates a random color with max alpha
func randomColor() color.Color {
	return color.RGBA{
		R: uint8(rand.Intn(256)),
		G: uint8(rand.Intn(256)),
		B: uint8(rand.Intn(256)),
		A: 255,
	}
}

// Converts a string like "#ff0000ff" to a `color.Color“ struct value
// If hexString == "random", returns a random color.
func HexStringToColor(hexString string) (color.Color, error) {
	if strings.ToLower(hexString) == "random" {
		return randomColor(), nil
	}

	hexString = strings.TrimPrefix(hexString, "#")
	if len(hexString) != 6 && len(hexString) != 8 {
		return nil, fmt.Errorf("invalid length for hex color: %q", hexString)
	}
	rVal, err := ParseUint8FromHexString(hexString[0:2])
	if err != nil {
		return nil, fmt.Errorf("invalid red value: %w", err)
	}
	gVal, err := ParseUint8FromHexString(hexString[2:4])
	if err != nil {
		return nil, fmt.Errorf("invalid green value: %w", err)
	}
	bVal, err := ParseUint8FromHexString(hexString[4:6])
	if err != nil {
		return nil, fmt.Errorf("invalid blue value: %w", err)
	}

	aVal := uint8(255)
	if len(hexString) == 8 {
		aVal, err = ParseUint8FromHexString(hexString[6:8])
		if err != nil {
			return nil, fmt.Errorf("invalid alpha value: %w", err)
		}
	}

	col := color.RGBA{rVal, gVal, bVal, aVal}
	return col, nil
}

// ParseUint8 parses a hex string (like "ff") into a uint8.
// Returns an error if the string is not a valid number
// or if it exceeds the uint8 range (0–255).
func ParseUint8FromHexString(s string) (uint8, error) {
	val, err := strconv.ParseUint(s, 16, 8)
	if err != nil {
		return 0, fmt.Errorf("ParseUint8: cannot parse %q: %w", s, err)
	}
	return uint8(val), nil
}
