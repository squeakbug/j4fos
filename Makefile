# riscv64-unknown-elf- or riscv64-linux-gnu-
# perhaps in /opt/riscv/bin
TOOLPREFIX = riscv64-unknown-elf-

QEMU = qemu-system-riscv64

CC = $(TOOLPREFIX)gcc
AS = $(TOOLPREFIX)as
LD = $(TOOLPREFIX)ld
OBJCOPY = $(TOOLPREFIX)objcopy
OBJDUMP = $(TOOLPREFIX)objdump

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

LDFLAGS = -z max-page-size=4096

OBJS = \
  entry.o \
  start.o \
  main.o

kernel_img: $(OBJS) kernel.ld
	$(LD) $(LDFLAGS) -T kernel.ld -o kernel_img $(OBJS)
	$(OBJDUMP) -S kernel_img > kernel.asm
	$(OBJDUMP) -t kernel_img | sed '1,/SYMBOL TABLE/d; s/ .* / /; /^$$/d' > kernel.sym

tags: $(OBJS) _init
	etags *.S *.c

# Prevent deletion of intermediate files, e.g. cat.o, after first build, so
# that disk image changes after first build are persistent until clean.  More
# details:
# http://www.gnu.org/software/make/manual/html_node/Chained-Rules.html
.PRECIOUS: %.o

-include *.d

clean: 
	rm -f *.tex *.dvi *.idx *.aux *.log *.ind *.ilg \
	*.o */*.o *.d */*.d *.asm */*.asm *.sym */*.sym \
	kernel_img

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

