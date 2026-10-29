package checkpointhttp

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"net/http"
)

const ImmutableCacheControl = "public, max-age=31536000, s-maxage=31536000, immutable"

// IsAlignedCheckpoint reports whether cp is a valid 64-ledger Stellar checkpoint.
func IsAlignedCheckpoint(cp uint32) bool {
	return cp >= 63 && (uint64(cp)+1)%64 == 0
}

// NearestCheckpoint returns the checkpoint sequence containing ledger seq.
func NearestCheckpoint(seq uint32) uint32 {
	return uint32(((uint64(seq)+64)/64)*64 - 1)
}

// ServeGzipCheckpoint writes raw .xdr.gz bytes with strong ETag and immutable headers.
func ServeGzipCheckpoint(w http.ResponseWriter, r *http.Request, rawGz []byte) {
	sum := sha256.Sum256(rawGz)
	etag := fmt.Sprintf(`"%s"`, hex.EncodeToString(sum[:16]))
	w.Header().Set("Content-Type", "application/gzip")
	w.Header().Set("Cache-Control", ImmutableCacheControl)
	w.Header().Set("ETag", etag)
	if r.Header.Get("If-None-Match") == etag {
		w.WriteHeader(http.StatusNotModified)
		return
	}
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write(rawGz)
}
