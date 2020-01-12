package calculator

type CalcParams struct {
	Expression string `json:"expression"`
}

type CalcResult struct {
	Result string `json:"result"`
}
