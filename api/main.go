package main

import (
	"myapp/api/handlers"
	"myapp/api/hub"
	"myapp/api/middleware"
	"net/http"
	"os"

	ghandlers "github.com/gorilla/handlers"
	"github.com/gorilla/mux"
)

const (
	WEBSERVERPORT = ":5000"
)

func main() {
	r := mux.NewRouter()
	hubServer := hub.NewHub()
	go hubServer.Run()

	// handlers

	r.HandleFunc("/api/healthcheck", handlers.HomeHandler)

	r.HandleFunc("/api/doodle", func(w http.ResponseWriter, r *http.Request) {
		hub.ServeWs(hubServer, w, r)
	})

	// r.HandleFunc("/api/doodle", handlers.DoodleHandler)

	// middleware

	http.Handle("/", middleware.ContextExampleHandler(middleware.PanicRecoveryHandler(ghandlers.LoggingHandler(os.Stdout, r))))

	http.ListenAndServe(WEBSERVERPORT, nil)
}
