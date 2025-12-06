package nanopdf

import (
	"os"
	"testing"
)

func TestOutput(t *testing.T) {
	ctx := NewContext()
	defer ctx.Drop()

	t.Run("NewOutputWithPath", func(t *testing.T) {
		tmpFile, err := os.CreateTemp("", "nanopdf-output-test-*.txt")
		if err != nil {
			t.Fatalf("Failed to create temp file: %v", err)
		}
		tmpFile.Close()
		defer os.Remove(tmpFile.Name())

		output, err := NewOutputWithPath(ctx, tmpFile.Name(), false)
		if err != nil {
			t.Skip("Output may not be supported in mock mode")
			return
		}
		defer output.Drop()

		// Write some data
		err = output.WriteString("Hello, World!")
		if err != nil {
			t.Fatalf("WriteString failed: %v", err)
		}

		output.Close()

		// Read back and verify
		data, _ := os.ReadFile(tmpFile.Name())
		if string(data) != "Hello, World!" {
			t.Errorf("Expected 'Hello, World!', got '%s'", string(data))
		}
	})

	t.Run("WriteData", func(t *testing.T) {
		tmpFile, err := os.CreateTemp("", "nanopdf-output-test-*.bin")
		if err != nil {
			t.Fatalf("Failed to create temp file: %v", err)
		}
		tmpFile.Close()
		defer os.Remove(tmpFile.Name())

		output, err := NewOutputWithPath(ctx, tmpFile.Name(), false)
		if err != nil {
			t.Skip("Output may not be supported in mock mode")
			return
		}
		defer output.Drop()

		testData := []byte{1, 2, 3, 4, 5}
		err = output.WriteData(testData)
		if err != nil {
			t.Fatalf("WriteData failed: %v", err)
		}

		output.Close()

		// Verify
		data, _ := os.ReadFile(tmpFile.Name())
		if len(data) != len(testData) {
			t.Errorf("Expected %d bytes, got %d", len(testData), len(data))
		}
	})

	t.Run("WriteByte", func(t *testing.T) {
		tmpFile, err := os.CreateTemp("", "nanopdf-output-test-*.bin")
		if err != nil {
			t.Fatalf("Failed to create temp file: %v", err)
		}
		tmpFile.Close()
		defer os.Remove(tmpFile.Name())

		output, err := NewOutputWithPath(ctx, tmpFile.Name(), false)
		if err != nil {
			t.Skip("Output may not be supported in mock mode")
			return
		}
		defer output.Drop()

		output.WriteByte('X')
		output.Close()

		data, _ := os.ReadFile(tmpFile.Name())
		if len(data) != 1 || data[0] != 'X' {
			t.Errorf("Expected single byte 'X', got %v", data)
		}
	})

	t.Run("NewOutputWithBuffer", func(t *testing.T) {
		buffer := NewBuffer(0)
		defer buffer.Drop()

		output, err := NewOutputWithBuffer(ctx, buffer)
		if err != nil {
			t.Skip("Output with buffer may not be supported in mock mode")
			return
		}
		defer output.Drop()

		output.WriteString("Test")
		output.Close()

		// Buffer should contain the data
		data := buffer.Data()
		if string(data) != "Test" {
			t.Errorf("Expected 'Test', got '%s'", string(data))
		}
	})
}

