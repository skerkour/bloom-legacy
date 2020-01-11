package calculator

type CalcParams struct {
	Expression string `json:"expression"`
}

type CalcResult struct {
	Result string `json:"result"`
}

func Calc(params CalcParams) (CalcResult, error) {
	return CalcResult{Result: params.Expression}, nil
}
