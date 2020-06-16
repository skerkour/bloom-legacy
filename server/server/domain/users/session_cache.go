package users

import (
	"sync"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

var globalSessionsCache *sessionsCache

type sessionsCache struct {
	cache map[uuid.UUID]*Session
	mutex *sync.RWMutex
}

func (cache *sessionsCache) Set(session *Session) {
	cache.mutex.Lock()
	cache.cache[session.ID] = session
	cache.mutex.Unlock()
}

func (cache *sessionsCache) Get(sessionID uuid.UUID) *Session {
	cache.mutex.RLock()
	data, ok := cache.cache[sessionID]
	cache.mutex.RUnlock()
	if !ok {
		return nil
	}
	return data
}

func (cache *sessionsCache) Delete(sessionID uuid.UUID) {
	cache.mutex.Lock()
	delete(cache.cache, sessionID)
	cache.mutex.Unlock()
}

func newSessionsCache() *sessionsCache {
	return &sessionsCache{
		cache: map[uuid.UUID]*Session{},
		mutex: &sync.RWMutex{},
	}
}

// InitGlobalSessionsCache inits the sessions cache used to reduce the number of DB requests for each
// HTTP requests
func InitGlobalSessionsCache(logger rz.Logger) error {
	var err error
	allSessions := []Session{}

	globalSessionsCache = newSessionsCache()

	queryFind := "SELECT * FROM sessions"
	err = db.DB.Select(&allSessions, queryFind)
	if err != nil {
		return err
	}
	for _, session := range allSessions {
		globalSessionsCache.Set(&session)
	}
	return nil
}
