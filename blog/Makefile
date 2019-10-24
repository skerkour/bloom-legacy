.PHONY: all dev build

BUILD_DIR := public

all: build

dev:
	zola serve --drafts

build:
	zola build

clean:
	rm -rf $(BUILD_DIR)
