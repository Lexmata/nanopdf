// Package nanopdf - Font types and operations
package nanopdf

// Font represents a font for text rendering
type Font struct {
	handle uintptr
	ctx    uintptr
}

// NewFont creates a new font
func NewFont(ctx *Context, name string, isBold, isItalic bool) (*Font, error) {
	handle := fontNew(ctx.Handle(), name, isBold, isItalic)
	if handle == 0 {
		return nil, ErrGeneric( "failed to create font")
	}

	return &Font{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// NewFontFromFile loads a font from a file
func NewFontFromFile(ctx *Context, name, path string, index int) (*Font, error) {
	handle := fontNewFromFile(ctx.Handle(), name, path, index)
	if handle == 0 {
		return nil, ErrGeneric( "failed to load font from file")
	}

	return &Font{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// NewFontFromMemory loads a font from memory
func NewFontFromMemory(ctx *Context, name string, data []byte, index int) (*Font, error) {
	handle := fontNewFromMemory(ctx.Handle(), name, data, index)
	if handle == 0 {
		return nil, ErrGeneric( "failed to load font from memory")
	}

	return &Font{
		handle: handle,
		ctx:    ctx.Handle(),
	}, nil
}

// Drop releases the font resources
func (f *Font) Drop() {
	if f.handle != 0 {
		fontDrop(f.ctx, f.handle)
		f.handle = 0
	}
}

// Name returns the font name
func (f *Font) Name() string {
	return fontName(f.ctx, f.handle)
}

// IsBold returns whether the font is bold
func (f *Font) IsBold() bool {
	return fontIsBold(f.ctx, f.handle)
}

// IsItalic returns whether the font is italic
func (f *Font) IsItalic() bool {
	return fontIsItalic(f.ctx, f.handle)
}

// EncodeCharacter encodes a Unicode character to a glyph ID
func (f *Font) EncodeCharacter(unicode int) int {
	return fontEncodeCharacter(f.ctx, f.handle, unicode)
}

// AdvanceGlyph returns the advance width for a glyph
func (f *Font) AdvanceGlyph(glyph int) float32 {
	return fontAdvanceGlyph(f.ctx, f.handle, glyph)
}

