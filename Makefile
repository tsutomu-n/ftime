## ftime SVG demo workflow (asciinema -> svg-term -> svgo)

# Output files
CAST ?= demo.cast
SVG  ?= demo.svg
MIN  ?= demo.min.svg
# Slowdown factor for playback timing (1 = no change, 2 = 2x slower)
SLOW ?= 1

# Start offset for svg-term rendering (seconds). 0 = start at first event
FROM ?= 0

# Idle time limit for recording (seconds). Empty = no trimming (natural speed)
IDLE ?=
IDLE_FLAG := $(if $(strip $(IDLE)),-i $(IDLE),)

# Initial pause added to casts at conversion time (seconds)
PREROLL ?= 0

# Final height in rows for the cast
ROWS ?= 26

# Final width in columns for the cast (approx 600px)
COLS ?= 58

# Record a terminal session to $(CAST)
.PHONY: rec
rec:
	@echo "[rec] Recording to $(CAST). Press Ctrl-D or type 'exit' to finish."
	@asciinema rec $(CAST)

.PHONY: rec-% svg-% svg-min-% demos

# Named demos (must-have for README)
DEMO ?= basic pattern dir tz

# Do not delete intermediate SVGs
.SECONDARY: media/%.svg tmp/%.slow.cast tmp/%.height.cast
.PRECIOUS: media/%.svg media/%.min.svg tmp/%.slow.cast tmp/%.height.cast

# Interactive recording for named demo (basic/pattern/dir/tz)
rec-%:
	@echo "[rec] Recording to $*.cast. Run your commands, then Ctrl-D or 'exit'."
	@asciinema rec $*.cast

# Optionally scale timing before converting to SVG
tmp/%.slow.cast: %.cast tools/scale-cast.js
	@node tools/scale-cast.js --in $< --out $@ --factor $(SLOW) --preroll $(PREROLL)

# Stamp to force rebuild when SLOW/PREROLL changes
tmp/slow-$(SLOW)-$(PREROLL).stamp:
	@mkdir -p tmp
	@echo "SLOW=$(SLOW) PREROLL=$(PREROLL)" > $@

# Stamp to force rebuild when FROM changes
tmp/from-$(FROM).stamp:
	@mkdir -p tmp
	@echo "FROM=$(FROM)" > $@

# Build file: media/<name>.svg from <name>.cast
media/%.svg: %.cast tmp/slow-$(SLOW)-$(PREROLL).stamp tmp/from-$(FROM).stamp tools/svg-poster-shift.js tools/svg-add-poster.js tools/set-cast-height.js
	@mkdir -p media tmp
	@if [ "$(SLOW)" != "1" ] || [ "$(PREROLL)" != "0" ]; then \
	  in_cast="tmp/$*.slow.cast"; \
	  node tools/scale-cast.js --in $< --out "$$in_cast" --factor $(SLOW) --preroll $(PREROLL); \
	else \
	  in_cast="$<"; \
	fi; \
	height_cast="tmp/$*.height.cast"; \
	node tools/set-cast-height.js --in "$$in_cast" --out "$$height_cast" --rows $(ROWS); \
	npx -y --package svg-term-cli svg-term --in tmp/$*.height.cast --out $@ --width $(COLS) --no-cursor --from $(FROM);
	@cp $@ media/$(basename $(notdir $@)).raw # Save intermediate for debugging;
	@node tools/svg-poster-shift.js --in $@;
	@echo "[svg] Wrote $@"

# Convenience target: also allow `make svg-<name>`
svg-%: media/%.svg
	@true

# Build file: media/<name>.min.svg from media/<name>.svg
media/%.min.svg: media/%.svg
	@npx -y --package svgo svgo --multipass -i $< -o $@
	@echo "[svgo] Wrote $@"

# Convenience target: also allow `make svg-min-<name>`
svg-min-%: media/%.min.svg
	@true

# Build all base SVGs explicitly so they are kept
media-all-svg: $(patsubst %,media/%.svg,$(DEMO))
	@echo "[svg] Built all: $(DEMO)"

