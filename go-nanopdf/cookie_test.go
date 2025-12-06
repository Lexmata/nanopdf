package nanopdf

import (
	"testing"
)

func TestCookie(t *testing.T) {
	ctx := NewContext()
	defer ctx.Drop()

	t.Run("NewCookie", func(t *testing.T) {
		cookie, err := NewCookie(ctx)
		if err != nil {
			t.Fatalf("Failed to create cookie: %v", err)
		}
		defer cookie.Drop()

		if cookie.Handle() == 0 {
			t.Error("Cookie handle is zero")
		}
	})

	t.Run("Progress", func(t *testing.T) {
		cookie, err := NewCookie(ctx)
		if err != nil {
			t.Fatalf("Failed to create cookie: %v", err)
		}
		defer cookie.Drop()

		progress := cookie.Progress()
		if progress < 0 || progress > 100 {
			t.Errorf("Invalid progress value: %d", progress)
		}
	})

	t.Run("Abort", func(t *testing.T) {
		cookie, err := NewCookie(ctx)
		if err != nil {
			t.Fatalf("Failed to create cookie: %v", err)
		}
		defer cookie.Drop()

		// Initially not aborted
		if cookie.IsAborted() {
			t.Error("Cookie should not be aborted initially")
		}

		// Abort the cookie
		cookie.Abort()

		// Should be aborted now
		if !cookie.IsAborted() {
			t.Error("Cookie should be aborted after calling Abort()")
		}
	})

	t.Run("Reset", func(t *testing.T) {
		cookie, err := NewCookie(ctx)
		if err != nil {
			t.Fatalf("Failed to create cookie: %v", err)
		}
		defer cookie.Drop()

		// Abort
		cookie.Abort()
		if !cookie.IsAborted() {
			t.Error("Cookie should be aborted")
		}

		// Reset
		cookie.Reset()
		if cookie.IsAborted() {
			t.Error("Cookie should not be aborted after reset")
		}
	})
}

