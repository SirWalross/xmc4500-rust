#!/usr/bin/env bash
cargo objcopy -- -O ihex build/main.hex
JLinkExe -Device XMC4500 -If SWD -Speed 1000 -CommanderScript JLinkCommands