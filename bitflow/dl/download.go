package dl

// Download represent a Download
type Download struct {
	Name      string `json:"name"` // the filename for a signle file download or the torrent name for a torrent
	TotalSize uint64 `json:"total_size"`
	Path      string `json:"path"` // The download's path
	Files     []File `json:"files"`
}

// File represent a file in a downlaod instance
type File struct {
	Path string `json:"path"` // file's path within the download's directory
}
