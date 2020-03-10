package dl

import (
	"path/filepath"

	"github.com/anacrolix/torrent"
)

// Magnet Download a torrent magnet
func Magnet(magnet, dataDir string) (Download, error) {
	if dataDir == "" {
		dataDir = "downloads"
	}

	ret := Download{}
	config := torrent.NewDefaultClientConfig()
	config.DataDir = dataDir
	c, err := torrent.NewClient(config)
	if err != nil {
		return ret, err
	}

	defer c.Close()
	t, err := c.AddMagnet(magnet)
	if err != nil {
		return ret, err
	}

	<-t.GotInfo()
	t.DownloadAll()
	c.WaitAll()

	ret = Download{
		Name:      t.Name(),
		Path:      dataDir,
		TotalSize: uint64(t.Length()),
		Files:     make([]File, len(t.Files())),
	}
	for i, torrentFile := range t.Files() {
		torrentFilePath := torrentFile.Path()
		filePath := filepath.Join(dataDir, torrentFilePath)
		ret.Files[i] = File{
			Path: filePath,
		}
	}

	return ret, nil
}
