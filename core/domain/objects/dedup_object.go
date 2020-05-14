package objects

import (
	"encoding/json"
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
	case "com.bloom42.bloom.note":
		if val, ok := objDataMap["title"]; ok {
			if valStr, ok := val.(string); ok {
				objDataMap["title"] = "[Conflict] " + valStr
			}
		}
	case "com.bloom42.bloom.contact":
		if val, ok := objDataMap["firstName"]; ok {
			if valStr, ok := val.(string); ok {
				objDataMap["firstName"] = "[Conflict] " + valStr
			}
		}
	case "com.bloom42.bloom.calendar_event":
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
