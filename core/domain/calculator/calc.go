package calculator

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strconv"

	"gitlab.com/bloom42/bloom/core/messages"
)

func Eval(expression string) (float64, error) {
	exp, err := parser.ParseExpr(expression)
	if err != nil {
		return 0, err
	}
	return evalExpr(exp), nil
}

func Calc(params messages.CalcParams) (messages.CalcResult, error) {
	res, err := Eval(params.Expression)
	if err != nil {
		return messages.CalcResult{}, err
	}
	return messages.CalcResult{Result: strconv.FormatFloat(res, 'f', -1, 64)}, nil
}

func evalExpr(exp ast.Expr) float64 {
	switch exp := exp.(type) {
	case *ast.BinaryExpr:
		return evalBinaryExpr(exp)
	case *ast.BasicLit:
		switch exp.Kind {
		case token.INT, token.FLOAT:
			i, _ := strconv.ParseFloat(exp.Value, 64)
			return i
		}
	}

	return 0
}

func evalBinaryExpr(exp *ast.BinaryExpr) float64 {
	left := evalExpr(exp.X)
	right := evalExpr(exp.Y)

	switch exp.Op {
	case token.ADD:
		return left + right
	case token.SUB:
		return left - right
	case token.MUL:
		return left * right
	case token.QUO:
		return left / right
	}

	return 0
}
