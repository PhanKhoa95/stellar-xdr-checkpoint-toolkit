package checkpointhttp

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestIsAlignedCheckpoint(t *testing.T) {
	if !IsAlignedCheckpoint(63) || !IsAlignedCheckpoint(0x03dca33f) {
		t.Fatalf("expected aligned checkpoints to be valid")
	}
	if IsAlignedCheckpoint(64) {
		t.Fatalf("expected 64 to be rejected")
	}
}

func TestServeGzipCheckpointETagAnd304(t *testing.T) {
	payload := []byte{0x1f, 0x8b, 0x08, 0x00}
	req1 := httptest.NewRequest(http.MethodGet, "/v1/checkpoints/63/ledger", nil)
	rec1 := httptest.NewRecorder()
	ServeGzipCheckpoint(rec1, req1, payload)
	if rec1.Code != http.StatusOK {
		t.Fatalf("status = %d, want 200", rec1.Code)
	}
	etag := rec1.Header().Get("ETag")
	if etag == "" {
		t.Fatalf("expected non-empty ETag")
	}

	req2 := httptest.NewRequest(http.MethodGet, "/v1/checkpoints/63/ledger", nil)
	req2.Header.Set("If-None-Match", etag)
	rec2 := httptest.NewRecorder()
	ServeGzipCheckpoint(rec2, req2, payload)
	if rec2.Code != http.StatusNotModified {
		t.Fatalf("status = %d, want 304", rec2.Code)
	}
}
