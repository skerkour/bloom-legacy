package objects

import (
	"testing"
)

func TestEncodeDecodeState(t *testing.T) {
	states := []int64{
		-234,
		-1,
		0,
		1,
		234,
	}

	for _, state := range states {
		stateStr := EncodeState(state)
		decodedState, err := DecodeStateString(stateStr)
		if err != nil {
			t.Error(err)
		}
		if decodedState != state {
			t.Errorf("state != decodedState (%d, %d)", state, decodedState)
		}
	}
}
