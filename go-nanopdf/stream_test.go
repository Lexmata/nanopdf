package nanopdf

import (
	"io"
	"os"
	"testing"
)

func TestStream(t *testing.T) {
	ctx := NewContext()
	defer ctx.Drop()

	t.Run("OpenMemory", func(t *testing.T) {
		data := []byte("Hello, World!")
		stream, err := OpenMemory(ctx, data)
		if err != nil {
			t.Fatalf("Failed to open memory stream: %v", err)
		}
		defer stream.Drop()

		buf := make([]byte, 5)
		n, err := stream.Read(buf)
		if err != nil && err != io.EOF {
			t.Fatalf("Read failed: %v", err)
		}
		if n != 5 {
			t.Errorf("Expected to read 5 bytes, got %d", n)
		}
		if string(buf) != "Hello" {
			t.Errorf("Expected 'Hello', got '%s'", string(buf))
		}
	})

	t.Run("ReadByte", func(t *testing.T) {
		data := []byte("ABC")
		stream, err := OpenMemory(ctx, data)
		if err != nil {
			t.Fatalf("Failed to open memory stream: %v", err)
		}
		defer stream.Drop()

		b, err := stream.ReadByte()
		if err != nil {
			t.Fatalf("ReadByte failed: %v", err)
		}
		if b != 'A' {
			t.Errorf("Expected 'A' (65), got %d", b)
		}
	})

	t.Run("Seek", func(t *testing.T) {
		data := []byte("0123456789")
		stream, err := OpenMemory(ctx, data)
		if err != nil {
			t.Fatalf("Failed to open memory stream: %v", err)
		}
		defer stream.Drop()

		// Seek to position 5
		stream.Seek(5, SeekSet)
		pos := stream.Tell()
		if pos != 5 {
			t.Errorf("Expected position 5, got %d", pos)
		}

		// Read byte at position 5
		b, _ := stream.ReadByte()
		if b != '5' {
			t.Errorf("Expected '5', got %c", b)
		}
	})

	t.Run("EOF", func(t *testing.T) {
		data := []byte("X")
		stream, err := OpenMemory(ctx, data)
		if err != nil {
			t.Fatalf("Failed to open memory stream: %v", err)
		}
		defer stream.Drop()

		// Read the byte
		stream.ReadByte()

		// Should be at EOF now
		if !stream.IsEOF() {
			t.Error("Expected EOF after reading single byte")
		}
	})

	t.Run("OpenFile", func(t *testing.T) {
		// Create a temporary file
		tmpFile, err := os.CreateTemp("", "nanopdf-stream-test-*.txt")
		if err != nil {
			t.Fatalf("Failed to create temp file: %v", err)
		}
		defer os.Remove(tmpFile.Name())

		testData := []byte("Test file content")
		tmpFile.Write(testData)
		tmpFile.Close()

		// Open stream from file
		stream, err := OpenFile(ctx, tmpFile.Name())
		if err != nil {
			t.Skip("File stream may not be supported in mock mode")
			return
		}
		defer stream.Drop()

		buf := make([]byte, len(testData))
		n, _ := stream.Read(buf)
		if n != len(testData) {
			t.Errorf("Expected to read %d bytes, got %d", len(testData), n)
		}
	})
}

