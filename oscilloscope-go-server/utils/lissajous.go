package utils

import (
	"fmt"
	"image"
	"image/color"
	"image/draw"
	"image/png"
	"io"
	"math"
	"math/rand"
	"os"
	"os/exec"
	"path/filepath"
)

// Lissajous generates a Lissajous figure as an animated WebP
//
// The function plots the parametric equation:
//
//	x(t) = sin(t)
//	y(t) = sin(t * freq + phase)
//
// over the range [0, cycles * 2π], at step size `res`. The figure is rendered
// at the center of a square canvas of size (2*size + 1), using foreground and
// background colors as defined by the caller.
func Lissajous(out io.Writer, cycles float64, res float64, size int, frames int, bgColor, fgColor color.Color) error {
	tmpDir, err := os.MkdirTemp("", "lissajous_frames")
	if err != nil {
		return fmt.Errorf("failed to create temp dir: %w", err)
	}
	defer os.RemoveAll(tmpDir)

	width := 2*size + 1
	height := 2*size + 1
	freq := rand.Float64()*3.0 + 1.0
	phase := 0.0

	for i := 0; i < frames; i++ {
		img := image.NewRGBA(image.Rect(0, 0, width, height))
		draw.Draw(img, img.Bounds(), &image.Uniform{bgColor}, image.Point{}, draw.Src)

		for t := 0.0; t < cycles*2*math.Pi; t += res {
			x := math.Sin(t)
			y := math.Sin(t*freq + phase)
			px := size + int(x*float64(size)+0.5)
			py := size + int(y*float64(size)+0.5)
			img.Set(px, py, fgColor)
		}

		framePath := filepath.Join(tmpDir, fmt.Sprintf("frame_%03d.png", i))
		f, err := os.Create(framePath)
		if err != nil {
			return fmt.Errorf("failed to create frame: %w", err)
		}
		if err := png.Encode(f, img); err != nil {
			f.Close()
			return fmt.Errorf("failed to encode frame: %w", err)
		}
		f.Close()
		phase += 0.1
	}

	// Use ffmpeg to encode animated WebP
	cmd := exec.Command("ffmpeg",
		"-y", "-framerate", "25",
		"-i", filepath.Join(tmpDir, "frame_%03d.png"),
		"-loop", "0",
		"-lossless", "1",
		"-plays", "0", // ✅ ensure infinite loop
		"-c:v", "libwebp_anim", // ✅ force animated WebP
		"-an", // ✅ no audio
		"-f", "webp",
		"pipe:1",
	)

	if os.Getenv("DEBUG") == "true" {
		cmd.Stderr = os.Stderr
	} else {
		cmd.Stderr = nil
	}

	cmd.Stdout = out

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("ffmpeg failed: %w", err)
	}

	return nil
}

const LissajousMimeType = "image/webp"
