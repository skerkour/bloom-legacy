package driver

// Cache is the interface defining a cache service
type Cache interface {
	Clear()
	Get(key string) (value interface{}, found bool)
	Set(key string, value interface{})
	Delete(key string)
}
