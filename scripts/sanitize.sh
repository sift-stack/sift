#!/usr/bin/env zsh
# remove go_package option from all proto files in the sift directory
pattern="^option go_package.*"
for file in $(grep -ERl "$pattern" --include="*.proto" protos/sift); do
  sed -E -i.bak "s/${pattern}//g" "$file" && rm -f "$file.bak"
done

# remove unstable messages and fields from all proto files in the sift directory
for file in $(find protos/sift -name "*.proto"); do
  if [[ $file =~ "unstable.proto" ]]; then
    rm "$file"
    continue
  fi

  # Remove entire files marked with unstable_file option
  if grep -q "option.*sift\.options\.v1\.unstable_file.*=.*true" "$file"; then
    rm "$file"
    continue
  fi

  # Remove messages marked as unstable
  awk '/^message/{p=$0;next} /option.*unstable_message.*true/{printf "/%s/,/^}/d\n", p}' "$file" | sed -i.bak -f - "$file" && rm -f "$file.bak"

  # Remove fields marked as unstable
  sed -i.bak -e '/.*\[.*sift\.options\.v1\.unstable_field.*\].*/d' "$file" && rm -f "$file.bak"

  # Remove import of unstable proto
  sed -i.bak -e '/^import.*unstable.proto.*/d' "$file" && rm -f "$file.bak"
done

echo "Done."
