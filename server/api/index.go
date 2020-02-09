package api

import (
	"net/http"

	"gitlab.com/bloom42/bloom/server/config"
)

func IndexHandler(w http.ResponseWriter, r *http.Request) {
	http.Redirect(w, r, config.WebsiteUrl, 301)
}
