package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/calculator"
)

func handleCalculatorMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "calc":
		var params calculator.CalcParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		res, err := calculator.Calc(params)
		if err != nil {
			return InternalError(err) // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return methodNotFoundError(method, "calculator")
	}
}
