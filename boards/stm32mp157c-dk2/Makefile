# Makefile for building the tock kernel for the stm32412gdiscovery platform
#
TARGET=thumbv7em-none-eabi
PLATFORM=stm32mp157c-dk2

include ../../tock/boards/Makefile.common

APP=../../../libtock-c/examples/blink/build/cortex-m4/cortex-m4.tbf
KERNEL=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM).bin
KERNEL_WITH_APP=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM)-app.bin

# Default target for installing the kernel.
.PHONY: install
install: program


.PHONY: program
program: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
	$(Q)$(OBJCOPY) --output-target=elf32-littlearm -S $< $(KERNEL)
	$(Q)$(SHA256SUM) $(KERNEL)

.PHONY: install-app
install-app: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
	$(Q)arm-none-eabi-objcopy --output-target=elf32-littlearm -S --update-section .apps=$(APP) $< $(KERNEL_WITH_APP)
	$(Q)$(SHA256SUM) $(KERNEL_WITH_APP)
	