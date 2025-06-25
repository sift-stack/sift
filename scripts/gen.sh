#!/usr/bin/env bash

# TODO: This is not maintainable. Each language should have their own build process now that
# libraries are being built on top of protobuf generated code.

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
  local python_gen_dir="python/gen"
  local python_lib="python/lib"
  local sift_py="python/lib/sift_py"
  local sift_grafana="python/lib/sift_grafana"

  if [[ ! -d "$python_gen_dir" ]]; then
    err_and_exit "The '$python_gen_dir' directory could not be located. Failed to generate python modules."
  fi

  printf "Generating python modules... "
  for dir in $(find "$python_gen_dir" -type d); do
    local init_py="$dir/__init__.py"

    if [[ ! -f "$init_py" ]]; then
      touch "$init_py"
    fi
  done

  # copy over generated python code
  for sub_dir in $(find $python_gen_dir -type d -maxdepth 1); do
    rm -rf $python_lib/$(basename $sub_dir) # remove old
    mv $sub_dir $python_lib # copy in new
  done

  # This is necessary to split `google` module into separate directories: one generated from the googleapis buf plugin,
  # and the other coming from the `protobuf` PyPI package that gets installed as `google`.
  echo "__path__ = __import__('pkgutil').extend_path(__path__, __name__)" >> "$python_lib/google/__init__.py"
  
  rm -rf "${python_lib}/__init__.py"

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
    if [[ "$lang" == "rust" ]]; then
      buf generate "$OUTPUT_PROTOS" --template "$lang/crates/sift_rs/buf.gen.yaml" --output "$lang/crates/sift_rs"
    else
      buf generate "$OUTPUT_PROTOS" --template "$lang/buf.gen.yaml" --output "$lang"
    fi
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
