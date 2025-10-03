package utils

import "net/http"

func GetRemoteIP(r *http.Request) string {
	remoteIP := r.RemoteAddr
	if ip := r.Header.Get("X-Forwarded-For"); ip != "" {
		remoteIP = ip
	}
	return remoteIP
}
