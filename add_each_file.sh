for file in *; do
  if [ -f "$file" ]; then
    git add "$file"
    git commit -m "initial commit for $file"
  fi
done