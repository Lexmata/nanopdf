// Package nanopdf - Colorspace types and operations
package nanopdf

// ColorspaceType represents different colorspace types
type ColorspaceType int

const (
	// ColorspaceNone represents no colorspace
	ColorspaceNone ColorspaceType = 0
	// ColorspaceGray represents grayscale
	ColorspaceGray ColorspaceType = 1
	// ColorspaceRGB represents RGB
	ColorspaceRGB ColorspaceType = 2
	// ColorspaceBGR represents BGR
	ColorspaceBGR ColorspaceType = 3
	// ColorspaceCMYK represents CMYK
	ColorspaceCMYK ColorspaceType = 4
)

// Colorspace represents a PDF colorspace
type Colorspace struct {
	handle uintptr
	ctx    uintptr
}

// DeviceRGB returns the device RGB colorspace
func DeviceRGB(ctx *Context) *Colorspace {
	handle := colorspaceDeviceRGB(ctx.Handle())
	return &Colorspace{
		handle: handle,
		ctx:    ctx.Handle(),
	}
}

// DeviceGray returns the device grayscale colorspace
func DeviceGray(ctx *Context) *Colorspace {
	handle := colorspaceDeviceGray(ctx.Handle())
	return &Colorspace{
		handle: handle,
		ctx:    ctx.Handle(),
	}
}

// DeviceBGR returns the device BGR colorspace
func DeviceBGR(ctx *Context) *Colorspace {
	handle := colorspaceDeviceBGR(ctx.Handle())
	return &Colorspace{
		handle: handle,
		ctx:    ctx.Handle(),
	}
}

// DeviceCMYK returns the device CMYK colorspace
func DeviceCMYK(ctx *Context) *Colorspace {
	handle := colorspaceDeviceCMYK(ctx.Handle())
	return &Colorspace{
		handle: handle,
		ctx:    ctx.Handle(),
	}
}

// Handle returns the internal handle (for internal use)
func (cs *Colorspace) Handle() uintptr {
	return cs.handle
}

// NumComponents returns the number of color components
func (cs *Colorspace) NumComponents() int {
	return colorspaceN(cs.ctx, cs.handle)
}

// Name returns the colorspace name
func (cs *Colorspace) Name() string {
	return colorspaceName(cs.ctx, cs.handle)
}

// Type returns the colorspace type
func (cs *Colorspace) Type() ColorspaceType {
	switch cs.handle {
	case 1:
		return ColorspaceGray
	case 2:
		return ColorspaceRGB
	case 3:
		return ColorspaceBGR
	case 4:
		return ColorspaceCMYK
	default:
		return ColorspaceNone
	}
}

