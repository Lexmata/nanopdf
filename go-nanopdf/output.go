// Package nanopdf - Output types and operations
package nanopdf

// Output represents an output stream for writing data
type Output struct {
	handle uintptr
	ctx    uintptr
}

// NewOutputWithPath creates an output stream to a file
func NewOutputWithPath(ctx *Context, filename string, append bool) (*Output, error) {
	handle := outputNewWithPath(ctx.Handle(), filename, append)
	if handle == 0 {
		return nil, ErrGeneric( "failed to create output: "+filename)
	}

	return &Output{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// NewOutputWithBuffer creates an output stream to a buffer
func NewOutputWithBuffer(ctx *Context, buffer *Buffer) (*Output, error) {
	handle := outputNewWithBuffer(ctx.Handle(), buffer.Handle())
	if handle == 0 {
		return nil, ErrGeneric( "failed to create output with buffer")
	}

	return &Output{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// Drop releases the output resources
func (o *Output) Drop() {
	if o.handle != 0 {
		outputDrop(o.ctx, o.handle)
		o.handle = 0
	}
}

// WriteData writes raw data to the output
func (o *Output) WriteData(data []byte) error {
	outputWriteData(o.ctx, o.handle, data)
	return nil
}

// WriteString writes a string to the output
func (o *Output) WriteString(s string) error {
	outputWriteString(o.ctx, o.handle, s)
	return nil
}

// WriteByte writes a single byte to the output
func (o *Output) WriteByte(b byte) error {
	outputWriteByte(o.ctx, o.handle, b)
	return nil
}

// Close closes the output and flushes any buffered data
func (o *Output) Close() error {
	if o.handle != 0 {
		outputClose(o.ctx, o.handle)
	}
	return nil
}

// Tell returns the current position in the output
func (o *Output) Tell() int64 {
	return outputTell(o.ctx, o.handle)
}

// Write implements io.Writer interface
func (o *Output) Write(p []byte) (n int, err error) {
	err = o.WriteData(p)
	if err != nil {
		return 0, err
	}
	return len(p), nil
}

