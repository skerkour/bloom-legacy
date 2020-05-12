package objects

func dedupObject(object *StoredObject, username []byte) (*StoredObject, error) {
	var err error
	ret := &StoredObject{}

	ret.Data = make([]byte, len(object.Data))
	copy(ret.Data, object.Data)
	ret.Type = object.Type
	ret.GroupID = object.GroupID
	ret.OutOfSync = true
	ret.ID, err = GenerateObjectID(username)
	return ret, err
}
