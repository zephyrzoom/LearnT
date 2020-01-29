# 逆向

## 代码模式

### ISA

x86的指令集是变长的，包含了以前的指令。

arm是RISC指令，分为三种，一种是arm模式，定长的4个字节；另一种是thumb模式，定长的2个字节，因为thumb指令不够用，所以有创造了thumb-2，2字节和4字节的指令都有，它不是arm模式和thumb模式的叠加；最后64位处理器用的arm64。

其他的RISC定长指令有MIPS、PowerPC和Alpha AXP。

### 进制

8进制通常用在表示权限，如111=>rwx，110=>rw-

### 空函数

空函数编译会生成一个ret指令。

```x86
f:
    ret
```

```c++
void dbg_print (const char *fmt, ...)
{
    #ifdef _DEBUG
    // open log file
    // write to log file
    // close log file
    #endif
};
void some_function()
{
    dbg_print ("we did something\n");
};
```

调试的时候经常会用到，

```c
void save_file ()
{
    #ifndef DEMO
    // a real saving code
    #endif
};
```

还有在Demo软件版本中，demo版本无法保存文件，购买后才能用，那么破解者会去想办法修改这里。

### 有返回值的函数

有返回值的函数他的返回值会放到eax中，ret指令会返回eax的值给调用者

```asm
f:
    mov eax, 123
    ret
```

### hello world

```MSVC
CONST SEGMENT
$SG3830 DB 'hello, world', 0AH, 00H ; 编译器会给字符串命名，字符串以\0结束
CONST ENDS
PUBLIC _main
EXTRN _printf:PROC
; Function compile flags: /Odtp
_TEXT SEGMENT
_main PROC
    push ebp
    mov ebp, esp
    push OFFSET $SG3830
    call _printf
    add esp, 4  ; 移动四个字节，相当于pop操作，但是不会用寄存器返回值，也有编译器使用pop ecx指令的
    xor eax, eax    // 置0操作，他的操作码比mov eax, 0短，有的编译器也会用sub eax, eax
    pop ebp
    ret 0
_main ENDP
_TEXT ENDS
```
