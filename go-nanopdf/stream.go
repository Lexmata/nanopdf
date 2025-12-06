// Package nanopdf - Stream types and operations
package nanopdf

import "io"

// SeekOrigin represents the origin for seek operations
type SeekOrigin int

const (
	// SeekSet seeks relative to the start of the stream
	SeekSet SeekOrigin = 0
	// SeekCur seeks relative to the current position
	SeekCur SeekOrigin = 1
	// SeekEnd seeks relative to the end of the stream
	SeekEnd SeekOrigin = 2
)

// Stream represents an input stream
type Stream struct {
	handle uintptr
	ctx    uintptr
}

// OpenFile opens a stream from a file
func OpenFile(ctx *Context, filename string) (*Stream, error) {
	handle := streamOpenFile(ctx.Handle(), filename)
	if handle == 0 {
		return nil, ErrFailedToOpen
	}

	return &Stream{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// OpenMemory opens a stream from memory
func OpenMemory(ctx *Context, data []byte) (*Stream, error) {
	handle := streamOpenMemory(ctx.Handle(), data)
	if handle == 0 {
		return nil, ErrGeneric( "failed to open memory stream")
	}

	return &Stream{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// Drop releases the stream resources
func (s *Stream) Drop() {
	if s.handle != 0 {
		streamDrop(s.ctx, s.handle)
		s.handle = 0
	}
}

// Read reads data from the stream
func (s *Stream) Read(p []byte) (int, error) {
	if len(p) == 0 {
		return 0, nil
	}

	n := streamRead(s.ctx, s.handle, p)
	if n == 0 && s.IsEOF() {
		return 0, io.EOF
	}

	return n, nil
}

// ReadByte reads a single byte
func (s *Stream) ReadByte() (byte, error) {
	b := streamReadByte(s.ctx, s.handle)
	if b == -1 {
		return 0, io.EOF
	}
	return byte(b), nil
}

// IsEOF returns whether the stream is at end-of-file
func (s *Stream) IsEOF() bool {
	return streamIsEOF(s.ctx, s.handle)
}

// Seek seeks to a position in the stream
func (s *Stream) Seek(offset int64, whence SeekOrigin) {
	streamSeek(s.ctx, s.handle, offset, int(whence))
}

// Tell returns the current position in the stream
func (s *Stream) Tell() int64 {
	return streamTell(s.ctx, s.handle)
}

