package preferences

type SetParams struct {
	Key   string `json:"key"`
	Value string `json:"value"`
}

type GetParams struct {
	Key string `json:"key"`
}

type DeleteParams struct {
	Key string `json:"key"`
}
