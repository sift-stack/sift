# remove go_package option from all proto files in the sift directory
pattern="^option go_package.*"
for file in $(grep -ERl "$pattern" --include="*.proto" protos/sift); do
  sed -E -i '' "s/${pattern}//g" $file
done

# remove unstable messages and fields from all proto files in the sift directory
for file in $(find protos/sift -name "*.proto"); do
  if [[ $file =~ "unstable.proto" ]]; then
    rm "$file"
    continue
  fi

  # Remove messages marked as unstable
  awk '/^message/{p=$0;next} /option.*unstable_message.*true/{printf "/%s/,/^}/d\n", p}' "$file" | sed -i '' -f - "$file"
  
  # Remove fields marked as unstable
  sed -i '' -e '/.*\[.*sift\.options\.v1\.unstable_field.*\].*/d' "$file"

  # Remove import of unstable proto
  sed -i '' -e '/^import.*unstable.proto.*/d' "$file"
done

echo "Done."
