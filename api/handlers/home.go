package handlers

import (
	"encoding/json"
	"net/http"
)

func HomeHandler(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode("hello")
}
