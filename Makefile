ARCH ?= x86_64
KERNEL := build/kernel-$(ARCH).bin
ISO := build/os-$(ARCH).iso
TARGET ?= $(ARCH)-rust_os
RUST_OS := target/$(TARGET)/debug/librust_os.a

SRCPATH := src/arch/$(ARCH)
OBJPATH := build/arch/$(ARCH)
LINKER_SCRIPT := $(SRCPATH)/linker.ld
GRUB_CFG := $(SRCPATH)/grub.cfg
ASM_SRC := $(wildcard $(SRCPATH)/*.asm)
ASM_OBJ := $(patsubst $(SRCPATH)/%.asm,$(OBJPATH)/%.o,$(ASM_SRC))

ASM := nasm
LD := x86_64-elf-ld
GRUB_MKRESCUE := $(HOME)/opt/grub/bin/grub-mkrescue
QEMU := qemu-system-x86_64

.PHONY: all clean run iso kernel

all: $(KERNEL)

clean:
	@rm -r build

run: $(ISO)
	$(QEMU) -cdrom $(ISO) -nographic -vnc :0

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUB_CFG)
	mkdir -p build/isofiles/boot/grub
	cp $(KERNEL) build/isofiles/boot/kernel.bin
	cp $(GRUB_CFG) build/isofiles/boot/grub
	$(GRUB_MKRESCUE) -o $(ISO) build/isofiles 2> /dev/null
	rm -r build/isofiles

$(KERNEL): kernel $(RUST_OS) $(ASM_OBJ) $(LINKER_SCRIPT)
	$(LD) -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJ) $(RUST_OS)

kernel:
	RUST_TARGET_PATH=$(shell pwd) xargo build --target $(TARGET)

$(OBJPATH)/%.o: $(SRCPATH)/%.asm
	mkdir -p $(shell dirname $@)
	$(ASM) -felf64 $< -o $@
