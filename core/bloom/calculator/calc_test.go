package calculator

import "testing"

func TestEval(t *testing.T) {
	var expected float64
	var exp string
	var res float64
	var err error

	exp = "1+1"
	expected = 2
	res, err = Eval(exp)
	if err != nil {
		t.Errorf("err is not null: %v", err)
	}
	if expected != res {
		t.Errorf("%s = %f; want %f", exp, res, expected)
	}

	exp = "1 + 1"
	expected = 2
	res, err = Eval(exp)
	if err != nil {
		t.Errorf("err is not null: %v", err)
	}
	if expected != res {
		t.Errorf("%s = %f; want %f", exp, res, expected)
	}

	exp = "42"
	expected = 42
	res, err = Eval(exp)
	if err != nil {
		t.Errorf("err is not null: %v", err)
	}
	if expected != res {
		t.Errorf("%s = %f; want %f", exp, res, expected)
	}

	exp = "42 - 1"
	expected = 41
	res, err = Eval(exp)
	if err != nil {
		t.Errorf("err is not null: %v", err)
	}
	if expected != res {
		t.Errorf("%s = %f; want %f", exp, res, expected)
	}

	exp = "2 * 4"
	expected = 8
	res, err = Eval(exp)
	if err != nil {
		t.Errorf("err is not null: %v", err)
	}
	if expected != res {
		t.Errorf("%s = %f; want %f", exp, res, expected)
	}
}
