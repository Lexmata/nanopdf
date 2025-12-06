// Package nanopdf - Path types and operations for vector graphics
package nanopdf

// Path represents a vector graphics path
type Path struct {
	handle uintptr
	ctx    uintptr
}

// NewPath creates a new empty path
func NewPath(ctx *Context) (*Path, error) {
	handle := pathNew(ctx.Handle())
	if handle == 0 {
		return nil, ErrGeneric( "failed to create path")
	}

	return &Path{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// Drop releases the path resources
func (p *Path) Drop() {
	if p.handle != 0 {
		pathDrop(p.ctx, p.handle)
		p.handle = 0
	}
}

// MoveTo begins a new sub-path at the specified point
func (p *Path) MoveTo(x, y float32) {
	pathMoveTo(p.ctx, p.handle, x, y)
}

// LineTo adds a straight line to the path
func (p *Path) LineTo(x, y float32) {
	pathLineTo(p.ctx, p.handle, x, y)
}

// CurveTo adds a Bézier curve to the path
func (p *Path) CurveTo(x1, y1, x2, y2, x3, y3 float32) {
	pathCurveTo(p.ctx, p.handle, x1, y1, x2, y2, x3, y3)
}

// ClosePath closes the current sub-path
func (p *Path) ClosePath() {
	pathClosePath(p.ctx, p.handle)
}

// RectTo adds a rectangle to the path
func (p *Path) RectTo(x, y, w, h float32) {
	pathRectTo(p.ctx, p.handle, x, y, w, h)
}

// Handle returns the internal handle (for internal use)
func (p *Path) Handle() uintptr {
	return p.handle
}

