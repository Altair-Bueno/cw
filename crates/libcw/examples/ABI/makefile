CARGO_LIBCW_FOLDER='../../'
CARGO_ROOT_FOLDER='$(CARGO_LIBCW_FOLDER)../../'
OUTPUT_NAME='a.out'

example: cargo_build
	clang ./lib/* ./src/* 
	chmod +x $(OUTPUT_NAME)

run:
	./a.out

cargo_build: make_dirs
	cd $(CARGO_LIBCW_FOLDER);cargo build --release
	cd $(CARGO_LIBCW_FOLDER);cp *.h examples/ABI/lib
	cd $(CARGO_ROOT_FOLDER);cp target/release/liblibcw.a crates/libcw/examples/ABI/lib/libcw.a

make_dirs:
	mkdir lib || true

clean:
	rm -fr lib
	rm $(OUTPUT_NAME)