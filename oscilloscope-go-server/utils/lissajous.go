package utils

import (
	"context"
	"crypto/rand"
	"fmt"
	"image"
	"image/color"
	"image/draw"
	"image/png"
	"io"
	"log"
	"math"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"time"
)

const ANIMATION_DURATION = 5 * time.Second // Total duration of the animation
const SLEEP_DURATION = 1 * time.Millisecond // Duration to sleep between frames

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
func Lissajous(
	ctx context.Context,
	out io.Writer,
	cycles float64,
	res float64,
	size int,
	fps int,
	bgColor,
	fgColor color.Color,
	statusCh chan<- string,
) error {
	tmpDir, err := os.MkdirTemp("", "lissajous_frames")
	if err != nil {
		return fmt.Errorf("failed to create temp dir: %w", err)
	}
	defer os.RemoveAll(tmpDir)

	width := 2*size + 1
	height := 2*size + 1
	frames := fps * int(ANIMATION_DURATION.Seconds())

	for i := 0; i < frames; i++ {
		select {
		case <-ctx.Done():
			return ctx.Err() // Abort if context is cancelled
		default:
			// Continue if context is still active
		}

		randomBytes := make([]byte, 2)
		_, err = rand.Read(randomBytes)
		if err != nil {
			return fmt.Errorf("failed to read random bytes: %w", err)
		}

		// Generate random frequency and phase for each frame,
		// using the host platform's secure random number generator
		// (such as /dev/urandom on Unix-like systems).
		freq := float64(randomBytes[0]) / 255.0 * 3.0 // Scale to [0, 3]
		phase := float64(randomBytes[1]) / 255.0 * 2

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

		log.Printf("🖼️ Generated frame %d/%d", i+1, frames)

		if statusCh != nil && (i+1)%100 == 0 {
			select {
			case statusCh <- fmt.Sprintf("generated frame %d/%d", i+1, frames):
			case <-ctx.Done():
				return ctx.Err() // Abort if context is cancelled
			}
		}

		// Sleep between frames to simulate a long-running process
		time.Sleep(SLEEP_DURATION)
	}

	// Use ffmpeg to encode animated WebP
	if statusCh != nil {
		select {
		case statusCh <- "encoding animated WebP using ffmpeg":
		case <-ctx.Done():
			return ctx.Err() // Abort if context is cancelled
		}
	}

	cmd := exec.Command("ffmpeg",
		"-y", "-framerate", strconv.Itoa(fps),
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
		if statusCh != nil {
			select {
			case statusCh <- fmt.Sprintf("ffmpeg failed: %v", err):
			case <-ctx.Done():
				return ctx.Err() // Abort if context is cancelled
			}
		}

		// Return error with context
		return fmt.Errorf("ffmpeg failed: %w", err)
	}

	if statusCh != nil {
		select {
		case statusCh <- "done":
		case <-ctx.Done():
			return ctx.Err() // Abort if context is cancelled
		}
	}

	return nil
}

const LissajousMimeType = "image/webp"
