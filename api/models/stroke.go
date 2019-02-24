package models

type Stroke struct {
	Color Color `json:"color"`
	Point Point `json:"point"`
}

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
