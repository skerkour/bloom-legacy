.PHONY: build

DIST_DIR = dist

build:
	make -C server build
	cp -r server/dist $(DIST_DIR)
	make -C webapp build
	cp -r webapp/dist $(DIST_DIR)/public
