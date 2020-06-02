package objects

import (
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

func dedupObject(object *Object, username []byte) (*Object, error) {
	var err error
	ret := &Object{}

	ret.Data = make([]byte, len(object.Data))
	copy(ret.Data, object.Data)
	ret.Type = object.Type
	ret.GroupID = object.GroupID
	ret.OutOfSync = true
	ret.ID, err = GenerateObjectID(username)
	dedupObjectAddConflict(ret)
	return ret, err
}

func dedupObjectAddConflict(object *Object) {
	objDataMap := map[string]interface{}{}
	var err error

	err = json.Unmarshal(object.Data, &objDataMap)
	if err != nil {
		return
	}
	switch object.Type {
	case kernel.OBJECT_TYPE_NOTE:
		if val, ok := objDataMap["title"]; ok {
			if valStr, ok := val.(string); ok {
				objDataMap["title"] = "[Conflict] " + valStr
			}
		}
	case kernel.OBJECT_TYPE_CONTACT:
		if val, ok := objDataMap["firstName"]; ok {
			if valStr, ok := val.(string); ok {
				objDataMap["firstName"] = "[Conflict] " + valStr
			}
		}
	case kernel.OBJECT_TYPE_CALENDAR_EVENT:
		if val, ok := objDataMap["title"]; ok {
			if valStr, ok := val.(string); ok {
				objDataMap["title"] = "[Conflict] " + valStr
			}
		}
	}
	jsonData, err := json.Marshal(objDataMap)
	if err != nil {
		object.Data = jsonData
	}
}
