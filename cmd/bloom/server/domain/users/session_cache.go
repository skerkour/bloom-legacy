package users

import (
	"sync"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

var globalSessionsCache *SessionsCache

type SessionsCache struct {
	cache map[uuid.UUID]*Session
	mutex *sync.RWMutex
}

func (cache *SessionsCache) Set(session *Session) {
	cache.mutex.Lock()
	cache.cache[session.ID] = session
	cache.mutex.Unlock()
}

func (cache *SessionsCache) Get(sessionId uuid.UUID) *Session {
	cache.mutex.RLock()
	data, ok := cache.cache[sessionId]
	cache.mutex.RUnlock()
	if !ok {
		return nil
	}
	return data
}

func (cache *SessionsCache) Delete(sessionId uuid.UUID) {
	cache.mutex.Lock()
	delete(cache.cache, sessionId)
	cache.mutex.Unlock()
}

func newSessionsCache() *SessionsCache {
	return &SessionsCache{
		cache: map[uuid.UUID]*Session{},
		mutex: &sync.RWMutex{},
	}
}

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
