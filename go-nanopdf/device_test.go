package nanopdf

import (
	"testing"
)

func TestDevice(t *testing.T) {
	ctx := NewContext()
	defer ctx.Drop()

	t.Run("NewDrawDevice", func(t *testing.T) {
		// Create a pixmap to render to
		cs := DeviceRGB(ctx)
		pixmap, err := NewPixmap(ctx, cs, 100, 100, false)
		if err != nil {
			t.Skip("Pixmap creation may not be supported in mock mode")
			return
		}
		defer pixmap.Drop()

		// Create draw device
		device, err := NewDrawDevice(ctx, MatrixIdentity(), pixmap)
		if err != nil {
			t.Fatalf("Failed to create draw device: %v", err)
		}
		defer device.Drop()

		if device.Handle() == 0 {
			t.Error("Device handle is zero")
		}
	})

	t.Run("DeviceClose", func(t *testing.T) {
		cs := DeviceRGB(ctx)
		pixmap, err := NewPixmap(ctx, cs, 100, 100, false)
		if err != nil {
			t.Skip("Pixmap creation may not be supported in mock mode")
			return
		}
		defer pixmap.Drop()

		device, err := NewDrawDevice(ctx, MatrixIdentity(), pixmap)
		if err != nil {
			t.Fatalf("Failed to create draw device: %v", err)
		}
		defer device.Drop()

		// Close device
		device.Close()

		// Should not panic
	})
}

// NewPixmap helper function for testing
func NewPixmap(ctx *Context, cs *Colorspace, width, height int, alpha bool) (*Pixmap, error) {
	handle := pixmapNew(ctx.Handle(), cs.Handle(), width, height, alpha)
	if handle == 0 {
		return nil, ErrGeneric("failed to create pixmap")
	}

	return &Pixmap{
		ptr: handle,
		ctx: ctx,
	}, nil
}

