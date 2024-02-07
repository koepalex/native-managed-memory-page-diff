# Native managed memory page diff

The purpose of this script is to calculate the difference the output of the `!address` and `!sos maddress` from WinDbg.

## Workflow

(1) Create an memory dump of the target application.

```zsh
export TARGET_PID = 12345
procdump $TARGET_PID -o /tmp/dumps
```

There are couple of ways to create memory dumps (dotnet-dump, gcore), I typically use [procdump for linux](https://github.com/Sysinternals/ProcDump-for-Linux).

(2) Load the dump into WinDBG 

(3) Run `!address` command and copy the table from the command output into an file (e.g. as `address.output`).

The table looks like:

```output
        BaseAddress      EndAddress+1        RegionSize     Type       State                 Protect             Usage
--------------------------------------------------------------------------------------------------------------------------
+        0`00000000     57ae`159b8000     57ae`159b8000                                                            <unknown>  
+     57ae`159b8000     57ae`159c4000        0`0000c000 MEM_PRIVATE MEM_COMMIT  PAGE_READONLY                      Image      [dotnet; "/usr/share/dotnet/dotnet"]
+     57ae`159c4000     57ae`159da000        0`00016000 MEM_PRIVATE MEM_COMMIT  PAGE_EXECUTE_READ                  Image      [dotnet; "/usr/share/dotnet/dotnet"]
+     57ae`159da000     57ae`159db000        0`00001000 MEM_PRIVATE MEM_COMMIT  PAGE_READONLY                      Image      [dotnet; "/usr/share/dotnet/dotnet"]
+     57ae`159db000     57ae`159dc000        0`00001000 MEM_PRIVATE MEM_COMMIT  PAGE_READWRITE                     Image      [dotnet; "/usr/share/dotnet/dotnet"]
+     57ae`159dc000     57ae`15c68000        0`0028c000                                                            <unknown>  
+     57ae`15c68000     57ae`1628b000        0`00623000 MEM_PRIVATE MEM_COMMIT  PAGE_READWRITE                     <unknown>  [................]
+     57ae`1628b000     7d18`98000000     256a`81d75000                                                            <unknown>  
+     7d18`98000000     7d18`98135000        0`00135000 MEM_PRIVATE MEM_COMMIT  PAGE_READWRITE                     <unknown>  [0...............]
+     7d18`98135000     7d18`a0000000        0`07ecb000                                                            <unknown>  
+     7d18`a0000000     7d18`a0115000        0`00115000 MEM_PRIVATE MEM_COMMIT  PAGE_READWRITE                     <unknown>  [0...............]
+     7d18`a0115000     7d18`a4000000        0`03eeb000                                                            <unknown>  
+     7d18`a4000000     7d18`a40f2000        0`000f2000 MEM_PRIVATE MEM_COMMIT  PAGE_READWRITE                     <unknown>  [0...............]
+     7d18`a40f2000     7d18`a8000000        0`03f0e000                                                            <unknown>  
+     7d18`a8000000     7d18`a80cc000        0`000cc000 MEM_PRIVATE MEM_COMMIT  PAGE_READWRITE                     <unknown>  [0...............]
+     7d18`a80cc000     7d18`ac000000        0`03f34000                                                            <unknown>  
+     7d18`ac000000     7d18`ac07b000        0`0007b000 MEM_PRIVATE MEM_COMMIT  PAGE_READWRITE                     <unknown>  [0...............]
[... truncated ...]
```

(4) Load the son-of-strikes WinDBG extension `.load sos`, call `!sos maddress` command and copy the table from the command output into an file (e.g. as `maddress.output`).

The table looks like:

```output
 +--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+ 
 | Memory Kind            |        StartAddr |        EndAddr-1 |         Size | Type        | State       | Protect                | Image                                                   | 
 +--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+ 
 | Image                  |     57ae159b8000 |     57ae159c4000 |      48,00kb | MEM_IMAGE   | MEM_COMMIT  | PAGE_READONLY          | dotnet                                                  | 
 | Image                  |     57ae159c4000 |     57ae159da000 |      88,00kb | MEM_IMAGE   | MEM_COMMIT  | PAGE_EXECUTE_READ      | dotnet                                                  | 
 | Image                  |     57ae159da000 |     57ae159db000 |       4,00kb | MEM_IMAGE   | MEM_COMMIT  | PAGE_READONLY          | dotnet                                                  | 
 | Image                  |     57ae159db000 |     57ae159dc000 |       4,00kb | MEM_IMAGE   | MEM_COMMIT  | PAGE_READWRITE         | dotnet                                                  | 
 | PAGE_READWRITE         |     57ae15c68000 |     57ae1628b000 |       6,14mb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d1898000000 |     7d1898135000 |       1,21mb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18a0000000 |     7d18a0115000 |       1,08mb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18a4000000 |     7d18a40f2000 |     968,00kb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18a8000000 |     7d18a80cc000 |     816,00kb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18ac000000 |     7d18ac07b000 |     492,00kb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18b0000000 |     7d18b004c000 |     304,00kb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18b4000000 |     7d18b644a000 |      36,29mb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18b8000000 |     7d18b80da000 |     872,00kb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18bc000000 |     7d18bc045000 |     276,00kb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 | PAGE_READWRITE         |     7d18c0000000 |     7d18c00bd000 |     756,00kb | MEM_PRIVATE | MEM_COMMIT  | PAGE_READWRITE         |                                                         | 
 [... truncated ...]
```

(5) Run this program to calculate the differences by running `cargo run -- "<PathToMaddress.output>" "<PathAddress.output>"`

The result will look like:

```output
Memory regions found by sos maddress command: 3469
Memory regions found by windbg address command: 3452
After distinct: 353
Memory regions found by windbg address but not by sos maddress: 7
78e13f400000 - 78e145b1b000 Size: 103MB    Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: <unknown>            Usage: [........0.......]
78e15bc00000 - 78e15c6b5000 Size: 10MB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: <unknown>            Usage: [................]
78e14bc00000 - 78e152a22000 Size: 110MB    Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: <unknown>            Usage: [..........1.....]
78e12cc00000 - 78e13f310000 Size: 295MB    Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: <unknown>            Usage: [................]
78e848ec0000 - 78e848ed1000 Size: 68KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_EXECUTE_WRITECOPY   Kind: <unknown>            Usage: [P.O..V..........]
78e154800000 - 78e15ba31000 Size: 114MB    Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: <unknown>            Usage: [................]
78e12a000000 - 78e12c713000 Size: 39MB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: <unknown>            Usage: [................]
Total unaccounted size: 672MB
Memory regions found by sos maddress but not by windbg address: 370
78e84b3a0000 - 78e84b3b0000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: HighFrequencyHeap    Usage: 
78e84a930000 - 78e84a940000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: LowFrequencyHeap     Usage: 
78e84b1e0000 - 78e84b1f0000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: LowFrequencyHeap     Usage: 
78e84a1e0000 - 78e84a1f0000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: HighFrequencyHeap    Usage: 
78e849950000 - 78e849960000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: LowFrequencyHeap     Usage: 
78e849ebaa00 - 78e849ebb000 Size: 1KB      Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READONLY            Kind: PAGE_READONLY        Usage: 
78e84ce60000 - 78e84ce70000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: LowFrequencyHeap     Usage: 
78e8bf073000 - 78e8bf076000 Size: 12KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: PAGE_READWRITE       Usage: 
78e848750000 - 78e848760000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: HighFrequencyHeap    Usage: 
78e848ee0000 - 78e848ef0000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: LowFrequencyHeap     Usage: 
78e84cf10000 - 78e84cf20000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: LowFrequencyHeap     Usage:
78e849690000 - 78e8496a0000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: LowFrequencyHeap     Usage:
78e84c020000 - 78e84c030000 Size: 64KB     Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: LowFrequencyHeap     Usage:
78e8bebcd000 - 78e8bebcf000 Size: 8KB      Type: MEM_IMAGE    State: MEM_COMMIT   Protection: PAGE_READONLY            Kind: Image                Usage: System_Security_Cryptography_Encoding
78e848ed0000 - 78e848ed1000 Size: 4KB      Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_EXECUTE_WRITECOPY   Kind: HostCodeHeap         Usage:
78e8c6f5b000 - 78e8c6f9c000 Size: 260KB    Type: MEM_PRIVATE  State: MEM_COMMIT   Protection: PAGE_READWRITE           Kind: PAGE_READWRITE       Usage:
[... truncated ...]
```

