package objects

type compressionAlgorithm int

const (
	compressSnappy compressionAlgorithm = iota
	compressGzip
)
