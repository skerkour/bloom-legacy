package calculator

func Calc(params CalcParams) (CalcResult, error) {
	return CalcResult{Result: params.Expression}, nil
}
