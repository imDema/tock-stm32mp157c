/* Memory layout for the STM32F412G
 * rom = 1MB (LENGTH = 0x01000000)
 * kernel = 192KB
 * user = 832KB
 * ram = 256KB */

MEMORY
{
  interrupts  (rx)  : ORIGIN = 0x00000000, LENGTH = 0x00000298
  rom         (rx)  : ORIGIN = 0x10000000, LENGTH = 0x0001a000
  prog        (rx)  : ORIGIN = 0x1001a000, LENGTH = 0x00006000
  ram         (rwx) : ORIGIN = 0x10020000, LENGTH = 0x00020000
  ipc         (rw)  : ORIGIN = 0x10040000, LENGTH = 0x00008000
}

MPU_MIN_ALIGN = 1K;
