package config

import (
	"testing"

	"gitlab.com/bloom42/gobox/crypto"
)

func TestStripComments(t *testing.T) {
	input := []byte(`
	{
		// hello
		"a": "aValue",
		// world
		"b": "bValue"// comment
	}
	`)
	expected := []byte(`
	{

		"a": "aValue",

		"b": "bValue"
	}
	`)

	output := stripComments(input)

	if !crypto.ConstantTimeCompare(output, expected) {
		t.Errorf("\nOUTPUT:\n%s\nEXPECTED:\n%s\n", string(output), string(expected))
	}
}
