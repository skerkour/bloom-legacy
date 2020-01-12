package main

import (
	"C"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/bloom/calculator"
)

func handleCalculatorMehtod(method string, jsonParams json.RawMessage) *C.char {
	switch method {
	case "calc":
		var params calculator.CalcParams
		err := json.Unmarshal(jsonParams, &params)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		res, err := calculator.Calc(params)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		payloadOut := PayloadOut{Data: res}
		data, err := json.Marshal(payloadOut)
		if err != nil {
			return nil // TODO(z0mbie42): return error
		}
		return C.CString(string(data))
	default:
		return methodNotFoundError(method, "calculator")
	}
}
