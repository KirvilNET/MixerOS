All Stage Boxes have similar "Brain Boards" and other wise only differ in their slot count and Interface panel layout changes

## Setups

- 8 or more module slots
    - Processor - AMD Zynq 7020
    - RAM - 1GB
    - Storage
        - 32Mb QSPI
        - 16GB eMMC
- less than 8
    - Processor - AMD Zynq 7010
    - RAM - 1GB
    - Storage
        - 32Mb QSPI
        - 16GB eMMC
## Networking

The Zynq FPGA line has 2 builtin 10/100/1000Mb Ethernet interfaces. We will be using these for our management interfaces. This leaves the SFP+ interfaces that we have to provide externally.

- Copper Networking ( 10/100/1000Mb )
    - PS layer controlled
- SFP ( 10/100/1000Mb )
    - PS layer Ethernet driver
    - PL for I2C identification