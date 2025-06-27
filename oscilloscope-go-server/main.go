// HTTP server serving up pretty GIF animations
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"image/color"
	"log"
	"net/http"
	"os"
	"os/exec"
	"oscilloscope-go-server/utils"
	"strconv"

	"github.com/joho/godotenv"
)

const DefaultPort = 8000

const StatusChannelBufferSize = 20

func main() {
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

	mux := http.NewServeMux()

	mux.HandleFunc("/lissajous", func(writer http.ResponseWriter, request *http.Request) {
		log.Printf("🌍 Request to create waveform from IP: %s, User-Agent: %s", utils.GetRemoteIP(request), request.UserAgent())

		if request.Method != http.MethodPost {
			http.Error(writer, "POST only", http.StatusMethodNotAllowed)
			return
		}

		if request.Header.Get("Content-Type") != "application/x-www-form-urlencoded" {
			http.Error(writer, "Content-Type must be application/x-www-form-urlencoded", http.StatusUnsupportedMediaType)
			return
		}

		if err := request.ParseForm(); err != nil {
			http.Error(writer, "Failed to parse form: "+err.Error(), http.StatusBadRequest)
			return
		}

		jobID := GenerateJobID()
		job := &Job{
			Status:   JobStatus{Status: "pending"},
			StatusCh: make(chan string, StatusChannelBufferSize),
			Done:     make(chan struct{}),
		}
		AddJob(jobID, job)

		if !ffmpegAvailable {
			log.Println("❌ ffmpeg is not available, cannot process request")
			select {
			case job.StatusCh <- "ffmpeg is not available, cannot process request":
			default:
			}
			close(job.Done)
			http.Error(writer, "Server error: ffmpeg is not available", http.StatusInternalServerError)
			return
		}

		go func() {
			defer close(job.Done)
			var buf bytes.Buffer

			var err error
			var fgColor color.Color = DefaultFgColor
			var bgColor color.Color = DefaultBgColor
			cycles := 5.0
			res := 0.001
			size := 100
			bgColorHexString := request.FormValue("bgColor")
			fgColorHexString := request.FormValue("fgColor")

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

			fps := 60 // default
			if framesStr := request.FormValue("frames"); framesStr != "" {
				if parsed, err := strconv.Atoi(framesStr); err == nil && parsed > 0 && parsed <= 200 {
					fps = parsed
				}
			}

			if lissajousErr := utils.Lissajous(
				context.Background(),
				&buf,
				cycles,
				res,
				size,
				fps,
				bgColor,
				fgColor,
				job.StatusCh,
			); lissajousErr != nil {
				log.Printf("❌ Failed to generate waveform: %v", lissajousErr)
				http.Error(writer, "Error generating animation", http.StatusInternalServerError)
				return
			}

			job.Result = buf.Bytes()
			job.Status = JobStatus{Status: "complete"}
		}()

		writer.Header().Set("Content-Type", "application/json")
		json.NewEncoder(writer).Encode(map[string]string{
			"jobID":  jobID,
			"status": "Job started, check status with /lissajous/status/" + jobID})
	})

	mux.HandleFunc("/lissajous/status/", func(writer http.ResponseWriter, request *http.Request) {
		jobID := request.URL.Path[len("/lissajous/status/"):]
		log.Printf("🌍 Incoming job status request for job %s. Request IP: %s; User-Agent: %s", jobID, utils.GetRemoteIP(request), request.UserAgent())

		job, exists := GetJob(jobID)
		if !exists {
			log.Println("❌ Job not found:", jobID)
			http.Error(writer, "Job not found", http.StatusNotFound)
			return
		}

		select {
		case status := <-job.StatusCh:
			job.Status.Status = status
		default:
			// No new status, keep the current one
		}

		log.Println("🔄 Job status:", job.Status.Status)
		writer.Header().Set("Content-Type", "application/json")
		json.NewEncoder(writer).Encode(job.Status)
	})

	mux.HandleFunc("/lissajous/result", func(writer http.ResponseWriter, request *http.Request) {
		jobID := request.URL.Query().Get("id")
		log.Printf("🌍 Request for job result %s from IP: %s, User Agent: %s", jobID, utils.GetRemoteIP(request), request.UserAgent())

		if jobID == "" {
			http.Error(writer, "Missing job ID", http.StatusBadRequest)
			return
		}

		job, exists := GetJob(jobID)
		if !exists {
			http.Error(writer, "Job not found", http.StatusNotFound)
			return
		}

		if job.Status.Status != "done" {
			http.Error(writer, "Job is not complete yet", http.StatusBadRequest)
			return
		}
		writer.Header().Set("Content-Disposition", "attachment; filename=waveform.webp")
		writer.Header().Set("Content-Type", utils.LissajousMimeType)
		log.Printf("📦 Sending result for job %s", jobID)
		if _, err := writer.Write(job.Result); err != nil {
			log.Printf("❌ Error sending result for job %s: %v", jobID, err)
			http.Error(writer, "Error sending result", http.StatusInternalServerError)
			return
		}
		log.Printf("✅ Job %s result sent successfully", jobID)
		RemoveJob(jobID) // Clean up after sending the result
	})

	mux.Handle("/static/", http.StripPrefix("/static/", http.FileServer(http.Dir("static"))))

	mux.HandleFunc("/", func(writer http.ResponseWriter, request *http.Request) {
		if request.URL.Path != "/" {
			log.Printf("🚫 404 Not Found for path: %s", request.URL.Path)
			http.NotFound(writer, request)
			return
		}

		log.Printf("🌍 Request to view demo from IP: %s, User-Agent: %s", utils.GetRemoteIP(request), request.UserAgent())

		if _, err := os.Stat("static/demo.html"); os.IsNotExist(err) {
			log.Printf("❌ static/demo.html not found")
			http.Error(writer, "demo.html not found", http.StatusInternalServerError)
			return
		} else {
			http.ServeFile(writer, request, "static/demo.html")
			log.Println("✅ Demo page served successfully")
		}
	})

	var address = "0.0.0.0:" + port
	log.Println("📡 Listening on " + address)
	log.Fatal(http.ListenAndServe(address, mux))
}
