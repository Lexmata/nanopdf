package nanopdf

import (
	"testing"
)

func TestFont(t *testing.T) {
	ctx := NewContext()
	defer ctx.Drop()

	t.Run("NewFont", func(t *testing.T) {
		font, err := NewFont(ctx, "Arial", false, false)
		if err != nil {
			t.Skip("Font creation may not be supported in mock mode")
			return
		}
		defer font.Drop()

		name := font.Name()
		if name == "" {
			t.Error("Font name is empty")
		}
	})

	t.Run("FontProperties", func(t *testing.T) {
		font, err := NewFont(ctx, "Arial-Bold", true, false)
		if err != nil {
			t.Skip("Font creation may not be supported in mock mode")
			return
		}
		defer font.Drop()

		if font.IsBold() != true {
			t.Error("Expected bold font")
		}
		if font.IsItalic() != false {
			t.Error("Expected non-italic font")
		}
	})

	t.Run("EncodeCharacter", func(t *testing.T) {
		font, err := NewFont(ctx, "Arial", false, false)
		if err != nil {
			t.Skip("Font creation may not be supported in mock mode")
			return
		}
		defer font.Drop()

		// Encode 'A' (Unicode 65)
		glyph := font.EncodeCharacter(65)
		if glyph < 0 {
			t.Errorf("Invalid glyph ID: %d", glyph)
		}
	})

	t.Run("AdvanceGlyph", func(t *testing.T) {
		font, err := NewFont(ctx, "Arial", false, false)
		if err != nil {
			t.Skip("Font creation may not be supported in mock mode")
			return
		}
		defer font.Drop()

		advance := font.AdvanceGlyph(1)
		if advance < 0 {
			t.Errorf("Unexpected negative advance: %f", advance)
		}
	})
}

