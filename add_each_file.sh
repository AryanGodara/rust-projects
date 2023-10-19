function git_add_commit {
  for file in *; do
    if [ -f "$file" ]; then
      git add "$file"
      git commit -m "initial commit for $file"
    elif [ -d "$file" ]; then
      cd "$file"
      git_add_commit
      cd ..
    fi
  done
}

git_add_commit