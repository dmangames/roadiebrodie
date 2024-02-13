SRC_FILES       = $(shell find src -type f)
TEMPLATE_FILES  = $(shell find templates -type f)
STATIC_FILES    = $(shell find static -type f) static/styles.css
TAILWIND_CONFIG = tailwind/config.js

.DEFAULT_GOAL   = target/debug/roadiebrodie

.PHONY: run
run: target/debug/roadiebrodie
	./$<

target/debug/roadiebrodie: $(SRC_FILES) $(TEMPLATE_FILES) $(STATIC_FILES)
	cargo build
	@touch $@

static/styles.css: tailwind/styles.css $(TAILWIND_CONFIG)
	npx tailwindcss --input $< --output $@ --config $(TAILWIND_CONFIG)

