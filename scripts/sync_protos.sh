ENV_VAR_NAME="SIFT_PROTOS_DIR"

if [[ "$SIFT_PROTOS_DIR" == "" ]]; then
  echo "Make sure that $ENV_VAR_NAME is set before re-invoking this script."
  exit 1
fi

if [[ ! -d "$SIFT_PROTOS_DIR" ]]; then
  echo "$ENV_VAR_NAME is set but it does not point to an existing directory."
  exit 1
fi

if [[ ! -d "protos" ]]; then
  echo "Could not find 'protos' directory. Ensure that this script is being run from the project's root directory."
  exit 1
fi

DST="protos/sift"
printf "Syncing protos... "
rm -rf "$DST"
cp -R "$SIFT_PROTOS_DIR" "$DST"
echo "done."
