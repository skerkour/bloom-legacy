package objects

type compressionAlgorithm int

const (
	compressAlgoSnappy compressionAlgorithm = iota
	compressAlgoGzip

	DEFAULT_ALGORITHM_STRING = "snappy+xchacha20-poly1305"
	MAX_OBJECT_SIZE          = 100000 // 100kb
)
