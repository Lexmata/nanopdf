// Package nanopdf - Cookie types and operations for progress tracking
package nanopdf

// Cookie represents a progress tracking cookie for long-running operations
type Cookie struct {
	handle uintptr
	ctx    uintptr
}

// NewCookie creates a new cookie for tracking operation progress
func NewCookie(ctx *Context) (*Cookie, error) {
	handle := cookieNew(ctx.Handle())
	if handle == 0 {
		return nil, ErrGeneric("failed to create cookie")
	}

	return &Cookie{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// Drop releases the cookie resources
func (c *Cookie) Drop() {
	if c.handle != 0 {
		cookieDrop(c.ctx, c.handle)
		c.handle = 0
	}
}

// Abort aborts the operation associated with this cookie
func (c *Cookie) Abort() {
	cookieAbort(c.ctx, c.handle)
}

// Progress returns the current progress (0-100)
func (c *Cookie) Progress() int {
	return cookieProgress(c.ctx, c.handle)
}

// IsAborted returns whether the operation has been aborted
func (c *Cookie) IsAborted() bool {
	return cookieIsAborted(c.ctx, c.handle)
}

// Reset resets the cookie to initial state
func (c *Cookie) Reset() {
	cookieReset(c.ctx, c.handle)
}

// Handle returns the internal handle (for internal use)
func (c *Cookie) Handle() uintptr {
	return c.handle
}

