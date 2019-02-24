package models

import (
	"time"
)

type Message struct {
	Msg       string `json:"msg"`
	Timestamp int64  `json:"timestamp"`
}

func NewMessage(msgText string) *Message {
	timestamp := time.Now().Unix()
	msg := Message{Msg: msgText, Timestamp: timestamp}
	return &msg
}
