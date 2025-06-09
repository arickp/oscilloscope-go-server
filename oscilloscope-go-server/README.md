# Oscilloscope Go Server by Eric Popelka

This is a lightweight Go server that serves animated GIF waveforms, with customizable foreground and background colors via query string parameters.
Based on the sample program in the first chapter of _The Go Programming Language_ by Donovan and Kernighan. 

## 🚀 Features

- Customize waveform color using `?fgColor=#RRGGBB` and `&bgColor=#RRGGBB`. (Remember to encode the`#` character by providing `%23` in its place. For a random color, provide a vlaue of `random` for either value.)
- Supports `random` for either color
- Configurable port via `PORT` environment variable. Default is 8000.
- Lightweight and fast — written in pure Go

---

## 📦 Requirements

- Go 1.24 or newer
- Internet browser or any HTTP client (like `curl`)
- Docker (optional)

---

## 🔧 Usage

### ▶️ Run directly

```bash
go run main.go
```

### 🏗️ Build the binary

```bash
go build -o oscilloscope-go-server main.go
```

Then run it:

```bash
./oscilloscope-go-server
```

---

## 🌐 Making a request

### Default URL:

```http
http://localhost:8000/?fgColor=%23ff0000&bgColor=%23000000
```

(The `%23` is URL-encoded `#`.)

### Use random colors:

```http
http://localhost:8000/?fgColor=random&bgColor=random
```

---

## ⚙️ Configuring the port

You can override the default port (`8000`) by setting the `PORT` environment variable:

```bash
PORT=9000 go run main.go
```

or if using the binary:

```bash
PORT=9000 ./oscilloscope-go-server
```

Then hit:

```http
http://localhost:9000/?fgColor=random&bgColor=random
```

---

## 📎 Example curl command

```bash
curl "http://localhost:8000/?fgColor=%23ff9900&bgColor=%23000000" --output waveform.gif
```

---

## 🧪 Dev tip

You can build a Docker image and run it using these commands:

```bash
docker build -t oscilloscope-go-server
docker run -p 8000:8000 oscilloscope-go-server
```
