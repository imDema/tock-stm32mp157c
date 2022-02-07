# Discovery kit with STM32MP157C MPU
======================================================

For more details [visit STM32MP157C Discovery Kit
website](https://www.st.com/en/evaluation-tools/stm32mp157c-dk2.html).

## Flashing the kernel

The kernel is loaded using the Linux remoteproc framework. The kernel must be compiled to an ELF file which must contain a `.resource_table` section, needed by remoteproc.

<!-- TODO: Start procedure -->

## Flashing app

Apps are built out-of-tree. Once an app is built, you can use
`arm-none-eabi-objcopy` with `--update-section` to create an ELF image with the
apps included.

```bash
$ arm-none-eabi-objcopy  \
    --update-section .apps=../../../libtock-c/examples/c_hello/build/cortex-m4/cortex-m4.tbf \
    target/thumbv7em-none-eabi/release/stm32mp157c-dk2.elf \
    target/thumbv7em-none-eabi/release/stm32mp157c-dk2-app.elf
```

For example, you can update `Makefile` as follows.

```
APP=../../../libtock-c/examples/c_hello/build/cortex-m4/cortex-m4.tbf
KERNEL=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM).elf
KERNEL_WITH_APP=$(TOCK_ROOT_DIRECTORY)/target/$(TARGET)/release/$(PLATFORM)-app.elf

.PHONY: program
program: $(TOCK_ROOT_DIRECTORY)target/$(TARGET)/release/$(PLATFORM).elf
	arm-none-eabi-objcopy --update-section .apps=$(APP) $(KERNEL) $(KERNEL_WITH_APP)
```

After setting `APP`, `KERNEL`, `KERNEL_WITH_APP`, and `program` target
dependency, you can do

```bash
$ make program
```

to flash the image.
