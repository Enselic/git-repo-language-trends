package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProgressPadding(t *testing.T) {
	assert.Equal(t, PaddedProgress(2, 5), "2/5", "should be same")
	assert.Equal(t, PaddedProgress(2, 50), " 2/50", "should be same")
	assert.Equal(t, PaddedProgress(2, 500), "  2/500", "should be same")
	assert.Equal(t, PaddedProgress(20, 500), " 20/500", "should be same")
	assert.Equal(t, PaddedProgress(200, 500), "200/500", "should be same")
}
