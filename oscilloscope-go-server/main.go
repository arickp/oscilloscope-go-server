// HTTP server serving up pretty GIF animations
package main

import (
	"fmt"
	"image"
	"image/color"
	"image/gif"
	"io"
	"log"
	"math"
	"math/rand"
	"net/http"
	"os"
	"strconv"
	"strings"

	"github.com/joho/godotenv"
)

func randomColor() color.Color {
	return color.RGBA{
		R: uint8(rand.Intn(256)),
		G: uint8(rand.Intn(256)),
		B: uint8(rand.Intn(256)),
		A: 0xff,
	}
}

func hexStringToColor(hexString string) (color.Color, error) {
	hexString = strings.TrimPrefix(hexString, "#")
	if len(hexString) != 6 {
		return nil, fmt.Errorf("invalid length for hex color: %q", hexString)
	}
	if strings.ToLower(hexString) == "random" {
		return randomColor(), nil
	}
	rVal, err := strconv.ParseUint(hexString[0:2], 16, 8)
	if err != nil {
		return nil, fmt.Errorf("invalid red value: %w", err)
	}
	gVal, err := strconv.ParseUint(hexString[2:4], 16, 8)
	if err != nil {
		return nil, fmt.Errorf("invalid green value: %w", err)
	}
	bVal, err := strconv.ParseUint(hexString[4:6], 16, 8)
	if err != nil {
		return nil, fmt.Errorf("invalid blue value: %w", err)
	}

	col := color.RGBA{uint8(rVal), uint8(gVal), uint8(bVal), 255}
	return col, nil
}

func main() {
	const DefaultPort = 8000

	var (
		DefaultFgColor = color.RGBA{255, 255, 255, 255}
		DefaultBgColor = color.RGBA{0, 0, 0, 255}
	)

	err := godotenv.Load()
	if err != nil {
		log.Println("No .env file found, using default settings")
	}

	port := os.Getenv("PORT")
	if port == "" {
		port = strconv.Itoa(DefaultPort)
	}

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		var err error
		var fgColor color.Color = DefaultFgColor
		var bgColor color.Color = DefaultBgColor
		cycles := 5.0
		res := 0.001
		size := 100
		nframes := 64
		delay := 8
		bgColorHexString := r.URL.Query().Get("bgColor")
		fgColorHexString := r.URL.Query().Get("fgColor")

		if bgColorHexString != "" {
			bgColor, err = hexStringToColor(bgColorHexString)
			if err != nil {
				http.Error(w, "Invalid bgColor: "+err.Error(), http.StatusBadRequest)
			}
		}

		if fgColorHexString != "" {
			fgColor, err = hexStringToColor(fgColorHexString)
			if err != nil {
				http.Error(w, "Invalid bgColor: "+err.Error(), http.StatusBadRequest)
			}
		}

		lissajous(w, cycles, res, size, nframes, delay, bgColor, fgColor)
	})
	var address = "0.0.0.0:" + port
	log.Println("Listening on " + address)
	log.Fatal(http.ListenAndServe(address, nil))
}

func lissajous(out io.Writer, cycles float64, res float64, size int, nframes int, delay int, bgColor color.Color, fgColor color.Color) {
	freq := rand.Float64() * 3.0 // relative frequency of y oscillator
	anim := gif.GIF{LoopCount: nframes}
	phase := 0.0 // phase difference
	palette := []color.Color{bgColor, fgColor}

	const (
		bgColorIndex = 0
		fgColorIndex = 1
	)

	for i := 0; i < nframes; i++ {
		rect := image.Rect(0, 0, 2*size+1, 2*size+1)
		img := image.NewPaletted(rect, palette)

		for t := 0.0; t < cycles*2*math.Pi; t += res {
			x := math.Sin(t)
			y := math.Sin(t*freq + phase)
			img.SetColorIndex(size+int(x*float64(size)+0.5), size+int(y*float64(size)+0.5), fgColorIndex)
		}

		phase += 0.1
		anim.Delay = append(anim.Delay, delay)
		anim.Image = append(anim.Image, img)
	}

	gif.EncodeAll(out, &anim)
}
