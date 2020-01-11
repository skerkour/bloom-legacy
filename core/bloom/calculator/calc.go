package calculator

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strconv"
)

func evalExpr(exp ast.Expr) int {
	switch exp := exp.(type) {
	case *ast.BinaryExpr:
		return evalBinaryExpr(exp)
	case *ast.BasicLit:
		switch exp.Kind {
		case token.INT:
			i, _ := strconv.Atoi(exp.Value)
			return i
		}
	}

	return 0
}

func evalBinaryExpr(exp *ast.BinaryExpr) int {
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

func Eval(expression string) (int, error) {
	exp, err := parser.ParseExpr(expression)
	if err != nil {
		return 0, err
	}
	return evalExpr(exp), nil
}

func Calc(params CalcParams) (CalcResult, error) {
	res, err := Eval(params.Expression)
	if err != nil {
		return CalcResult{}, err
	}
	return CalcResult{Result: strconv.Itoa(res)}, nil
}
