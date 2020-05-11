package objects

type compressionAlgorithm int

const (
	compressAlgoSnappy compressionAlgorithm = iota
	compressAlgoGzip
)
