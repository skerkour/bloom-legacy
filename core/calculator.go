package core

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/bloom/calculator"
)

func HandleCalculatorMehtod(method string, jsonParams json.RawMessage) MessageOut {
	switch method {
	case "calc":
		var params calculator.CalcParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return MethodNotFoundError(method, "calculator") // TODO(z0mbie42): return error
		}
		res, err := calculator.Calc(params)
		if err != nil {
			return MethodNotFoundError(method, "calculator") // TODO(z0mbie42): return error
		}
		return MessageOut{Data: res}
	default:
		return MethodNotFoundError(method, "calculator")
	}
}
