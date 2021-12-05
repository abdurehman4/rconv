build:
	@echo 'Building UConv!!'
	@mkdir -p ./build/bin/
	@cargo build --release
	@cp ./target/release/rconv ./build/bin/
	@strip ./build/bin/rconv
	@echo 'Building Done!!'
	@echo 'Binaries are placed in ./build/bin/'

help:
	@echo 'build         - Build the program.'
	@echo 'clean         - Remove the previous build.'
	@echo 'clean-build   - Remove the previous build and then rebuild.'
	@echo 'install       - Install the built program. (root access required!)'
	@echo 'uninstall     - Uninstall the previous installed program. (root access required!)'
clean:
	@echo 'Removing previuos build!!'
	@rm -R ./build

install:
	@echo 'Installing UConv!!!!'
	@echo 'Copying Files!!!'
	@cp -v ./build/bin/rconv /usr/bin/
	@mkdir -p /usr/share/rconv/
	@cp -v ./LICENSE /usr/share/rconv/
	@cp -v ./README.md /usr/share/rconv/
	@echo 'Done!!!'

uninstall:
	@echo 'Removing UConv!!!!'
	@echo 'Deleting File!!!'
	@rm -Rfv /usr/bin/rconv /usr/share/rconv
	@echo 'Removed UConv!!'

clean-build: clean build
