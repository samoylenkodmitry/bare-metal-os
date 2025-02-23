.PHONY: all clean rust-run run1 run2

all: boot.bin boot2.bin

boot.bin: boot.asm
	nasm -f bin boot.asm -o boot.bin

boot2.bin: boot2.asm
	nasm -f bin boot2.asm -o boot2.bin

clean:
	rm -f *.bin
	cargo clean

run1: boot.bin
	qemu-system-x86_64 -drive format=raw,file=boot.bin

run2: boot2.bin
	qemu-system-x86_64 -drive format=raw,file=boot2.bin

rust-run: clean
	cargo bootimage
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
