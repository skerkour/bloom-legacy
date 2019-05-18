.PHONY: all build clean re

DIST_DIR = dist

all: build

build:
	make -C server build
	cp -r server/dist $(DIST_DIR)
	make -C webapp build
	cp -r webapp/dist $(DIST_DIR)/public

clean:
	rm -rf $(DIST_DIR)
	make -C server clean
	make -C webapp clean

re: clean build
