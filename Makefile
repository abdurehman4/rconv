build:
	@echo 'Building UConv!!'
	@mkdir -p ./build/bin/
	@cargo build --release
	@cp ./LICENSE ./build/
	@cp ./target/release/uconv ./build/bin/
	@echo 'Building Done!!'
	@echo 'Binaries are placed in ./build/bin/'

help:
	@echo 'build     - Build the program.'
	@echo 'clean     - Remove the previous build.'
	@echo 'install   - Install the built program. (root access required!)'

clean:
	@echo 'Removing previuos build!!'
	@rm -R ./build

install:
	@echo 'Installing UConv!!!!'
	@cp ./build/bin/uconv /usr/bin/
	@mkdir -p /usr/share/licenses/uconv/
	@cp ./build/LICENSE /usr/share/licenses/uconv/
	@echo 'Done!!!'
