package api

import (
	"net/http"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/config"
)

// IndexHandler simply redirect to `config.WebsiteUrl`
func IndexHandler(w http.ResponseWriter, r *http.Request) {
	http.Redirect(w, r, config.WebsiteUrl, 301)
}
