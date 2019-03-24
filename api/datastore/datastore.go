package datastore

import (
	"errors"
	"myapp/api/models"
)

// Datastore interfaces database connections for persisting strokes
type Datastore interface {
	SaveStroke(stroke *models.Stroke) error
	BeginTransaction() error
	ExecTransaction() error
	Close()
}

// enumerates types of database connections
const (
	REDIS = iota
)

// NewDatastore returns a datastore connection
func NewDatastore(datastoreType int, dbConnectionString string) (Datastore, error) {
	switch datastoreType {
	case REDIS:
		return NewRedisDatastore(dbConnectionString)
	default:
		return nil, errors.New("The datastore you specified does not exist")
	}
}
