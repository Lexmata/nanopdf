package nanopdf

import (
	"testing"
)

func TestColorspace(t *testing.T) {
	ctx := NewContext()
	defer ctx.Drop()

	t.Run("DeviceRGB", func(t *testing.T) {
		cs := DeviceRGB(ctx)
		if cs == nil {
			t.Fatal("DeviceRGB returned nil")
		}
		if cs.NumComponents() != 3 {
			t.Errorf("Expected 3 components, got %d", cs.NumComponents())
		}
		name := cs.Name()
		if name != "DeviceRGB" {
			t.Errorf("Expected name 'DeviceRGB', got '%s'", name)
		}
		if cs.Type() != ColorspaceRGB {
			t.Errorf("Expected type RGB, got %v", cs.Type())
		}
	})

	t.Run("DeviceGray", func(t *testing.T) {
		cs := DeviceGray(ctx)
		if cs == nil {
			t.Fatal("DeviceGray returned nil")
		}
		if cs.NumComponents() != 1 {
			t.Errorf("Expected 1 component, got %d", cs.NumComponents())
		}
		if cs.Type() != ColorspaceGray {
			t.Errorf("Expected type Gray, got %v", cs.Type())
		}
	})

	t.Run("DeviceCMYK", func(t *testing.T) {
		cs := DeviceCMYK(ctx)
		if cs == nil {
			t.Fatal("DeviceCMYK returned nil")
		}
		if cs.NumComponents() != 4 {
			t.Errorf("Expected 4 components, got %d", cs.NumComponents())
		}
		if cs.Type() != ColorspaceCMYK {
			t.Errorf("Expected type CMYK, got %v", cs.Type())
		}
	})

	t.Run("DeviceBGR", func(t *testing.T) {
		cs := DeviceBGR(ctx)
		if cs == nil {
			t.Fatal("DeviceBGR returned nil")
		}
		if cs.NumComponents() != 3 {
			t.Errorf("Expected 3 components, got %d", cs.NumComponents())
		}
		if cs.Type() != ColorspaceBGR {
			t.Errorf("Expected type BGR, got %v", cs.Type())
		}
	})
}

