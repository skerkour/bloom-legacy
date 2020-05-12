package users

import (
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
	"golang.org/x/net/context"
)

func FindPersistedSession() (*model.SignedIn, error) {
	var ret *model.SignedIn
	me := model.User{}
	session := model.Session{}
	var err error
	var meValue *string
	var sessionValue *string
	ctx := context.Background()

	meValue, err = preferences.Get(ctx, nil, "me")
	if err != nil || meValue == nil {
		return ret, err
	}

	err = json.Unmarshal([]byte(*meValue), &me)
	if err != nil {
		return ret, err
	}

	sessionValue, err = preferences.Get(ctx, nil, "session")
	if err != nil || sessionValue == nil {
		return ret, err
	}

	err = json.Unmarshal([]byte(*sessionValue), &session)
	if err != nil {
		return ret, err
	}

	ret = &model.SignedIn{
		Me:      &me,
		Session: &session,
	}
	return ret, nil
}
