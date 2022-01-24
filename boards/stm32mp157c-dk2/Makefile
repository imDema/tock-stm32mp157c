# Makefile for building the tock kernel for the stm32412gdiscovery platform
#
TARGET=thumbv7em-none-eabi
PLATFORM=stm32mp157c-dk2

include ../../tock/boards/Makefile.common

# Default target for installing the kernel.
.PHONY: install
install: flash

# .PHONY: flash-debug
# flash-debug: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/debug/$(PLATFORM).elf
# 	$(OPENOCD) $(OPENOCD_OPTIONS) -c "init; reset halt; flash write_image erase $<; verify_image $<; reset; shutdown"

# .PHONY: flash
# flash: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
# 	$(OPENOCD) $(OPENOCD_OPTIONS) -c "init; reset halt; flash write_image erase $<; verify_image $<; reset; shutdown"

.PHONY: program
program: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
	$(Q)$(OBJCOPY) --output-target=elf32-littlearm -S --remove-section .apps $< $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).bin
	$(Q)$(SHA256SUM) $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).bin
