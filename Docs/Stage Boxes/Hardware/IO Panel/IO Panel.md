The I/O Panel is how we Interface with both the User and the Console
![[Screenshot 2026-02-14 at 11.11.57 AM.png]]
## World Clock - 1
This is the world clock input for the stage Box
- Connectors - BNC
- Signal - World Clock

## "Service" Port - 2
The Service/Diagnostics port for the stage box
- Connector - USB-C
- Signal - Serial, J-TAG
- Function(s)
    - Programing the internal FPGA and Micro controller
    - Diagnosing problems in the field and development

## Management Network - 3
The Management Network for the Stage Box
- Connector - Dual RJ45
- Signal - 1000 BASE-T Ethernet
- Function(s)
    - Management of settings and configuration for the micro controller and FPGA
    - Web-UI
    - Other Task that don't need to be 10GbE

## Audio Network - 4
The 10GbE Network for all digital audio communication
- Connector - Dual SFP+
- Signal - SFP+ Module ( Fiber or Copper )
- Function(s)
    - Digital Audio Connection (Dante, Ravenna, AES67, Custom Protocols)

## Control Panel - 5
The control and configuration panel for the stage box
- OLED Screen
    - 2 Color or RGB
    - Menu and audio level display
- D-Pad ( maybe swapped for a encoder /w click )
    - Menu navigation

## Device Info Label - 6
The factory printed device label standardized format across all MixerOS products
- Logo
- Info
    - Device Model
    - Serial Number
    - MAC addresses for Ethernet ports
    - Other Compliance and regulatory information

## Power Supplies - 7
The power supplies for the stage box
- Operation Modes
    - Single PSU mode
    - Dual PSU mode ( Redundant )
- Capacity - 300w - 500w
- Hot-swap capable
- Smart Monitoring
    - Metrics and event monitoring
    - Remote Power on/off
    - Input Monitoring
        - Voltage
        - Amps
    - Output Monitoring
        - Voltage
        - Amps