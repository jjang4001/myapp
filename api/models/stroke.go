package models

import (
	"encoding/json"
)

type Color struct {
	R     int32 `json:"r"`
	G     int32 `json:"g"`
	B     int32 `json:"b"`
	Alpha int32 `json:"alpha"`
}

type Point struct {
	X int64 `json:"x"`
	Y int64 `json:"y"`
}

type Stroke struct {
	RGB   string  `json:"rgb"`
	Alpha float64 `json:"alpha"`
	X     float64 `json:"x"`
	Y     float64 `json:"y"`
	Size  float64 `json:"size"`
}

func NewStroke(stroke string) *Stroke {
	data := &Stroke{}
	json.Unmarshal([]byte(stroke), data)
	return data
}
