package inmemory

import (
	"sync"
)

// Cache is an implementation of an unbounded, in-memory cache
type Cache struct {
	cache map[string]interface{}
	mutex *sync.RWMutex
}

// NewCache returns a new implementation of an unbounded, in-memory cache
func NewCache() *Cache {
	return &Cache{
		cache: map[string]interface{}{},
		mutex: &sync.RWMutex{},
	}
}

// Set a value in the cache at the given key
func (cache *Cache) Set(key string, value interface{}) {
	cache.mutex.Lock()
	cache.cache[key] = value
	cache.mutex.Unlock()
}

// Get returns a value corresponding to the key if found
func (cache *Cache) Get(key string) (value interface{}, found bool) {
	cache.mutex.RLock()
	value, found = cache.cache[key]
	cache.mutex.RUnlock()

	return
}

// Delete remove a specific entry from the cache
func (cache *Cache) Delete(key string) {
	cache.mutex.Lock()
	delete(cache.cache, key)
	cache.mutex.Unlock()
}

// Clear remove all entries from the cache
func (cache *Cache) Clear() {
	cache.mutex.Lock()
	cache.cache = map[string]interface{}{}
	cache.mutex.Unlock()
}
