# Copyright (C) 2016 Nicolas Lamirault <nicolas.lamirault@gmail.com>

# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

APP = julius

SHELL = /bin/bash

DIR = $(shell pwd)

NO_COLOR=\033[0m
OK_COLOR=\033[32;01m
ERROR_COLOR=\033[31;01m
WARN_COLOR=\033[33;01m

CARGO = cargo

SRC=src

VERSION=$(shell \
        grep "version" Cargo.toml \
        |awk -F'=' '{print $$2}' \
        |sed -e "s/[^0-9.]//g" \
	|sed -e "s/ //g")

all: help

help:
	@echo -e "$(OK_COLOR)==== $(APP) [$(VERSION)] ====$(NO_COLOR)"
	@echo -e "$(WARN_COLOR)build$(NO_COLOR)    :  Make all binaries"
	@echo -e "$(WARN_COLOR)test$(NO_COLOR)     :  Launch unit tests"
	@echo -e "$(WARN_COLOR)run$(NO_COLOR)      :  Run the application"
	@echo -e "$(WARN_COLOR)clean$(NO_COLOR)    :  Cleanup"

clean:
	@echo -e "$(OK_COLOR)[$(APP)] Cleanup$(NO_COLOR)"
	@rm -fr target

.PHONY: build
build:
	@echo -e "$(OK_COLOR)[$(APP)] Build $(NO_COLOR)"
	@$(CARGO) build

.PHONY: release
release:
	@echo -e "$(OK_COLOR)[$(APP)] Build $(NO_COLOR)"
	@$(CARGO) build --release
