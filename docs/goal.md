# Goal

## Цель

Приемник в VHF дипазоне

## Задачи 

1. Добиться полного использования Ethernet канала
2. Довести проект до конца:
    1. OS kernel
    2. Messaging IPC
    3. RISC-V core
    4. Bus arbiter
    5. MAC and MII layers
    6. DSP core

Can be part of project, but too amazing
* GSM Core
* Jamming? DRFM?
* Systolic arrays with appropriate interface

## Side tasks

* Replace ARM hardcore with RISC-V softcore
* Practice with booloaders and eMMC
* Practice with Chisel, System Verilog and UVM

## Requirements

* Support some embedding DSP library with FIR construction
* Transcoding signals to 1 GbE 
* Easy control of data streams
* Hard-time scheduling with windowing (no dynamic priotities)
* Project can be developed in Yosys, Vivado, Vitis
