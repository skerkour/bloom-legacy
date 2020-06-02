package bitflow

type DownloadQueuedMessage struct {
	ID  string `json:"id"`
	URL string `json:"url"`
}

type DownloadStartedMessage struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

type ProgressUpdatedMessage struct {
	ID       string `json:"id"`
	Progress int32  `json:"progress"`
}

type DownloadCompletedMessage struct {
	DownloadID string           `json:"download_id"`
	Files      []DownloadedFile `json:"files"`
}

type DownloadedFile struct {
	ID     string `json:"id"`
	Name   string `json:"name"`
	Path   string `json:"path"`
	Type   string `json:"type"`
	Size   uint64 `json:"size"`
	URL    string `json:"url"`
	MD5    string `json:"md5"`
	SHA1   string `json:"sha1"`
	SHA256 string `json:"sha256"`
	SHA512 string `json:"sha512"`
}
