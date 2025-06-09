# ===== Stage 1: Build the binary =====
FROM golang:1.24-alpine AS builder

# Install git (required for go get sometimes)
RUN apk add --no-cache git

# Set working directory inside container
WORKDIR /app

# Copy go module files first and download deps
COPY go.mod go.sum ./
RUN go mod download

# Copy the rest of your code
COPY . .

# Build the binary
RUN go build -o oscilloscope-go-server

# ===== Stage 2: Create minimal final image =====
FROM alpine:latest

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/oscilloscope-go-server .

# Copy optional .env file (you can override it at runtime)
COPY .env .

# Expose the port (you can override it)
EXPOSE 8000

# Run it!
CMD ["./oscilloscope-go-server"]

