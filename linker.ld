ENTRY(_start)

SECTIONS
{
    /* 仮想アドレスは標準的な位置へ */
    . = 0x8048000;

    /* AT 指定により、物理メモリ上のロード位置を 16MB 以降に強制移動 */
    .text : AT(0x1000000)
    {
        *(.text .text.*)
    }

    .rodata : ALIGN(0x1000)
    {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(0x1000)
    {
        *(.data .data.*)
    }

    .bss : ALIGN(0x1000)
    {
        *(.bss .bss.*)
    }
}