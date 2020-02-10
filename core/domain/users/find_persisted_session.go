package users

import (
	"database/sql"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
)

func FindPersistedSession() (*model.SignedIn, error) {
	var ret *model.SignedIn
	me := model.User{}
	session := model.Session{}
	var err error
	var meValue string
	var sessionValue string

	row := db.DB.QueryRow("SELECT value FROM preferences WHERE key = ?", "me")
	if row == nil {
		return ret, nil
	}

	err = row.Scan(&meValue)
	switch err {
	case nil:
		break
	case sql.ErrNoRows:
		return ret, nil
	default:
		return ret, err
	}

	err = json.Unmarshal([]byte(meValue), &me)
	if err != nil {
		return ret, err
	}

	row = db.DB.QueryRow("SELECT value FROM preferences WHERE key = ?", "session")
	if row == nil {
		return ret, nil
	}

	err = row.Scan(&sessionValue)
	switch err {
	case nil:
		break
	case sql.ErrNoRows:
		return ret, nil
	default:
		return ret, err
	}

	err = json.Unmarshal([]byte(sessionValue), &session)
	if err != nil {
		return ret, err
	}

	ret = &model.SignedIn{
		Me:      &me,
		Session: &session,
	}
	return ret, nil
}
