package repository

import "gitlab.com/bloom42/gobox/uuid"

func getUserCacheKey(userID uuid.UUID) string {
	return "users/" + userID.String()
}

func getSessionCacheKey(sessionID uuid.UUID) string {
	return "sessions/" + sessionID.String()
}
