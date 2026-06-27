package types

import (
	"encoding/json"
	"fmt"
)

type ContentPart struct {
	Value ContentPartValue
}

type ContentPartValue interface {
	isContentPart()
}

func (v TextContentPart) isContentPart() {}
func (v DataContentPart) isContentPart() {}
func (v MediaContentPart) isContentPart() {}
func (v SignalContentPart) isContentPart() {}
func (v StreamRefContentPart) isContentPart() {}

func (v *ContentPart) UnmarshalJSON(data []byte) error {
	var discriminator struct {
		Kind string `json:"kind"`
	}
	if err := json.Unmarshal(data, &discriminator); err != nil {
		return err
	}

	switch discriminator.Kind {
	case "text":
		var value TextContentPart
		if err := json.Unmarshal(data, &value); err != nil {
			return err
		}
		v.Value = value
		return nil
	case "data":
		var value DataContentPart
		if err := json.Unmarshal(data, &value); err != nil {
			return err
		}
		v.Value = value
		return nil
	case "media":
		var value MediaContentPart
		if err := json.Unmarshal(data, &value); err != nil {
			return err
		}
		v.Value = value
		return nil
	case "signal":
		var value SignalContentPart
		if err := json.Unmarshal(data, &value); err != nil {
			return err
		}
		v.Value = value
		return nil
	case "stream_ref":
		var value StreamRefContentPart
		if err := json.Unmarshal(data, &value); err != nil {
			return err
		}
		v.Value = value
		return nil
	default:
		return fmt.Errorf("unknown kind discriminator: %s", discriminator.Kind)
	}
}

func (v ContentPart) MarshalJSON() ([]byte, error) {
	if v.Value == nil {
		return []byte("null"), nil
	}
	return json.Marshal(v.Value)
}
