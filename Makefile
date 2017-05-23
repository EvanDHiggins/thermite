arch ?= x86_64
target ?= $(arch)-thermite
kernel := target/kernel-$(arch).bin
iso := target/thermite-$(arch).iso

rust_kernel := target/$(target)/debug/libthermite.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	target/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	cargo clean

run: iso
	qemu-system-x86_64 -cdrom $(iso)

debug: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -s -S

gdb:
	rust-os-gdb/bin/rust-gdb "target/kernel-x86_64.bin" -ex "target remote :1234"

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p target/isofiles/boot/grub
	cp $(kernel) target/isofiles/boot/thermite.bin
	cp $(grub_cfg) target/isofiles/boot/grub
	grub-mkrescue -o $(iso) target/isofiles 2> /dev/null


$(kernel): kernel $(assembly_object_files) $(linker_script)
	ld -gc-sections -m elf_x86_64 -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_kernel)

kernel:
	xargo build --target=$(target)

target/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

