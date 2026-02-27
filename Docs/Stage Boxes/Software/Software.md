An attempt to explain and document the software integration soup 

# The Main Processor -  AMD Zynq SoC
![[zynq-mp-core-dual1_501467e669fcba329e86f04a138fd9a4843d4def.webp]]
The Zynq SoC consist of two layers: Processing System (PS) and the Programmable Logic (PL) 

## Processing System (PS)
This is the ARM part of the SoC made up of 2 ARM Cortex A9 Cores
- What runs here
	- PetaLinux 
		- Configuration WebUI
		- Digital Audio Routing and Services
		- Other Linux Based Processes
	- Networking
		- Gigabit Ethernet
		- SFP network
## Programable Logic (PL)
- What runs here
	- Networking
		- Only the SFP I2C
	- DAC (Digital to Analog Converter) processing
	- ADC (Analog to Digital Converter) processing
	- Display Interface (I2C connection to its Microcontroller)
	- Power Management (Interfacing with Smart PSUs)

# The Linux OS
Linux is good for this type of task where we want to have a lightweight and customizable operating system. This can handle all of our Networking, WebUI, Basic Audio Processing and other linux task.

