package datastore

import (
	"errors"
	"myapp/api/models"

	"github.com/mediocregopher/radix.v2/pool"
)

// RedisDatastore contains pool of threads to redis connection
type RedisDatastore struct {
	*pool.Pool
}

// NewRedisDatastore initializes new thread pool connected to Redis
func NewRedisDatastore(address string) (*RedisDatastore, error) {

	connectionPool, err := pool.New(redisConnectionProtocol, address, 10)
	if err != nil {
		return nil, err
	}
	return &RedisDatastore{
		Pool: connectionPool,
	}, nil
}

// SaveStroke saves stroke to redis
func (r *RedisDatastore) SaveStroke(stroke *models.Stroke) error {
	if r.Cmd(set, "filler", stroke).Err != nil {
		return errors.New("Failed to execute Redis SET command")
	}
	return nil
}

// BeginTransaction initiates the queue of txs to execute
func (r *RedisDatastore) BeginTransaction() error {
	if r.Cmd(multi).Err != nil {
		return errors.New("Failed to begin transaction")
	}
	return nil
}

// ExecTransaction executes all txs in the queue
func (r *RedisDatastore) ExecTransaction() error {
	if r.Cmd(exec).Err != nil {
		return errors.New("Failed to execute transaction")
	}
	return nil
}

// Close closes the redis connection
func (r *RedisDatastore) Close() {
	r.Empty()
}

const redisConnectionProtocol = "tcp"
const set = "SET"
const multi = "MULTI"
const exec = "EXEC"
