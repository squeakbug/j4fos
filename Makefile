# riscv64-unknown-elf- or riscv64-linux-gnu-
# perhaps in /opt/riscv/bin
TOOLPREFIX := riscv64-unknown-elf-

SRC := src

QEMU := qemu-system-riscv64

CC := $(TOOLPREFIX)gcc
AS := $(TOOLPREFIX)as
LD := $(TOOLPREFIX)ld
OBJCOPY := $(TOOLPREFIX)objcopy
OBJDUMP := $(TOOLPREFIX)objdump

TARGET := riscv64gc-unknown-none-elf
TYPE := debug
RELEASE_FLAG := 
TARGET_PATH := ./target/$(TARGET)/$(TYPE)
KERNEL_LIBS := $(TARGET_PATH)
KERNEL_LIB_OUT := $(KERNEL_LIBS)/libkernel.a
KERNEL_LIB_ASM := kernel_lib.S
RUSTFLAGS := -C soft-float -C panic=abort

LINKFLAGS := -T $(LINKSCRIPT)
LINKFLAGS += -Map $(OBJDIR)map.txt
LINKFLAGS += --gc-sections
LINKFLAGS += -z max-page-size=0x1000

$(KERNEL_LIB_OUT):
	RUSTFLAGS="$(RUSTFLAGS)" cargo build --target=$(TARGET) $(RELEASE_FLAG)
	$(OBJDUMP) -d $(KERNEL_LIB_OUT) > $(KERNEL_LIB_ASM)

CFLAGS = -Wall -Werror -O -fno-omit-frame-pointer -ggdb -gdwarf-2
CFLAGS += -MD
CFLAGS += -Wl,--gc-sections -mcmodel=medany -march=rv64gc
CFLAGS += -Wl,--no-warn-rwx-segments
CFLAGS += -ffreestanding -nostartfiles -nostdlib -nodefaultlibs -fno-common -mno-relax
CFLAGS += -I.
CFLAGS += $(shell $(CC) -fno-stack-protector -E -x c /dev/null >/dev/null 2>&1 && echo -fno-stack-protector)

ifneq ($(shell $(CC) -dumpspecs 2>/dev/null | grep -e '[^f]no-pie'),)
CFLAGS += -fno-pie -no-pie
endif
ifneq ($(shell $(CC) -dumpspecs 2>/dev/null | grep -e '[^f]nopie'),)
CFLAGS += -fno-pie -nopie
endif

LDFLAGS = -z max-page-size=4096 --gc-sections -Map map.txt

OBJS = \
	$(SRC)/entry.o \
	$(KERNEL_LIB_OUT)
#  $(SRC)/start.o \
#  $(SRC)/main.o

kernel_img: $(OBJS) $(SRC)/kernel.ld
	$(LD) $(LDFLAGS) -T $(SRC)/kernel.ld -o kernel_img $(OBJS)
	$(OBJDUMP) -S kernel_img > kernel.asm
	$(OBJDUMP) -t kernel_img | sed '1,/SYMBOL TABLE/d; s/ .* / /; /^$$/d' > kernel.sym

tags: $(OBJS) _init
	etags $(SRC)/*.s $(SRC)/*.c

# Prevent deletion of intermediate files, e.g. cat.o, after first build, so
# that disk image changes after first build are persistent until clean.  More
# details:
# http://www.gnu.org/software/make/manual/html_node/Chained-Rules.html
.PRECIOUS: %.o

-include *.d

clean: 
	rm -rf *.tex *.dvi *.idx *.aux *.log *.ind *.ilg \
	*.o */*.o *.d */*.d *.asm */*.asm *.sym */*.sym map.txt *.S \
	target kernel_img

# try to generate a unique GDB port
GDBPORT = 1234

# QEMU's gdb stub command line changed in 0.11
QEMUGDB = $(shell if $(QEMU) -help | grep -q '^-gdb'; \
	then echo "-gdb tcp::$(GDBPORT)"; \
	else echo "-s -p $(GDBPORT)"; fi)

QEMUOPTS = \
	-machine virt \
	-bios none \
	-kernel kernel_img \
	-m 128M \
	-cpu rv64 \
	-smp 1 \
	-serial mon:stdio \
	-device virtio-rng-device \
	-device virtio-gpu-device \
	-device virtio-net-device \
	-device virtio-tablet-device \
	-device virtio-keyboard-device

qemu: kernel_img
	$(QEMU) $(QEMUOPTS)

qemu-gdb: kernel_img .gdbinit.tmpl-riscv
	@echo "*** Now run 'gdb' in another window." 1>&2
	$(QEMU) $(QEMUOPTS) -S $(QEMUGDB)
