// HTTP server serving up pretty GIF animations
package main

import (
	"bytes"
	"image/color"
	"log"
	"net/http"
	"os"
	"os/exec"
	"oscilloscope-go-server/utils"
	"strconv"
	"time"

	"github.com/joho/godotenv"
)

func getRemoteIP(r *http.Request) string {
	remoteIP := r.RemoteAddr
	if ip := r.Header.Get("X-Forwarded-For"); ip != "" {
		remoteIP = ip
	}
	return remoteIP
}

func main() {
	const DefaultPort = 8000

	var (
		DefaultFgColor = color.RGBA{255, 255, 255, 255}
		DefaultBgColor = color.RGBA{0, 0, 0, 255}
	)

	if dotenvErr := godotenv.Load(); dotenvErr != nil {
		log.Println("No .env file found, using default settings")
	}

	port := os.Getenv("PORT")
	if port == "" {
		port = strconv.Itoa(DefaultPort)
	}

	var ffmpegAvailable bool
	if _, ffmpegErr := exec.LookPath("ffmpeg"); ffmpegErr != nil {
		log.Fatal("❌ ffmpeg not found")
		ffmpegAvailable = false
	} else {
		log.Println("✅ ffmpeg found in PATH")
		ffmpegAvailable = true
	}

	http.HandleFunc("/lissajous", func(writer http.ResponseWriter, request *http.Request) {
		start := time.Now()
		remoteIP := getRemoteIP(request)
		log.Printf("🌍 Request to create waveform from IP: %s, User-Agent: %s", remoteIP, request.UserAgent())

		if !ffmpegAvailable {
			log.Println("ffmpeg not installed, cannot process request")
			http.Error(writer, "Server error", http.StatusInternalServerError)
			return
		}

		var err error
		var fgColor color.Color = DefaultFgColor
		var bgColor color.Color = DefaultBgColor
		cycles := 5.0
		res := 0.001
		size := 100
		bgColorHexString := request.URL.Query().Get("bgColor")
		fgColorHexString := request.URL.Query().Get("fgColor")

		if bgColorHexString != "" {
			bgColor, err = utils.HexStringToColor(bgColorHexString)
			if err != nil {
				http.Error(writer, "Invalid bgColor: "+err.Error(), http.StatusBadRequest)
				return
			}
		}

		if fgColorHexString != "" {
			fgColor, err = utils.HexStringToColor(fgColorHexString)
			if err != nil {
				http.Error(writer, "Invalid bgColor: "+err.Error(), http.StatusBadRequest)
				return
			}
		}

		frames := 60 // default
		if framesStr := request.URL.Query().Get("frames"); framesStr != "" {
			if parsed, err := strconv.Atoi(framesStr); err == nil && parsed > 0 && parsed <= 200 {
				frames = parsed
			}
		}

		var buf bytes.Buffer
		if lissajousErr := utils.Lissajous(&buf, cycles, res, size, frames, bgColor, fgColor); lissajousErr != nil {
			log.Printf("❌ Failed to generate waveform: %v", lissajousErr)
			http.Error(writer, "Error generating animation", http.StatusInternalServerError)
			return
		}

		writer.Header().Set("Content-Type", utils.LissajousMimeType)
		writer.Header().Set("Content-Length", strconv.Itoa(buf.Len()))
		if _, writeErr := writer.Write(buf.Bytes()); writeErr != nil {
			log.Printf("❌ Failed to send response")
			return
		}

		elapsed := time.Since(start)
		log.Printf("✅ Waveform created in %dms", elapsed.Milliseconds())
	})

	// Serve static files like JS, CSS, and demo.html
	http.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))

	// Root path serves demo.html
	http.HandleFunc("/", func(writer http.ResponseWriter, request *http.Request) {
		remoteIP := getRemoteIP(request)
		log.Printf("🌍 Request to view demo from IP: %s, User-Agent: %s", remoteIP, request.UserAgent())
		http.ServeFile(writer, request, "static/demo.html")
	})

	var address = "0.0.0.0:" + port
	log.Println("📡 Listening on " + address)
	log.Fatal(http.ListenAndServe(address, nil))
}
