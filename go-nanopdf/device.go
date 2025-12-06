// Package nanopdf - Device types and operations for rendering
package nanopdf

// Device represents a rendering device (destination for drawing operations)
type Device struct {
	handle uintptr
	ctx    uintptr
}

// NewDrawDevice creates a device that renders to a pixmap
func NewDrawDevice(ctx *Context, transform Matrix, pixmap *Pixmap) (*Device, error) {
	handle := deviceNewDraw(ctx.Handle(), transform, pixmap.Handle())
	if handle == 0 {
		return nil, ErrGeneric( "failed to create draw device")
	}

	return &Device{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// Drop releases the device resources
func (d *Device) Drop() {
	if d.handle != 0 {
		deviceDrop(d.ctx, d.handle)
		d.handle = 0
	}
}

// Close closes the device and finalizes rendering
func (d *Device) Close() {
	if d.handle != 0 {
		deviceClose(d.ctx, d.handle)
	}
}

// Handle returns the internal handle (for internal use)
func (d *Device) Handle() uintptr {
	return d.handle
}

