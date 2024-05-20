pattern="^option go_package.*"

for file in $(grep -ERl "$pattern" --include="*.proto" protos/sift); do
  sed -E -i '' "s/${pattern}//g" $file
done

echo "Done."
