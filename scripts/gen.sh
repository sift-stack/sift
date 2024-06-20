#!/usr/bin/env bash

cleanup() {
  printf "\x1b[?25h"
  if [[ -d ./tmp ]]; then
    rm -rf "$TMP_DIR"
  fi
}

trap cleanup EXIT INT

SUPPORTED_LANGS=(go python rust)
TMP_DIR="tmp"
BUF_CONF="protos/buf.yaml"
OUTPUT_PROTOS="${TMP_DIR}/protos"
PYTHON_GEN_DIR="python/gen"
PYTHON_LIB_DIR="python/lib"
PYTHON_CLIENT_LIB="sift_py"
PYTHON_CLIENT_LIB_INTERNAL="sift_internal"

USAGE=$(cat<<EOT
Compile protocol buffers into supported target languages.

Supported languages: ${SUPPORTED_LANGS[@]}

Usage: gen [LANGS ...]

Arguments:
  <LANGS>...     The languages to generate code for; generates code for all languages if omitted

Options:
  -h, --help     Print help
EOT)

for arg in ${@}; do
  if [[ "$arg" == "-h" || "$arg" == "--help" ]]; then
    echo "$USAGE"
    exit 0
  fi
done

err_and_exit() {
  echo "$1" >&2
  exit 1
}

gen_python_modules() {
  if [[ ! -d "$PYTHON_GEN_DIR" ]]; then
    err_and_exit "The '$PYTHON_GEN_DIR' directory could not be located. Failed to generate python modules."
  fi

  printf "Generating python modules... "
  for dir in $(find "$PYTHON_GEN_DIR" -type d); do
    local init_py="$dir/__init__.py"

    if [[ ! -f "$init_py" ]]; then
      touch "$init_py"
    fi
  done

  mv "$PYTHON_LIB_DIR/$PYTHON_CLIENT_LIB" "$PYTHON_GEN_DIR"
  mv "$PYTHON_LIB_DIR/$PYTHON_CLIENT_LIB_INTERNAL" "$PYTHON_GEN_DIR"
  rm -rf "$PYTHON_LIB_DIR"
  mv "$PYTHON_GEN_DIR" "$PYTHON_LIB_DIR"

  echo "ok"
}

gen_protos() {
  printf "\x1b[?25l"
  mkdir "$TMP_DIR"
  buf mod update protos
  buf export protos --output="$OUTPUT_PROTOS" --config="$BUF_CONF"

  local langs=( "${@}" )

  if (( ${#langs[@]} == 0 )); then
    langs=( "${SUPPORTED_LANGS[@]}" )
  fi

  local python_gen=false

  for lang in ${langs[@]}; do
    printf "Compiling protocol buffers for $lang... "
    buf generate "$OUTPUT_PROTOS" --template "$lang/buf.gen.yaml" --output "$lang"
    echo "ok"

    if [[ "$lang" == "python" ]]; then
      python_gen=true
    fi
  done

  if [[ "$python_gen" == true ]]; then
    gen_python_modules
  fi
}

if [[ ! -f $(which buf) ]]; then
  err_and_exit "Missing 'buf' command. Make sure it is installed and in the path."
fi

if [[ ! -d protos ]]; then
  err_and_exit "Missing 'protos' directory in root of project."
fi

if [[ ! -f "protos/buf.yaml" ]]; then
  err_and_exit "Missing 'buf.yaml' in 'protos' directory."
fi

for lang in ${SUPPORTED_LANGS[@]}; do
  if [[ ! -d "$lang" ]]; then
    err_and_exit "Missing '$lang' directory in root of project."
  fi
done

gen_protos ${@}
