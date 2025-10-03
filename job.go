package main

import (
	"sync"

	"github.com/google/uuid"
)

type JobStatus struct {
	Status string `json:"status"`
	Error  string `json:"error,omitempty"`
}

type Job struct {
	Status   JobStatus
	Result   []byte
	StatusCh chan string
	Done     chan struct{}
}

var (
	jobs   = make(map[string]*Job)
	jobsMu sync.Mutex
)

func AddJob(id string, job *Job) {
	jobsMu.Lock()
	defer jobsMu.Unlock()
	jobs[id] = job
}

func GetJob(id string) (*Job, bool) {
	jobsMu.Lock()
	defer jobsMu.Unlock()
	job, ok := jobs[id]
	return job, ok
}

func RemoveJob(id string) {
	jobsMu.Lock()
	defer jobsMu.Unlock()
	delete(jobs, id)
}

func GenerateJobID() string {
	// Generate a unique job ID
	return uuid.New().String()
}
