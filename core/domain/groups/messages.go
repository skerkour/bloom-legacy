package groups

type Groups struct {
	Groups []Group `json:"groups"`
}

type FetchGroupMembersParams struct {
	ID string `json:"id"`
}

type FetchGroupDetailsParams struct {
	ID string `json:"id"`
}
