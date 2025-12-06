package nanopdf

import (
	"testing"
)

func TestPath(t *testing.T) {
	ctx := NewContext()
	defer ctx.Drop()

	t.Run("NewPath", func(t *testing.T) {
		path, err := NewPath(ctx)
		if err != nil {
			t.Fatalf("Failed to create path: %v", err)
		}
		defer path.Drop()

		if path.Handle() == 0 {
			t.Error("Path handle is zero")
		}
	})

	t.Run("PathOperations", func(t *testing.T) {
		path, err := NewPath(ctx)
		if err != nil {
			t.Fatalf("Failed to create path: %v", err)
		}
		defer path.Drop()

		// Build a simple path
		path.MoveTo(0, 0)
		path.LineTo(100, 0)
		path.LineTo(100, 100)
		path.LineTo(0, 100)
		path.ClosePath()

		// No errors should occur
	})

	t.Run("Rectangle", func(t *testing.T) {
		path, err := NewPath(ctx)
		if err != nil {
			t.Fatalf("Failed to create path: %v", err)
		}
		defer path.Drop()

		path.RectTo(10, 10, 100, 50)

		// No errors should occur
	})

	t.Run("Curve", func(t *testing.T) {
		path, err := NewPath(ctx)
		if err != nil {
			t.Fatalf("Failed to create path: %v", err)
		}
		defer path.Drop()

		path.MoveTo(0, 0)
		path.CurveTo(10, 20, 30, 40, 50, 60)

		// No errors should occur
	})
}