# Build all required demos (SVG + min SVG)
demos: media-all-svg $(patsubst %,media/%.min.svg,$(DEMO))
	@echo "[done] Built: $(DEMO)"

# --- Non-interactive (scripted) recordings ---
.PHONY: rec-auto rec-auto-basic rec-auto-pattern rec-auto-dir rec-auto-tz auto-demos

# Record all four demos automatically (no typing needed)
rec-auto: rec-auto-basic rec-auto-pattern rec-auto-dir rec-auto-tz
	@echo "[rec-auto] Recorded: $(DEMO)"

# basic: narrative script
rec-auto-basic:
	@echo "[rec-auto] basic.cast"
	@asciinema rec --overwrite $(IDLE_FLAG) -c "bash -c './make-demo.sh --force >/dev/null 2>&1 || true; bash tools/demo-basic.sh'" basic.cast

# pattern: narrative script
rec-auto-pattern:
	@echo "[rec-auto] pattern.cast"
	@asciinema rec --overwrite $(IDLE_FLAG) -c "bash -c './make-demo.sh --force >/dev/null 2>&1 || true; bash tools/demo-pattern.sh'" pattern.cast

# dir: narrative script
rec-auto-dir:
	@echo "[rec-auto] dir.cast"
	@asciinema rec --overwrite $(IDLE_FLAG) -c "bash -c './make-demo.sh --force >/dev/null 2>&1 || true; bash tools/demo-dir.sh'" dir.cast

# tz: narrative script
rec-auto-tz:
	@echo "[rec-auto] tz.cast"
	@asciinema rec --overwrite $(IDLE_FLAG) -c "bash -c './make-demo.sh --force >/dev/null 2>&1 || true; bash tools/demo-tz.sh'" tz.cast

# Build everything end-to-end automatically (record -> svg -> min.svg)
auto-demos: rec-auto demos
	@echo "[done] Auto-built demos"

.PHONY: clean
clean:
	rm -f media/*.svg media/*.min.svg media/*.raw tmp/*.cast tmp/*.stamp
	@echo "[clean] Removed generated media and tmp files"

.PHONY: lint
lint:
	@if command -v shellcheck >/dev/null 2>&1; then \
	  echo "[lint] Running shellcheck"; \
	  shellcheck -s bash ftime-list.sh || true; \
	  shellcheck -s bash make-demo.sh || true; \
	  set -e; for f in tools/*.sh; do \
	    [ -e "$$f" ] || continue; \
	    echo "[lint] $$f"; \
	    shellcheck -s bash "$$f" || true; \
	  done; \
	else \
	  echo "[lint] shellcheck not found; skipping"; \
	fi

.PHONY: test
test:
	@if command -v bats >/dev/null 2>&1; then \
	  echo "[test] Running bats"; \
	  bats tests; \
	else \
	  echo "[test] bats not found; skipping"; \
	fi

# --- Simple release workflow (bump patch, update CHANGELOG.md, create git tag) ---
.PHONY: release
release:
	@set -e; \
	if ! command -v git >/dev/null 2>&1; then echo "[release] git not found; aborting" >&2; exit 1; fi; \
	if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then echo "[release] not a git repo; aborting" >&2; exit 1; fi; \
	if ! git diff --quiet; then echo "[release] working tree not clean; commit/stash changes first" >&2; exit 1; fi; \
	cur=$$(cat VERSION); \
	new=$$(awk -F. '{printf "%d.%d.%d", $$1, $$2, $$3+1}' VERSION); \
	echo "$$new" > VERSION; \
	date=$$(date +%F); \
	tmp=$$(mktemp); \
	awk -v v="$$new" -v d="$$date" 'NR==5{print "## ["v"] - " d; print "";} {print}' CHANGELOG.md > "$$tmp"; \
	mv "$$tmp" CHANGELOG.md; \
	git add VERSION CHANGELOG.md; \
	git commit -m "chore(release): v$$new"; \
	git tag "v$$new"; \
	echo "[release] Bumped to $$new and tagged. To push: git push --follow-tags"
