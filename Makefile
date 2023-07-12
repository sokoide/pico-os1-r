TARGETDIR := target/thumbv6m-none-eabi/debug

BUILD_ARTIFACT_NAME := pico-os1-rs
BUILD_ARTIFACT_EXTENSION :=
BUILD_ARTIFACT_PREFIX :=
BUILD_ARTIFACT := $(BUILD_ARTIFACT_PREFIX)$(BUILD_ARTIFACT_NAME)$(if $(BUILD_ARTIFACT_EXTENSION),.$(BUILD_ARTIFACT_EXTENSION),)
BUILD_MAP = $(BUILD_ARTIFACT_NAME).map
BUILD_HEX = $(BUILD_ARTIFACT_NAME).hex
BUILD_SIZE = $(BUILD_ARTIFACT_NAME).siz
EMULATOR_PATH = $(HOME)/repo/sokoide/rp2040js
export picotarget = $(realpath $(TARGETDIR)/$(BUILD_HEX))

.PHONY: all main-build run clean secondary-outputs  reset disass

all: main-build

main-build: $(TARGETDIR)/$(BUILD_ARTIFACT) secondary-outputs

$(TARGETDIR)/$(BUILD_ARTIFACT): *.rs
	@echo making $@...
	cargo build

$(TARGETDIR)/$(BUILD_SIZE): $(TARGETDIR)/$(BUILD_ARTIFACT)
	@echo 'Invoking: GNU Arm Cross Print Size'
	arm-none-eabi-size --format=berkeley $(TARGETDIR)/$(BUILD_ARTIFACT)
	@echo 'Finished building: $@'
	@echo ' '

run: $(TARGETDIR)/$(BUILD_HEX)
	@echo running $(picotarget)...
	cd  $(EMULATOR_PATH) && npm start

$(TARGETDIR)/$(BUILD_HEX): $(TARGETDIR)/$(BUILD_ARTIFACT)
	cargo build && cargo objcopy -- -O ihex $@

clean:
	rm -rf ./target

secondary-outputs: $(TARGETDIR)/$(BUILD_HEX) $(TARGETDIR)/$(BUILD_SIZE)

reset: $(TARGETDIR)/$(BUILD_HEX)
	ps -ef|grep node | grep emulator-run |grep -v grep |awk '{print("kill -HUP "$$2)}'| sh

disass: $(TARGETDIR)/$(BUILD_ARTIFACT)
	arm-none-eabi-objdump -D $<