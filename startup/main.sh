#!/bin/bash

cat ./tamagokill.ascii
cat ./startup.ascii

cmd_exists () {
  if command -v $1 &> /dev/null
  then
    return 0
  else
    return 1
  fi
}

disp_title() {
  echo ""
  echo "+++"
  echo "#" $1
  echo "+++"
  echo ""
}

step_requirements() {
  
  error_msgs=()
  links_msgs=()

  if cmd_exists "node" && cmd_exists "npm" ;
  then
    node_version=$(node -v)
    npm_version=$(npm -v)
  else
    node_version=" n/a"
    npm_version=" n/a"
    error_msgs+=("Node.js & npm are mandatory to build & run the Web part." "They are is also needed to run some helpful scripts to manipulate the project as a whole.")
    links_msgs+=("Node.js & npm: https://nodejs.org/en")
  fi

  if cmd_exists "cargo" ;
  then
    cargo_version=$(cargo -V)
  else
    cargo_version=" n/a"
    error_msgs+=("Cargo is mandatory to build & run the API part. Rust relies on it.")
    links_msgs+=("Cargo: https://doc.rust-lang.org/cargo/getting-started/installation.html")
  fi

  disp_title "Requirements check";

  echo "Node version:    " ${node_version}
  echo "Npm version:     " ${npm_version}
  echo "Cargo version:   " ${cargo_version}

  if ! [ ${#error_msgs[@]} -eq 0 ];
  then
    echo ""
    echo "--- Problems ---"
    printf '> %s\n' "${error_msgs[@]}"
  fi

  if ! [ ${#links_msgs[@]} -eq 0 ];
  then
    echo ""
    echo "--- Downloads ---"
    printf '%s\n' "${links_msgs[@]}"
  fi

}

step_setup_hooks() {
  disp_title "Setup hooks"
  cp -r ./git-hooks/* ../.git/hooks/
}

step_requirements ;
step_setup_hooks ;